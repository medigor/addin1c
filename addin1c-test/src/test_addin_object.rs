use std::{error::Error, ffi::c_void, ptr::{null, null_mut}};

use crate::{
    ffi::{
        CallAsFunc, FindMethod, FindProp, GetNParams, GetPropVal, RegisterExtensionAs,
        SetMemManager, SetPropVal,
    },
    test_addin_lib::{str1c_to_string, TestAddinLib},
    Variant,
};

pub struct TestAddinObject<'a> {
    ptr: *mut c_void,
    lib: &'a TestAddinLib,
}

impl<'a> TestAddinObject<'a> {
    pub(crate) fn new(lib: &'a TestAddinLib, ptr: *mut c_void) -> Self {
        Self { lib, ptr }
    }

    pub(crate) fn set_mem_manager(&self, mem: *const c_void) -> bool {
        unsafe { SetMemManager(self.ptr, mem) }
    }

    pub(crate) fn register_extension_as(&self) -> Result<String, Box<dyn Error>> {
        let mut class_name = null_mut::<u16>();
        if !unsafe { RegisterExtensionAs(self.ptr, &mut class_name) } {
            return Err("Не удалось получить имя класса:".into());
        }

        let extenion_name = str1c_to_string(class_name);
        self.lib.mem.free_str(&mut class_name);
        Ok(extenion_name)
    }

    pub fn set_property(&self, name: &[u16], val: &Variant) -> Result<(), Box<dyn Error>> {
        let num = unsafe { FindProp(self.ptr, name.as_ptr()) };
        if num < 0 {
            return Err("Property not found".into());
        }
        if unsafe { SetPropVal(self.ptr, num, val.as_ptr()) } {
            Ok(())
        } else {
            Err("Failed to set property".into())
        }
    }

    pub fn get_property(&self, name: &[u16]) -> Result<Variant, Box<dyn Error>> {
        let num = unsafe { FindProp(self.ptr, name.as_ptr()) };
        if num < 0 {
            return Err("Property not found".into());
        }
        let val = Variant::new();
        if unsafe { GetPropVal(self.ptr, num, val.as_ptr()) } {
            Ok(val)
        } else {
            Err("Failed to get property".into())
        }
    }

    pub fn call_as_func<const PARAMS: usize>(
        &self,
        name: &[u16],
        params: &mut [Variant; PARAMS],
    ) -> Result<Variant, Box<dyn Error>> {
        let num = unsafe { FindMethod(self.ptr, name.as_ptr()) };
        if num < 0 {
            return Err("Method not found".into());
        }

        let nparams = unsafe { GetNParams(self.ptr, num) } as usize;
        #[allow(clippy::comparison_chain)]
        if nparams > PARAMS {
            return Err("Too many parameters".into());
        } else if nparams < PARAMS {
            return Err("Not enough parameters".into());
        }

        let result = Variant::new();

        if !unsafe {
            CallAsFunc(
                self.ptr,
                num,
                result.as_ptr(),
                if PARAMS > 0  {params[0].as_ptr()} else {null()},
                PARAMS as _,
            )
        } {
            return Err("Failed to call function".into());
        };

        Ok(result)
    }
}

impl<'a> Drop for TestAddinObject<'a> {
    fn drop(&mut self) {
        let _ = self.lib.destroy_object(&mut self.ptr);
    }
}
