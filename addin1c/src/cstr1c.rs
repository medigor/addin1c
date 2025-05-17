use std::ops::Deref;

#[derive(PartialEq)]
pub struct CStr1C([u16]);

impl CStr1C {
    /// Unsafely creates a C string for 1C wrapper from a byte slice.
    ///
    /// # SAFETY
    ///
    /// Slice must be a nul-terminated string.
    pub const unsafe fn from_bytes_unchecked(bytes: &[u16]) -> &Self {
        debug_assert!(!bytes.is_empty() && bytes[bytes.len() - 1] == 0);
        unsafe { &*(bytes as *const [u16] as *const CStr1C) }
    }
}

impl AsRef<CStr1C> for CStr1C {
    fn as_ref(&self) -> &CStr1C {
        self
    }
}

impl Deref for CStr1C {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CString1C(Vec<u16>);

impl CString1C {
    pub fn new(str: &str) -> Self {
        let mut buf = Vec::with_capacity(str.len() + 1);
        buf.extend(str.encode_utf16());
        buf.push(0);
        Self(buf)
    }
}

impl Deref for CString1C {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<CStr1C> for CString1C {
    fn as_ref(&self) -> &CStr1C {
        unsafe { CStr1C::from_bytes_unchecked(self.0.as_ref()) }
    }
}

impl From<&str> for CString1C {
    fn from(value: &str) -> Self {
        CString1C::new(value)
    }
}

/// Null terminated utf-16 static string, used for names
#[macro_export]
macro_rules! cstr1c {
    ($text:expr) => {
        const { unsafe { addin1c::CStr1C::from_bytes_unchecked(&addin1c::utf16_null!($text)) } }
    };
}
