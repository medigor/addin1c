use std::{ffi::c_void, fmt, ops::Deref};

use crate::ffi::{CreateMemoryManager, DeleteMemoryManager, FreeMemory};

pub struct MemoryManager(*const c_void);

#[derive(Debug)]
pub struct AllocError(usize);

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to allocate {} bytes", self.0)
    }
}

impl std::error::Error for AllocError {}

impl MemoryManager {
    pub fn new() -> Self {
        let ptr = unsafe { CreateMemoryManager() };
        Self(ptr)
    }

    pub fn free_str(&self, ptr: *mut *mut u16) {
        unsafe {
            FreeMemory(self.0, ptr as _);
        }
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
        unsafe { DeleteMemoryManager(self.0) };
    }
}

impl Deref for MemoryManager {
    type Target = *const c_void;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
