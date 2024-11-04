use std::{
    collections::HashMap,
    error::Error,
    ffi::{c_long, c_void, OsStr},
    ptr::{self},
    slice,
};

use libloading::Library;

use crate::{memory_manager::MemoryManager, test_addin_object::TestAddinObject};

pub struct TestAddinLib {
    lib: Library,
    classes: HashMap<String, Vec<u16>>,
    pub(crate) mem: MemoryManager,
}

pub(crate) fn str1c_to_string(classes: *const u16) -> String {
    let mut ptr = classes;
    unsafe {
        let len = loop {
            if *ptr == 0 {
                break ptr.offset_from(classes);
            }
            ptr = ptr.add(1);
        };

        let slice = slice::from_raw_parts(classes, len as _);
        String::from_utf16_lossy(slice)
    }
}

impl TestAddinLib {
    pub fn new<P>(file_name: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<OsStr>,
    {
        unsafe {
            let lib = { Library::new(file_name)? };
            let mem = MemoryManager::new();

            let get_class_names: libloading::Symbol<unsafe extern "C" fn() -> *const u16> =
                { lib.get(b"GetClassNames")? };

            let class_names = { get_class_names() };

            let class_names = str1c_to_string(class_names);

            let this = Self {
                lib,
                classes: HashMap::<String, Vec<u16>>::new(),
                mem,
            };
            let mut classes = HashMap::<String, Vec<u16>>::new();

            for name in class_names.split('|') {
                let mut class = name.encode_utf16().collect::<Vec<_>>();
                class.push(0);

                let component = this.get_class_object(&class)?;
                let object = TestAddinObject::new(&this, component);

                if !object.set_mem_manager(*this.mem) {
                    return Err(format!("Не удалось установить MemManager: {name}").into());
                }

                let extension_name = object.register_extension_as()?;

                classes.insert(extension_name, class);
            }

            Ok(TestAddinLib {
                lib: this.lib,
                classes,
                mem: this.mem,
            })
        }
    }

    fn get_class_object(&self, name: &[u16]) -> Result<*mut c_void, Box<dyn Error>> {
        let mut component = ptr::null_mut::<c_void>();
        let get_class_object: libloading::Symbol<
            unsafe extern "C" fn(name: *const u16, component: *mut *mut c_void) -> c_long,
        > = unsafe { self.lib.get(b"GetClassObject")? };
        if unsafe { get_class_object(name.as_ptr(), &mut component) } == 0 {
            return Err("Не удалось создать объект".into());
        };
        Ok(component)
    }

    pub(crate) fn destroy_object(&self, ptr: *mut *mut c_void) -> Result<(), Box<dyn Error>> {
        unsafe {
            let destroy_object: libloading::Symbol<
                unsafe extern "C" fn(component: *mut *mut c_void) -> c_long,
            > = { self.lib.get(b"DestroyObject")? };

            destroy_object(ptr);
            Ok(())
        }
    }

    pub fn new_addin(&self, name: &str) -> Result<TestAddinObject, Box<dyn Error>> {
        let class = self.classes.get(name).ok_or("Class not found")?;
        let component = self.get_class_object(class)?;
        let object = TestAddinObject::new(self, component);
        if !object.set_mem_manager(*self.mem) {
            return Err(format!("Не удалось установить MemManager: {name}").into());
        }
        Ok(object)
    }
}
