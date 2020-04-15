use ssimd::*;

#[test]
fn test_load_add_store_int() {
    let mut v = [-1; 8];
    let s = i32x8::from_slice_unaligned(&v) + i32x8::splat(2);
    s.write_to_slice_aligned(&mut v);
    assert_eq!(v, [1; 8]);
}

#[test]
fn test_replace_extract() {
    assert_eq!(i32x4::new(0, 1, 2, 3).replace(1usize, 7).extract(1usize), 7);
}

#[test]
fn test_shr() {
    let mut v = [0; 4];
    (i32x4::new(0, 1, 2, 3) << 1u32).write_to_slice_aligned(&mut v);
    assert_eq!(v, [0, 2, 4, 6]);
}

#[test]
fn test_shl() {
    let mut v = [0u8; 8];
    (u8x8::new(0, 1, 2, 3, 4, 5, 6, 7) >> 1u32).write_to_slice_aligned(&mut v);
    assert_eq!(v, [0, 0, 1, 1, 2, 2, 3, 3]);
}

#[test]
fn test_shift() {
    let mut v = [0; 4];
    (i32x4::new(1, 2, 3, 4) >> 2).store(&mut v, 0);
    assert_eq!(v, [0, 0, 0, 1]);
}

#[test]
fn test_conversion() {
    let mut v = [0; 4];
    i32x4::from(i16x4::new(1, 2, 3, 4)).store(&mut v, 0);
    assert_eq!(v, [1, 2, 3, 4]);
}

#[test]
fn test_cast() {
    let mut v = [0u8; 4];
    u8x4::from_cast(i32x4::new(1, 2, 3, 4)).store(&mut v, 0);
    assert_eq!(v, [1, 2, 3, 4]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_load() {
    let _ = i16x8::from_slice_unaligned(&[-1i16; 2]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_store() {
    let mut v = [0i16; 3];
    i16x8::splat(0).write_to_slice_aligned(&mut v);
}

#[test]
fn test_i16x4() {
    let v: [i16; 4] = [-1, 3, 1, 2];
    assert_eq!(i16x4::from_slice_unaligned(&v).gt(i16x4::splat(0)).all(), false);
    assert_eq!(i16x4::from_slice_unaligned(&v).lt(i16x4::splat(4)).all(), true);
}