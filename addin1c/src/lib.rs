mod connection;
mod cstr1c;
mod ffi;
mod macros;
mod memory_manager;
mod simple;
mod tm;
mod tvariant;
mod variant;

pub use connection::Connection;
pub use cstr1c::{CStr1C, CString1C};
pub use ffi::{create_component, destroy_component, Addin as RawAddin};
pub use simple::{Addin as SimpleAddin, AddinResult, MethodInfo, Methods, PropInfo};
pub use tm::Tm;
pub use variant::{IncompatibleTypeError, ParamValue, Variant};

pub use utf16_lit::utf16;
pub use utf16_lit::utf16_null;

#[repr(C)]
#[derive(Debug)]
pub enum AttachType {
    NotIsolated = 1,
    Isolated,
    Any,
}
