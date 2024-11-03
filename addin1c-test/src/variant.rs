use core::slice;
use std::{ffi::c_void, fmt::Debug};

use crate::{ffi::*, tm::Tm};

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

impl From<u16> for VariantType {
    fn from(value: u16) -> Self {
        match value {
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
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct DataStr {
    pub ptr: *mut u16,
    pub len: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct DataBlob {
    pub ptr: *mut u8,
    pub len: u32,
}

#[repr(C)]
union VariantValue {
    pub bool: bool,
    pub i32: i32,
    pub f64: f64,
    pub tm: Tm,
    pub data_str: DataStr,
    pub data_blob: DataBlob,
}

#[repr(C)]
pub struct Variant {
    value: VariantValue,
    elements: u32,
    vt: u16,
}

impl Debug for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_type = self.get_type();
        let mut debug_struct = f.debug_struct("Variant");
        debug_struct.field("vt", &variant_type);

        let value: &dyn std::fmt::Debug = match variant_type {
            VariantType::Empty => &"",
            VariantType::I4 => unsafe { &self.value.i32 },
            VariantType::R8 => unsafe { &self.value.f64 },
            VariantType::TM => unsafe { &self.value.tm },
            VariantType::Bool => unsafe { &self.value.bool },
            VariantType::Pwstr => unsafe { &self.value.data_str },
            VariantType::Blob => unsafe { &self.value.data_blob },
            VariantType::Undefined => &"",
        };
        debug_struct.field("value", value).finish()
    }
}

impl Default for Variant {
    fn default() -> Self {
        Self {
            value: VariantValue {
                tm: Default::default(),
            },
            elements: 0,
            vt: VariantType::Empty as _,
        }
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
        let vt = unsafe { GetTypeVariant(self.as_ptr()) };
        vt.into()
    }

    pub fn get_empty(&self) -> Option<()> {
        if self.get_type() == VariantType::Empty {
            Some(())
        } else {
            None
        }
    }

    pub fn set_empty(&mut self) {
        unsafe { SetEmptyVariant(self.as_ptr()) };
    }

    pub fn get_bool(&self) -> Option<bool> {
        if self.get_type() == VariantType::Bool {
            Some(unsafe { GetValVariantBool(self.as_ptr()) })
        } else {
            None
        }
    }

    pub fn set_bool(&mut self, val: bool) {
        unsafe { SetValVariantBool(self.as_ptr(), val) };
    }

    pub fn create_bool(val: bool) -> Variant {
        let mut variant = Variant::new();
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

    pub fn set_i32(&mut self, val: i32) {
        unsafe { SetValVariantI4(self.as_ptr(), val) };
    }

    pub fn create_i32(val: i32) -> Variant {
        let mut variant = Variant::new();
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

    pub fn set_f64(&mut self, val: f64) {
        unsafe { SetValVariantR8(self.as_ptr(), val) };
    }

    pub fn create_f64(val: f64) -> Variant {
        let mut variant = Variant::new();
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

    pub fn set_str(&mut self, val: &[u16]) {
        unsafe { SetValVariantString(self.as_ptr(), val.as_ptr(), val.len() as u32) };
    }

    pub fn create_str(val: &[u16]) -> Variant {
        let mut variant = Variant::new();
        variant.set_str(val);
        variant
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe { SetEmptyVariant(self.as_ptr()) };
    }
}
