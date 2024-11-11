#[repr(C)]
struct ConnectionVTable {
    dtor: usize,
    #[cfg(target_family = "unix")]
    dtor2: usize,
}

#[repr(C)]
pub struct Connection {
    vptr1: &'static ConnectionVTable,
}
