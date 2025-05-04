use std::ffi::{c_long, c_ushort};

use crate::{tvariant::TVariant, CStr1C};

#[repr(C)]
struct ConnectionVTable {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
    add_error:
        unsafe extern "system" fn(&Connection, c_ushort, *const u16, *const u16, c_long) -> bool,
    read: unsafe extern "system" fn(
        &Connection,
        *mut u16,
        &mut TVariant,
        c_long,
        *mut *mut u16,
    ) -> bool,
    write: unsafe extern "system" fn(&Connection, *mut u16, &mut TVariant) -> bool,
    register_profile_as: unsafe extern "system" fn(&Connection, *mut u16) -> bool,
    set_event_buffer_depth: unsafe extern "system" fn(&Connection, c_long) -> bool,
    get_event_buffer_depth: unsafe extern "system" fn(&Connection) -> c_long,
    external_event:
        unsafe extern "system" fn(&Connection, *const u16, *const u16, *const u16) -> bool,
    clean_event_buffer: unsafe extern "system" fn(&Connection),
    set_status_line: unsafe extern "system" fn(&Connection, *mut u16) -> bool,
    reset_status_line: unsafe extern "system" fn(&Connection),
}

#[repr(C)]
pub struct Connection {
    vptr1: &'static ConnectionVTable,
}

impl Connection {
    pub fn external_event(
        &self,
        source: impl AsRef<CStr1C>,
        message: impl AsRef<CStr1C>,
        data: impl AsRef<CStr1C>,
    ) -> bool {
        unsafe {
            (self.vptr1.external_event)(
                self,
                source.as_ref().as_ptr(),
                message.as_ref().as_ptr(),
                data.as_ref().as_ptr(),
            )
        }
    }

    pub fn set_event_buffer_depth(&self, depth: c_long) -> bool {
        unsafe { (self.vptr1.set_event_buffer_depth)(self, depth) }
    }

    pub fn get_event_buffer_depth(&self) -> c_long {
        unsafe { (self.vptr1.get_event_buffer_depth)(self) }
    }

    pub fn clean_event_buffer(&self) {
        unsafe { (self.vptr1.clean_event_buffer)(self) }
    }
}
