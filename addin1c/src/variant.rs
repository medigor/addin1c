use std::{
    fmt,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use smallvec::SmallVec;

use crate::{
    memory_manager::{AllocError, MemoryManager},
    tvariant::{TVariant, VariantType},
    Tm,
};

pub struct Variant<'a> {
    pub(crate) mem: &'a MemoryManager,
    pub(crate) variant: &'a mut TVariant,
}

#[allow(dead_code)]
impl<'a> Variant<'a> {
    pub fn get(&self) -> ParamValue {
        ParamValue::from(self.variant as &_)
    }

    fn free_memory(&mut self) {
        match self.variant.vt {
            VariantType::Pwstr => unsafe {
                self.mem.free_str(&mut self.variant.value.data_str.ptr)
            },
            VariantType::Blob => unsafe {
                self.mem.free_blob(&mut self.variant.value.data_blob.ptr)
            },
            _ => (),
        }
    }
    pub fn get_empty(&self) -> Result<(), IncompatibleTypeError> {
        let ParamValue::Empty = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        Ok(())
    }
    pub fn set_empty(&mut self) {
        self.free_memory();
        self.variant.vt = VariantType::Empty;
    }

    pub fn get_i32(&self) -> Result<i32, IncompatibleTypeError> {
        let ParamValue::I32(value) = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        Ok(value)
    }
    pub fn set_i32(&mut self, val: i32) {
        self.free_memory();
        self.variant.vt = VariantType::I4;
        self.variant.value.i32 = val;
    }

    pub fn get_bool(&self) -> Result<bool, IncompatibleTypeError> {
        let ParamValue::Bool(value) = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        Ok(value)
    }
    pub fn set_bool(&mut self, val: bool) {
        self.free_memory();
        self.variant.vt = VariantType::Bool;
        self.variant.value.bool = val;
    }

    pub fn get_f64(&self) -> Result<f64, IncompatibleTypeError> {
        let ParamValue::F64(value) = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        Ok(value)
    }
    pub fn set_f64(&mut self, val: f64) {
        self.free_memory();
        self.variant.vt = VariantType::R8;
        self.variant.value.f64 = val;
    }

    pub fn get_date(&self) -> Result<Tm, IncompatibleTypeError> {
        let ParamValue::Date(value) = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        Ok(value)
    }
    pub fn set_date(&mut self, val: Tm) {
        self.free_memory();
        self.variant.vt = VariantType::TM;
        self.variant.value.tm = val;
    }

    pub fn get_str1c(&self) -> Result<&[u16], IncompatibleTypeError> {
        let ParamValue::Str(value) = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        Ok(value)
    }
    pub fn get_string(&self) -> Result<String, IncompatibleTypeError> {
        let ParamValue::Str(value) = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        let value = String::from_utf16(value).map_err(|_| IncompatibleTypeError {})?;
        Ok(value)
    }
    pub fn set_str1c(&mut self, val: impl IntoStr1C) -> Result<(), AllocError> {
        val.write(self)
    }

    pub fn get_blob(&self) -> Result<&[u8], IncompatibleTypeError> {
        let ParamValue::Blob(value) = self.get() else {
            return Err(IncompatibleTypeError {});
        };
        Ok(value)
    }
    pub fn set_blob(&mut self, val: &[u8]) -> Result<(), AllocError> {
        let ptr = self.mem.alloc_blob(val.len())?;
        self.free_memory();

        unsafe { std::ptr::copy_nonoverlapping(val.as_ptr(), ptr.as_ptr(), val.len()) };

        self.variant.vt = VariantType::Blob;
        self.variant.value.data_blob.ptr = ptr.as_ptr();
        self.variant.value.data_blob.len = val.len() as u32;
        Ok(())
    }

    pub fn alloc_str(&mut self, len: usize) -> Option<&'a mut [u16]> {
        let Ok(ptr) = self.mem.alloc_str(len) else {
            return None;
        };

        self.free_memory();

        self.variant.vt = VariantType::Pwstr;
        self.variant.value.data_str.ptr = ptr.as_ptr();
        self.variant.value.data_str.len = len as u32;

        Some(unsafe { from_raw_parts_mut(ptr.as_ptr(), len) })
    }

    pub fn alloc_blob(&mut self, len: usize) -> Option<&'a mut [u8]> {
        let Ok(ptr) = self.mem.alloc_blob(len) else {
            return None;
        };
        self.free_memory();

        self.variant.vt = VariantType::Blob;
        self.variant.value.data_blob.ptr = ptr.as_ptr();
        self.variant.value.data_blob.len = len as u32;

        Some(unsafe { from_raw_parts_mut(ptr.as_ptr(), len) })
    }
}

pub trait IntoStr1C {
    fn write(&self, variant: &mut Variant) -> Result<(), AllocError>;
}

impl IntoStr1C for &[u16] {
    fn write(&self, variant: &mut Variant) -> Result<(), AllocError> {
        let ptr = variant.mem.alloc_str(self.len())?;
        variant.free_memory();

        unsafe { std::ptr::copy_nonoverlapping(self.as_ptr(), ptr.as_ptr(), self.len()) };

        variant.variant.vt = VariantType::Pwstr;
        variant.variant.value.data_str.ptr = ptr.as_ptr();
        variant.variant.value.data_str.len = self.len() as u32;
        Ok(())
    }
}

impl IntoStr1C for &str {
    fn write(&self, variant: &mut Variant) -> Result<(), AllocError> {
        let mut buf = SmallVec::<[u16; 128]>::new();
        buf.extend(self.encode_utf16());
        IntoStr1C::write(&buf.as_slice(), variant)
    }
}

impl IntoStr1C for String {
    fn write(&self, variant: &mut Variant) -> Result<(), AllocError> {
        self.as_str().write(variant)
    }
}

pub enum ParamValue<'a> {
    Empty,
    Bool(bool),
    I32(i32),
    F64(f64),
    Date(Tm),
    Str(&'a [u16]),
    Blob(&'a [u8]),
}

impl<'a> From<&'a TVariant> for ParamValue<'a> {
    fn from(param: &'a TVariant) -> ParamValue<'a> {
        unsafe {
            match param.vt {
                VariantType::Empty => Self::Empty,
                VariantType::Bool => Self::Bool(param.value.bool),
                VariantType::I4 => Self::I32(param.value.i32),
                VariantType::R8 => Self::F64(param.value.f64),
                VariantType::TM => Self::Date(param.value.tm),
                VariantType::Pwstr => Self::Str(from_raw_parts(
                    param.value.data_str.ptr,
                    param.value.data_str.len as usize,
                )),
                VariantType::Blob => Self::Blob(from_raw_parts(
                    param.value.data_blob.ptr,
                    param.value.data_blob.len as usize,
                )),
                _ => Self::Empty,
            }
        }
    }
}

#[derive(Debug)]
pub struct IncompatibleTypeError {}

impl fmt::Display for IncompatibleTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incompatible type")
    }
}

impl std::error::Error for IncompatibleTypeError {}
