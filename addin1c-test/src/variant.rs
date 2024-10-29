use core::slice;
use std::ffi::c_void;

use crate::ffi::*;

#[repr(u16)]
#[derive(PartialEq, Eq, Debug)]
pub enum VariantType {
    Empty = 0,
    I4 = 3,
    R8 = 5,
    TM = 7,
    Bool = 11,
    Pwstr = 22,
    Blob = 23,
    Undefined = 0xFFFF,
}

#[cfg(target_family = "windows")]
const VARIANT_SIZE: usize = 48;

#[cfg(target_family = "unix")]
const VARIANT_SIZE: usize = 64;

pub struct Variant {
    _data: [u8; VARIANT_SIZE],
}

impl Default for Variant {
    fn default() -> Self {
        Self { _data: [0; VARIANT_SIZE] }
    }
}

impl Variant {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn as_ptr(&self) -> *const c_void {
        self as *const Variant as *const c_void
    }

    pub fn get_type(&self) -> VariantType {
        match unsafe { GetTypeVariant(self.as_ptr()) } {
            0 => VariantType::Empty,
            3 => VariantType::I4,
            5 => VariantType::R8,
            7 => VariantType::TM,
            11 => VariantType::Bool,
            22 => VariantType::Pwstr,
            23 => VariantType::Blob,
            _ => VariantType::Undefined,
        }
    }

    pub fn get_empty(&self) -> Option<()> {
        if self.get_type() == VariantType::Empty {
            Some(())
        } else {
            None
        }
    }

    pub fn set_empty(&self) {
        unsafe { SetEmptyVariant(self.as_ptr()) };
    }

    pub fn get_bool(&self) -> Option<bool> {
        if self.get_type() == VariantType::Bool {
            Some(unsafe { GetValVariantBool(self.as_ptr()) })
        } else {
            None
        }
    }

    pub fn set_bool(&self, val: bool) {
        unsafe { SetValVariantBool(self.as_ptr(), val) };
    }

    pub fn create_bool(val: bool) -> Variant {
        let variant = Variant::new();
        variant.set_bool(val);
        variant
    }

    pub fn get_i32(&self) -> Option<i32> {
        if self.get_type() == VariantType::I4 {
            Some(unsafe { GetValVariantI4(self.as_ptr()) })
        } else {
            None
        }
    }

    pub fn set_i32(&self, val: i32) {
        unsafe { SetValVariantI4(self.as_ptr(), val) };
    }

    pub fn create_i32(val: i32) -> Variant {
        let variant = Variant::new();
        variant.set_i32(val);
        variant
    }

    pub fn get_f64(&self) -> Option<f64> {
        if self.get_type() == VariantType::R8 {
            Some(unsafe { GetValVariantR8(self.as_ptr()) })
        } else {
            None
        }
    }

    pub fn set_f64(&self, val: f64) {
        unsafe { SetValVariantR8(self.as_ptr(), val) };
    }

    pub fn create_f64(val: f64) -> Variant {
        let variant = Variant::new();
        variant.set_f64(val);
        variant
    }

    pub fn get_str(&self) -> Option<&[u16]> {
        if self.get_type() == VariantType::Pwstr {
            let len = unsafe { GetLenVariantString(self.as_ptr()) } as usize;
            let val = unsafe { GetValVariantString(self.as_ptr()) };
            let data = unsafe { slice::from_raw_parts(val, len) };
            Some(data)
        } else {
            None
        }
    }

    pub fn set_str(&self, val: &[u16]) {
        unsafe { SetValVariantString(self.as_ptr(), val.as_ptr(), val.len() as u32) };
    }

    pub fn create_str(val: &[u16]) -> Variant {
        let variant = Variant::new();
        variant.set_str(val);
        variant
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe { SetEmptyVariant(self.as_ptr()) };
    }
}
