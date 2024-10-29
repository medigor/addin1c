mod ffi;
mod memory_manager;
mod test_addin_lib;
mod test_addin_object;
mod variant;

#[cfg(test)]
mod tests;

pub use test_addin_lib::TestAddinLib;
pub use variant::Variant;

pub use utf16_lit::utf16;
pub use utf16_lit::utf16_null;

/// Null terminated utf-16 static string, used for names
#[macro_export]
macro_rules! name {
    ($text:expr) => {
        &addin1c_test::utf16_null!($text)
    };
}

/// Non null utf-16 static string, used for 1c-strings
#[macro_export]
macro_rules! str1c {
    ($text:expr) => {
        &addin1c_test::utf16!($text)
    };
}
