#![feature(portable_simd)]
use std::fmt::Debug;
use std::simd::Simd;

use num::Num;

pub trait ReverseBits {
    fn reverse_bits(self) -> Self;
}

impl ReverseBits for u8 {
    #[inline(always)]
    fn reverse_bits(self) -> Self {
        #[inline(always)]
        fn rev(nibble: u8) -> u8 {
            let a = nibble & 0b1000;
            let b = nibble & 0b0100;
            let c = nibble & 0b0010;
            let d = nibble & 0b0001;
            d << 3 | c << 1 | b >> 1 | a >> 3
        }
        let top = (self & 0b11110000) >> 4;
        let bot = self & !0b11110000;
        rev(bot) << 4 | rev(top)
    }
}

impl<const N: usize> ReverseBits for [u8; N] {
    fn reverse_bits(mut self) -> Self {
        self.reverse();
        for i in 0..N {
            self[i] = self[i].reverse_bits()
        }
        self
    }
}

// macro to implement bit-reversion by converting to an array of bytes,
// reversing that and reversing each byte in itself
macro_rules! rev_through_bytes {
    ($t:ty) => {
        impl ReverseBits for $t {
            fn reverse_bits(self) -> Self {
                <$t>::from_ne_bytes(self.to_ne_bytes().reverse_bits())
            }
        }
    };
}

rev_through_bytes!(u16);
rev_through_bytes!(u32);
rev_through_bytes!(u64);
rev_through_bytes!(u128);
rev_through_bytes!(i8);
rev_through_bytes!(i16);
rev_through_bytes!(i32);
rev_through_bytes!(i64);
rev_through_bytes!(i128);
rev_through_bytes!(f32);
rev_through_bytes!(f64);

pub trait ReverseBitsNaive {
    fn reverse_bits_naive(self) -> Self;
}

impl<T> ReverseBitsNaive for T
where
    T: std::fmt::Binary + Num,
    <T as Num>::FromStrRadixErr: Debug,
{
    #[inline(always)]
    fn reverse_bits_naive(self) -> Self {
        Self::from_str_radix(
            &format!("{self:0width$b}", width = 8 * std::mem::size_of::<T>())
                .chars()
                .rev()
                .collect::<String>(),
            2,
        )
        .unwrap()
    }
}

pub trait ReverseBitsSimd {
    fn reverse_bits_simd(self) -> Self;
}

// macro to implement SIMD bit-reversion by converting to an array of bytes,
// reversing that and reversing each byte in itself
macro_rules! simd_rev_through_bytes {
    ($t:ty) => {
        impl ReverseBitsSimd for $t {
            #[inline(always)]
            fn reverse_bits_simd(self) -> Self {
                const N: usize = std::mem::size_of::<$t>();
                let bytes = Simd::from(self.to_ne_bytes()).reverse();

                let a = bytes & Simd::from([0b1000_0000; N]);
                let b = bytes & Simd::from([0b0100_0000; N]);
                let c = bytes & Simd::from([0b0010_0000; N]);
                let d = bytes & Simd::from([0b0001_0000; N]);
                let e = bytes & Simd::from([0b0000_1000; N]);
                let f = bytes & Simd::from([0b0000_0100; N]);
                let g = bytes & Simd::from([0b0000_0010; N]);
                let h = bytes & Simd::from([0b0000_0001; N]);
                Self::from_ne_bytes(
                    {
                        a >> Simd::from([7; N])
                            | b >> Simd::from([5; N])
                            | c >> Simd::from([3; N])
                            | d >> Simd::from([1; N])
                            | e << Simd::from([1; N])
                            | f << Simd::from([3; N])
                            | g << Simd::from([5; N])
                            | h << Simd::from([7; N])
                    }
                    .to_array(),
                )
            }
        }
    };
}

simd_rev_through_bytes!(u8);
simd_rev_through_bytes!(u16);
simd_rev_through_bytes!(u32);
simd_rev_through_bytes!(u64);
simd_rev_through_bytes!(u128);

pub trait ReverseBitsGpt {
    fn reverse_bits_gpt(self) -> Self;
}

impl ReverseBitsGpt for u8 {
    #[inline(always)]
    fn reverse_bits_gpt(self) -> Self {
        let mut n = self;
        let mut result = 0;
        let mut shift = 7;

        while n != 0 {
            result |= (n & 1) << shift;
            n >>= 1;
            shift -= 1;
        }

        result
    }
}

impl ReverseBitsGpt for u16 {
    #[inline(always)]
    fn reverse_bits_gpt(self) -> Self {
        let mut result = 0;
        let mut input = self;
        let mut shift = 15;

        while input != 0 {
            result |= (input & 1) << shift;
            input >>= 1;
            shift -= 1;
        }

        result
    }
}

impl ReverseBitsGpt for u32 {
    #[inline(always)]
    fn reverse_bits_gpt(self) -> Self {
        const REVERSE_LOOKUP: [u32; 16] = [
            0x0, 0x8, 0x4, 0xC, 0x2, 0xA, 0x6, 0xE, 0x1, 0x9, 0x5, 0xD, 0x3, 0xB, 0x7, 0xF,
        ];

        let mut reversed = 0;
        let mut shift = 28;

        for i in 0..8 {
            let bits = (self >> (i * 4)) & 0xF;
            reversed |= REVERSE_LOOKUP[bits as usize] << shift;
            shift -= 4;
        }

        reversed
    }
}

impl ReverseBitsGpt for u64 {
    #[inline(always)]
    fn reverse_bits_gpt(self) -> u64 {
        let mut result = 0;
        let mut value = self;

        for _ in 0..64 {
            result <<= 1;
            result |= value & 1;
            value >>= 1;
        }

        result
    }
}

impl ReverseBitsGpt for u128 {
    #[inline(always)]
    fn reverse_bits_gpt(self) -> Self {
        let mut result = 0;
        let mut value = self;

        for _ in 0..128 {
            result = (result << 1) | (value & 1);
            value >>= 1;
        }

        result
    }
}

/// Reverse bits based on a lookup table
pub trait ReverseBitsLut {
    fn reverse_bits_lut(self) -> Self;
}

impl ReverseBitsLut for u8 {
    #[inline(always)]
    fn reverse_bits_lut(self) -> Self {
        const LUT: [u8; 256] = [
            0_u8, 128, 64, 192, 32, 160, 96, 224, 16, 144, 80, 208, 48, 176, 112, 240, 8, 136, 72,
            200, 40, 168, 104, 232, 24, 152, 88, 216, 56, 184, 120, 248, 4, 132, 68, 196, 36, 164,
            100, 228, 20, 148, 84, 212, 52, 180, 116, 244, 12, 140, 76, 204, 44, 172, 108, 236, 28,
            156, 92, 220, 60, 188, 124, 252, 2, 130, 66, 194, 34, 162, 98, 226, 18, 146, 82, 210,
            50, 178, 114, 242, 10, 138, 74, 202, 42, 170, 106, 234, 26, 154, 90, 218, 58, 186, 122,
            250, 6, 134, 70, 198, 38, 166, 102, 230, 22, 150, 86, 214, 54, 182, 118, 246, 14, 142,
            78, 206, 46, 174, 110, 238, 30, 158, 94, 222, 62, 190, 126, 254, 1, 129, 65, 193, 33,
            161, 97, 225, 17, 145, 81, 209, 49, 177, 113, 241, 9, 137, 73, 201, 41, 169, 105, 233,
            25, 153, 89, 217, 57, 185, 121, 249, 5, 133, 69, 197, 37, 165, 101, 229, 21, 149, 85,
            213, 53, 181, 117, 245, 13, 141, 77, 205, 45, 173, 109, 237, 29, 157, 93, 221, 61, 189,
            125, 253, 3, 131, 67, 195, 35, 163, 99, 227, 19, 147, 83, 211, 51, 179, 115, 243, 11,
            139, 75, 203, 43, 171, 107, 235, 27, 155, 91, 219, 59, 187, 123, 251, 7, 135, 71, 199,
            39, 167, 103, 231, 23, 151, 87, 215, 55, 183, 119, 247, 15, 143, 79, 207, 47, 175, 111,
            239, 31, 159, 95, 223, 63, 191, 127, 255,
        ];
        LUT[self as usize]
    }
}

// macro to implement bit-reversion by converting to an array of bytes,
// reversing that and reversing each byte in itself
macro_rules! lut_rev_through_bytes {
    ($t:ty) => {
        impl ReverseBitsLut for $t {
            #[inline(always)]
            fn reverse_bits_lut(self) -> Self {
                let mut arr = self.to_ne_bytes();
                arr.reverse();
                for i in 0..std::mem::size_of::<$t>() {
                    arr[i] = arr[i].reverse_bits_lut();
                }
                <$t>::from_ne_bytes(arr)
            }
        }
    };
}

lut_rev_through_bytes!(u16);
lut_rev_through_bytes!(u32);
lut_rev_through_bytes!(u64);
lut_rev_through_bytes!(u128);

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{EitherOrBoth, Itertools};
    use proptest::prelude::*;

    use crate::ReverseBitsSimd;

    /// Returns true if the binary representation of x is the reversed one of y
    fn is_rev_pair<T>(x: T, y: T) -> bool
    where
        T: std::fmt::Binary,
    {
        format!("{x:0width$b}", width = 8 * std::mem::size_of::<T>())
            .chars()
            .rev()
            .zip_longest(format!("{y:0width$b}", width = 8 * std::mem::size_of::<T>()).chars())
            .all(|either_or_both| match either_or_both {
                EitherOrBoth::Both(x, y) => x == y,
                _ => false, // if one of the inputs is shorter something has to have gone wrong
            })
    }

    macro_rules! test_impls_for_type {
        ($t:ty, $reg:ident, $simd:ident, $naive:ident, $gpt:ident, $lut:ident) => {
            proptest! {
                #[test]
                fn $reg(x: $t) {
                    assert!(is_rev_pair(x, x.reverse_bits()))
                }

                #[test]
                fn $simd(x: $t) {
                    assert!(is_rev_pair(x, x.reverse_bits_simd()))
                }

                #[test]
                fn $naive(x: $t) {
                    assert!(is_rev_pair(x, x.reverse_bits_naive()))
                }

                #[test]
                fn $gpt(x: $t) {
                    assert!(is_rev_pair(x, x.reverse_bits_gpt()))
                }

                #[test]
                fn $lut(x: $t) {
                    assert!(is_rev_pair(x, x.reverse_bits_lut()))
                }
            }
        };
    }

    test_impls_for_type!(u8, reg_u8, simd_u8, naive_u8, gpt_u8, lut_u8);
    test_impls_for_type!(u16, reg_u16, simd_u16, naive_u16, gpt_u16, lut_u16);
    test_impls_for_type!(u32, reg_u32, simd_u32, naive_u32, gpt_u32, lut_u32);
    test_impls_for_type!(u64, reg_u64, simd_u64, naive_u64, gpt_u64, lut_u64);
    test_impls_for_type!(u128, reg_u128, simd_u128, naive_u128, gpt_u128, lut_u128);
}
