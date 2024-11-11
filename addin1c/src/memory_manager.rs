use std::{
    ffi::{c_ulong, c_void},
    fmt,
    ptr::{self, NonNull},
};

#[repr(C)]
struct MemoryManagerVTable {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    alloc_memory: unsafe extern "system" fn(&MemoryManager, *mut *mut c_void, c_ulong) -> bool,
    free_memory: unsafe extern "system" fn(&MemoryManager, *mut *mut c_void),
}

#[repr(C)]
pub(crate) struct MemoryManager {
    vptr: &'static MemoryManagerVTable,
}

impl MemoryManager {
    pub fn alloc_blob(&self, size: usize) -> Result<NonNull<u8>, AllocError> {
        let mut ptr = ptr::null_mut::<c_void>();
        unsafe {
            if (self.vptr.alloc_memory)(self, &mut ptr, size as c_ulong) {
                NonNull::new(ptr as *mut u8)
            } else {
                None
            }
        }
        .ok_or(AllocError(size))
    }

    pub fn alloc_str(&self, size: usize) -> Result<NonNull<u16>, AllocError> {
        let mut ptr = ptr::null_mut::<c_void>();
        unsafe {
            if (self.vptr.alloc_memory)(self, &mut ptr, size as c_ulong * 2) {
                NonNull::new(ptr as *mut u16)
            } else {
                None
            }
        }
        .ok_or(AllocError(size * 2))
    }

    pub fn free_str(&self, ptr: *mut *mut u16) {
        unsafe {
            (self.vptr.free_memory)(self, ptr as _);
        }
    }

    pub fn free_blob(&self, ptr: *mut *mut u8) {
        unsafe {
            (self.vptr.free_memory)(self, ptr as _);
        }
    }
}

#[derive(Debug)]
pub struct AllocError(usize);

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to allocate {} bytes", self.0)
    }
}

impl std::error::Error for AllocError {}
