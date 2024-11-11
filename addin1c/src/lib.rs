mod ffi;
mod macros;
mod simple;
mod tm;

pub use ffi::{
    create_component, destroy_component, Addin as RawAddin, AttachType, Connection,
    IncompatibleTypeError, ParamValue, Variant,
};
pub use simple::{Addin as SimpleAddin, MethodInfo, Methods, PropInfo, AddinResult};
pub use tm::Tm;

pub use utf16_lit::utf16;
pub use utf16_lit::utf16_null;
