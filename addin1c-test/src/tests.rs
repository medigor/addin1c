use crate::Variant;

extern "C" {
    pub fn SizeOfVariant() -> std::ffi::c_ulong;
}

#[test]
fn size_of_variant_correct() {
    assert_eq!(size_of::<Variant>(), unsafe { SizeOfVariant() } as usize);
}
