use std::ffi::{c_long, c_void};

// component_base
extern "C" {

    pub fn SetMemManager(component: *mut c_void, memory_manager: *const c_void) -> bool;

    pub fn RegisterExtensionAs(component: *mut c_void, name: *mut *mut u16) -> bool;

    pub fn FindProp(component: *mut c_void, name: *const u16) -> c_long;

    pub fn SetPropVal(component: *mut c_void, num: c_long, val: *const c_void) -> bool;

    pub fn GetPropVal(component: *mut c_void, num: c_long, val: *const c_void) -> bool;

    pub fn FindMethod(component: *mut c_void, name: *const u16) -> c_long;

    pub fn GetNParams(component: *mut c_void, num: c_long) -> c_long;

    pub fn CallAsFunc(
        component: *mut c_void,
        num: c_long,
        ret_value: *const c_void,
        params: *const c_void,
        len: c_long,
    ) -> bool;

}

// memory_manager
extern "C" {
    pub fn CreateMemoryManager() -> *const c_void;

    pub fn DeleteMemoryManager(mem: *const c_void);

    pub fn FreeMemory(mem: *const c_void, ptr: *mut *mut c_void);
}

// variant
extern "C" {

    pub fn GetTypeVariant(ptr: *const c_void) -> u16;

    pub fn SetEmptyVariant(ptr: *const c_void);

    pub fn GetValVariantBool(ptr: *const c_void) -> bool;

    pub fn SetValVariantBool(ptr: *const c_void, val: bool);

    pub fn GetValVariantI4(ptr: *const c_void) -> i32;

    pub fn SetValVariantI4(ptr: *const c_void, val: i32);

    pub fn GetValVariantR8(ptr: *const c_void) -> f64;

    pub fn SetValVariantR8(ptr: *const c_void, val: f64);

    pub fn GetLenVariantString(ptr: *const c_void) -> u32;

    pub fn GetValVariantString(ptr: *const c_void) -> *const u16;

    pub fn SetValVariantString(ptr: *const c_void, val: *const u16, len: u32);
}
