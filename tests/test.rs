use ssimd::*;

#[test]
fn test_load_add_store() {
    let mut v = [-1; 8];
    let s = i32x8::from_slice_unaligned(&v) + i32x8::splat(2);
    s.store(&mut v, 0);
    assert_eq!(v, [1; 8]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_load() {
    let _ = i16x8::from_slice_unaligned(&[-1i16; 2]);
}