mod ffi;
mod macros;
mod simple;

pub use ffi::{
    create_component, destroy_component, Addin as RawAddin, AttachType, Connection,
    IncompatibleTypeError, ParamValue, Tm, Variant,
};
pub use simple::{Addin as SimpleAddin, MethodInfo, Methods, PropInfo, AddinResult};

pub use utf16_lit::utf16;
pub use utf16_lit::utf16_null;
