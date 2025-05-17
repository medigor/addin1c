use std::{
    ffi::{c_long, c_void},
    ptr::{self},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use smallvec::SmallVec;

use crate::{
    memory_manager::MemoryManager, tvariant::TVariant, variant::Variant, CStr1C, Connection,
};

#[allow(unused_variables)]
pub trait Addin {
    fn init(&mut self, interface: &'static Connection) -> bool {
        true
    }

    /// default 2000, don't use version 1000, because static objects are created
    fn get_info(&mut self) -> u16 {
        2000
    }
    fn done(&mut self) {}
    fn register_extension_as(&mut self) -> &CStr1C;
    fn get_n_props(&mut self) -> usize {
        0
    }
    fn find_prop(&mut self, name: &CStr1C) -> Option<usize> {
        None
    }
    fn get_prop_name(&mut self, num: usize, alias: usize) -> Option<&'static CStr1C> {
        None
    }
    fn get_prop_val(&mut self, num: usize, val: &mut Variant) -> bool {
        false
    }
    fn set_prop_val(&mut self, num: usize, val: &Variant) -> bool {
        false
    }
    fn is_prop_readable(&mut self, num: usize) -> bool {
        false
    }
    fn is_prop_writable(&mut self, num: usize) -> bool {
        false
    }
    fn get_n_methods(&mut self) -> usize {
        0
    }
    fn find_method(&mut self, name: &CStr1C) -> Option<usize> {
        None
    }
    fn get_method_name(&mut self, num: usize, alias: usize) -> Option<&'static CStr1C> {
        None
    }
    fn get_n_params(&mut self, num: usize) -> usize {
        0
    }
    fn get_param_def_value(&mut self, method_num: usize, param_num: usize, value: Variant) -> bool {
        true
    }
    fn has_ret_val(&mut self, method_num: usize) -> bool {
        false
    }
    fn call_as_proc(&mut self, method_num: usize, params: &mut [Variant]) -> bool {
        false
    }
    fn call_as_func(
        &mut self,
        method_num: usize,
        params: &mut [Variant],
        val: &mut Variant,
    ) -> bool {
        false
    }
    fn set_locale(&mut self, loc: &[u16]) {}
    fn set_user_interface_language_code(&mut self, lang: &[u16]) {}
}

#[repr(C)]
struct This<const OFFSET: usize, T: Addin> {
    ptr: *mut Component<T>,
}

impl<const OFFSET: usize, T: Addin> This<OFFSET, T> {
    unsafe fn get_component(&mut self) -> &mut Component<T> {
        let new_ptr = (self as *mut This<OFFSET, T> as *mut c_void)
            .sub(OFFSET * std::mem::size_of::<usize>());
        &mut *(new_ptr as *mut Component<T>)
    }
}

#[repr(C)]
struct InitDoneBaseVTable<const OFFSET: usize, T: Addin> {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    init: unsafe extern "system" fn(&mut This<OFFSET, T>, &'static Connection) -> bool,
    set_mem_manager:
        unsafe extern "system" fn(&mut This<OFFSET, T>, &'static MemoryManager) -> bool,
    get_info: unsafe extern "system" fn(&mut This<OFFSET, T>) -> c_long,
    done: unsafe extern "system" fn(&mut This<OFFSET, T>),
}

unsafe extern "system" fn init<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    interface: &'static Connection,
) -> bool {
    let component = this.get_component();
    component.addin.init(interface)
}

unsafe extern "system" fn set_mem_manager<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    mem: &'static MemoryManager,
) -> bool {
    let component = this.get_component();
    component.memory = Some(mem);
    true
}

unsafe extern "system" fn get_info<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
) -> c_long {
    let component = this.get_component();
    component.addin.get_info() as c_long
}

unsafe extern "system" fn done<const OFFSET: usize, T: Addin>(this: &mut This<OFFSET, T>) {
    let component = this.get_component();
    component.addin.done()
}

#[repr(C)]
struct LanguageExtenderBaseVTable<const OFFSET: usize, T: Addin> {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    register_extension_as: unsafe extern "system" fn(&mut This<OFFSET, T>, *mut *mut u16) -> bool,
    get_n_props: unsafe extern "system" fn(&mut This<OFFSET, T>) -> c_long,
    find_prop: unsafe extern "system" fn(&mut This<OFFSET, T>, *const u16) -> c_long,
    get_prop_name: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long, c_long) -> *const u16,
    get_prop_val: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long, &mut TVariant) -> bool,
    set_prop_val: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long, &mut TVariant) -> bool,
    is_prop_readable: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long) -> bool,
    is_prop_writable: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long) -> bool,
    get_n_methods: unsafe extern "system" fn(&mut This<OFFSET, T>) -> c_long,
    find_method: unsafe extern "system" fn(&mut This<OFFSET, T>, *const u16) -> c_long,
    get_method_name: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long, c_long) -> *const u16,
    get_n_params: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long) -> c_long,
    get_param_def_value:
        unsafe extern "system" fn(&mut This<OFFSET, T>, c_long, c_long, &mut TVariant) -> bool,
    has_ret_val: unsafe extern "system" fn(&mut This<OFFSET, T>, c_long) -> bool,
    call_as_proc:
        unsafe extern "system" fn(&mut This<OFFSET, T>, c_long, *mut TVariant, c_long) -> bool,
    call_as_func: unsafe extern "system" fn(
        &mut This<OFFSET, T>,
        c_long,
        &mut TVariant,
        *mut TVariant,
        c_long,
    ) -> bool,
}

unsafe extern "system" fn register_extension_as<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    name: *mut *mut u16,
) -> bool {
    let component = this.get_component();
    let Some(allocator) = component.memory else {
        return false;
    };

    let extension_name = component.addin.register_extension_as();

    let Ok(ptr) = allocator.alloc_str(extension_name.len()) else {
        return false;
    };
    ptr::copy_nonoverlapping(extension_name.as_ptr(), ptr.as_ptr(), extension_name.len());
    *name = ptr.as_ptr();

    true
}

unsafe extern "system" fn get_n_props<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
) -> c_long {
    let component = this.get_component();
    component.addin.get_n_props() as c_long
}

unsafe extern "system" fn find_prop<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    name: *const u16,
) -> c_long {
    let component = this.get_component();
    let name = CStr1C::from_bytes_unchecked(get_str(name));
    match component.addin.find_prop(name) {
        Some(i) => i as c_long,
        None => -1,
    }
}

unsafe extern "system" fn get_prop_name<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    num: c_long,
    alias: c_long,
) -> *const u16 {
    let component = this.get_component();
    let Some(allocator) = component.memory else {
        return ptr::null();
    };
    let Some(prop_name) = component.addin.get_prop_name(num as usize, alias as usize) else {
        return ptr::null();
    };
    let Ok(ptr) = allocator.alloc_str(prop_name.len()) else {
        return ptr::null();
    };
    ptr::copy_nonoverlapping(prop_name.as_ptr(), ptr.as_ptr(), prop_name.len());

    ptr.as_ptr()
}

unsafe extern "system" fn get_prop_val<const OFFSET: usize, T: Addin>(
    component: &mut This<OFFSET, T>,
    num: c_long,
    val: &mut TVariant,
) -> bool {
    let component = component.get_component();
    let Some(mem) = component.memory else {
        return false;
    };

    let mut return_value = Variant { mem, variant: val };
    component
        .addin
        .get_prop_val(num as usize, &mut return_value)
}

unsafe extern "system" fn set_prop_val<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    num: c_long,
    val: &mut TVariant,
) -> bool {
    let component = this.get_component();
    let Some(mem) = component.memory else {
        return false;
    };
    let value = Variant { mem, variant: val };
    component.addin.set_prop_val(num as usize, &value)
}

unsafe extern "system" fn is_prop_readable<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    num: c_long,
) -> bool {
    let component = this.get_component();
    component.addin.is_prop_readable(num as usize)
}

unsafe extern "system" fn is_prop_writable<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    num: c_long,
) -> bool {
    let component = this.get_component();
    component.addin.is_prop_writable(num as usize)
}

unsafe extern "system" fn get_n_methods<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
) -> c_long {
    let component = this.get_component();
    component.addin.get_n_methods() as c_long
}

unsafe extern "system" fn find_method<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    name: *const u16,
) -> c_long {
    let component = this.get_component();
    let name = CStr1C::from_bytes_unchecked(get_str(name));
    match component.addin.find_method(name) {
        Some(i) => i as c_long,
        None => -1,
    }
}

unsafe extern "system" fn get_method_name<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    num: c_long,
    alias: c_long,
) -> *const u16 {
    let component = this.get_component();
    let Some(allocator) = component.memory else {
        return ptr::null();
    };
    let Some(method_name) = component
        .addin
        .get_method_name(num as usize, alias as usize)
    else {
        return ptr::null();
    };
    let Ok(ptr) = allocator.alloc_str(method_name.len()) else {
        return ptr::null();
    };

    ptr::copy_nonoverlapping(method_name.as_ptr(), ptr.as_ptr(), method_name.len());

    ptr.as_ptr()
}

unsafe extern "system" fn get_n_params<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    num: c_long,
) -> c_long {
    let component = this.get_component();
    component.addin.get_n_params(num as usize) as _
}

unsafe extern "system" fn get_param_def_value<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    method_num: c_long,
    param_num: c_long,
    val: &mut TVariant,
) -> bool {
    let component = this.get_component();
    let Some(mem) = component.memory else {
        return false;
    };

    let return_value = Variant { mem, variant: val };

    component
        .addin
        .get_param_def_value(method_num as usize, param_num as usize, return_value)
}

unsafe extern "system" fn has_ret_val<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    method_num: c_long,
) -> bool {
    let component = this.get_component();
    component.addin.has_ret_val(method_num as usize)
}

unsafe extern "system" fn call_as_proc<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    method_num: c_long,
    params: *mut TVariant,
    size_array: c_long,
) -> bool {
    let component = this.get_component();
    let Some(mem) = component.memory else {
        return false;
    };

    let size_array = size_array as usize;

    let mut param_values = SmallVec::<[Variant; 8]>::new();
    if size_array > 0 {
        for variant in from_raw_parts_mut(params, size_array) {
            param_values.push(Variant { mem, variant });
        }
    }

    component
        .addin
        .call_as_proc(method_num as usize, &mut param_values)
}

unsafe extern "system" fn call_as_func<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    method_num: c_long,
    ret_value: &mut TVariant,
    params: *mut TVariant,
    size_array: c_long,
) -> bool {
    let component = this.get_component();
    let Some(mem) = component.memory else {
        return false;
    };

    let size_array = size_array as usize;

    let mut return_value = Variant {
        mem,
        variant: ret_value,
    };

    let mut param_values = SmallVec::<[Variant; 8]>::new();
    if size_array > 0 {
        for variant in from_raw_parts_mut(params, size_array) {
            param_values.push(Variant { mem, variant });
        }
    }

    component
        .addin
        .call_as_func(method_num as usize, &mut param_values, &mut return_value)
}

#[repr(C)]
struct LocaleBaseVTable<const OFFSET: usize, T: Addin> {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    set_locale: unsafe extern "system" fn(&mut This<OFFSET, T>, *const u16),
}

unsafe extern "system" fn set_locale<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    loc: *const u16,
) {
    let component = this.get_component();
    let loc = get_str(loc);
    component.addin.set_locale(loc)
}

#[repr(C)]
struct UserLanguageBaseVTable<const OFFSET: usize, T: Addin> {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    set_user_interface_language_code: unsafe extern "system" fn(&mut This<OFFSET, T>, *const u16),
}

unsafe extern "system" fn set_user_interface_language_code<const OFFSET: usize, T: Addin>(
    this: &mut This<OFFSET, T>,
    lang: *const u16,
) {
    let component = this.get_component();
    let lang = get_str(lang);
    component.addin.set_user_interface_language_code(lang)
}

#[repr(C)]
struct Component<T: Addin> {
    vptr1: Box<InitDoneBaseVTable<0, T>>,
    vptr2: Box<LanguageExtenderBaseVTable<1, T>>,
    vptr3: Box<LocaleBaseVTable<2, T>>,
    vptr4: Box<UserLanguageBaseVTable<3, T>>,
    destroy: unsafe extern "system" fn(*mut *mut Component<T>),
    memory: Option<&'static MemoryManager>,
    addin: T,
}

unsafe extern "system" fn destroy<T: Addin>(component: *mut *mut Component<T>) {
    let comp = Box::from_raw(*component);
    drop(comp);
}

/// # Safety
///
/// Component must be non-null.
pub unsafe fn create_component<T: Addin>(component: *mut *mut c_void, addin: T) -> c_long {
    let vptr1 = Box::new(InitDoneBaseVTable {
        dtor: 0,
        #[cfg(target_family = "unix")]
        dtor2: 0,
        init,
        set_mem_manager,
        get_info,
        done,
    });

    let vptr2 = Box::new(LanguageExtenderBaseVTable {
        dtor: 0,
        #[cfg(target_family = "unix")]
        dtor2: 0,
        register_extension_as,
        get_n_props,
        find_prop,
        get_prop_name,
        get_prop_val,
        set_prop_val,
        is_prop_readable,
        is_prop_writable,
        get_n_methods,
        find_method,
        get_method_name,
        get_n_params,
        get_param_def_value,
        has_ret_val,
        call_as_proc,
        call_as_func,
    });

    let vptr3 = Box::new(LocaleBaseVTable {
        dtor: 0,
        #[cfg(target_family = "unix")]
        dtor2: 0,
        set_locale,
    });

    let vptr4 = Box::new(UserLanguageBaseVTable {
        dtor: 0,
        #[cfg(target_family = "unix")]
        dtor2: 0,
        set_user_interface_language_code,
    });

    let c = Box::new(Component {
        vptr1,
        vptr2,
        vptr3,
        vptr4,
        destroy: destroy::<T>,
        memory: None,
        addin,
    });

    *component = Box::into_raw(c) as *mut c_void;
    1
}

/// # Safety
///
/// Component must be returned from `create_component`, the function must be called once for each component.
pub unsafe fn destroy_component(component: *mut *mut c_void) -> c_long {
    #[repr(C)]
    struct ComponentWrapper {
        vptr1: usize,
        vptr2: usize,
        vptr3: usize,
        vptr4: usize,
        destroy: unsafe extern "system" fn(*mut *mut c_void),
    }

    let wrapper = *component as *mut ComponentWrapper;
    let wrapper = &mut *wrapper;
    (wrapper.destroy)(component);
    *component = ptr::null_mut();

    0
}

unsafe fn get_str<'a>(s: *const u16) -> &'a [u16] {
    unsafe fn strlen(s: *const u16) -> usize {
        let mut i = 0;
        while *s.add(i) != 0 {
            i += 1;
        }
        i + 1
    }

    let len = strlen(s);
    from_raw_parts(s, len)
}
