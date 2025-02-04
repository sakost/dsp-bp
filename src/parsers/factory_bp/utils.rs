/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

macro_rules! define_read_fn {
    ($func_name:ident, $type:ty, $size:expr) => {
        #[inline]
        pub fn $func_name(data: &[u8], offset: usize) -> ($type, usize) {
            let bytes: [u8; $size] = data[offset..offset + $size]
                .try_into()
                .expect("slice with incorrect length");
            (<$type>::from_le_bytes(bytes), offset + $size)
        }
    };
}

define_read_fn!(read_u8, u8, 1);
define_read_fn!(read_u16, u16, 2);
define_read_fn!(read_u32, u32, 4);
define_read_fn!(read_f32, f32, 4);
define_read_fn!(read_i8, i8, 1);
define_read_fn!(read_i16, i16, 2);
define_read_fn!(read_i32, i32, 4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let data = [10, 20, 30];
        assert_eq!(read_u8(&data, 1), (20, 2));
    }

    #[test]
    fn test_read_u16() {
        let data = [0x34, 0x12, 0x78, 0x56];
        let offset = 0;
        let (result, offset) = read_u16(&data, offset);
        assert_eq!(result, 0x1234);
        let (result, _) = read_u16(&data, offset);
        assert_eq!(result, 0x5678);
    }

    #[test]
    fn test_read_u32() {
        let data = [0x78, 0x56, 0x34, 0x12];
        assert_eq!(read_u32(&data, 0), (0x12345678, 4));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_read_f32() {
        let value: f32 = 3.14;
        let bytes = value.to_le_bytes();
        let (read_val, _) = read_f32(&bytes, 0);
        assert!((read_val - 3.14).abs() < 1e-6);
    }

    // todo: add tests for read_ixx functions
}
