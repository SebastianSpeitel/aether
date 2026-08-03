#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

/// Trait representing a primitive bit-encodable type.
pub trait Primitive: Copy + Default + PartialEq {
    /// Number of bits required to store this primitive.
    const BITS: u8;
    /// Unit increment constant for this primitive.
    const ONE: Self;

    /// Convert this primitive to a `usize`.
    fn as_usize(self) -> usize;
    /// Construct this primitive from a `usize`.
    fn from_usize(val: usize) -> Self;

    /// Perform wrapping addition.
    #[must_use]
    fn wrapping_add(self, rhs: Self) -> Self;
    /// Perform wrapping subtraction.
    #[must_use]
    fn wrapping_sub(self, rhs: Self) -> Self;
    /// Perform wrapping addition with a signed pointer-sized integer offset.
    #[must_use]
    fn wrapping_add_signed(self, rhs: isize) -> Self;
    /// Compute signed difference relative to another value as `isize`.
    fn difference_as_isize(self, other: Self) -> isize;
}

pub type U1 = bool;

impl Primitive for bool {
    const BITS: u8 = 1;
    const ONE: Self = true;

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_usize(val: usize) -> Self {
        (val & 1) != 0
    }

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        self ^ rhs
    }

    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        self ^ rhs
    }

    #[inline]
    fn wrapping_add_signed(self, rhs: isize) -> Self {
        self ^ ((rhs & 1) != 0)
    }

    #[inline]
    fn difference_as_isize(self, other: Self) -> isize {
        isize::from(self ^ other)
    }
}

impl Primitive for u8 {
    const BITS: Self = 8;
    const ONE: Self = 1;

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_usize(val: usize) -> Self {
        val as Self
    }

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }

    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }

    #[inline]
    fn wrapping_add_signed(self, rhs: isize) -> Self {
        self.wrapping_add(rhs as Self)
    }

    #[inline]
    fn difference_as_isize(self, other: Self) -> isize {
        isize::from(self.wrapping_sub(other).cast_signed())
    }
}

impl Primitive for u16 {
    const BITS: u8 = 16;
    const ONE: Self = 1;

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_usize(val: usize) -> Self {
        val as Self
    }

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }

    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }

    #[inline]
    fn wrapping_add_signed(self, rhs: isize) -> Self {
        self.wrapping_add(rhs as Self)
    }

    #[inline]
    fn difference_as_isize(self, other: Self) -> isize {
        isize::from(self.wrapping_sub(other).cast_signed())
    }
}

macro_rules! impl_primitive_wrapper {
    ($type:ident, $inner:ty, $bits:expr) => {
        #[doc = concat!("A ", stringify!($bits), "-bit unsigned primitive wrapper type.")]
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
        pub struct $type(pub $inner);

        impl $type {
            /// Bitmask corresponding to the valid bit field width.
            pub const MASK: $inner = (((1u128 << $bits) - 1) as $inner);

            /// Assert that the wrapped value fits within `Self::MASK`.
            #[inline(always)]
            pub const fn assert_valid(self) {
                debug_assert!((self.0 & Self::MASK) == self.0, "value exceeds bit mask");
            }
        }

        impl Primitive for $type {
            const BITS: u8 = $bits;
            const ONE: Self = Self(1);

            #[inline]
            fn as_usize(self) -> usize {
                self.assert_valid();
                self.0 as usize
            }

            #[inline]
            fn from_usize(val: usize) -> Self {
                Self((val as $inner) & Self::MASK)
            }

            #[inline]
            fn wrapping_add(self, rhs: Self) -> Self {
                self.assert_valid();
                rhs.assert_valid();
                Self((self.0.wrapping_add(rhs.0)) & Self::MASK)
            }

            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self {
                self.assert_valid();
                rhs.assert_valid();
                Self((self.0.wrapping_sub(rhs.0)) & Self::MASK)
            }

            #[inline]
            fn wrapping_add_signed(self, rhs: isize) -> Self {
                self.assert_valid();
                let val = i128::from(self.0) + (rhs as i128);
                let wrapped = val.rem_euclid(1i128 << $bits);
                Self((wrapped as $inner) & Self::MASK)
            }

            #[inline]
            fn difference_as_isize(self, other: Self) -> isize {
                self.assert_valid();
                other.assert_valid();
                let mask = (1u128 << $bits) - 1;
                let diff = (self.0.wrapping_sub(other.0) as u128) & mask;
                let half = 1u128 << ($bits - 1);
                if diff >= half {
                    let signed = (diff as i128) - (1i128 << $bits);
                    signed as isize
                } else {
                    diff as isize
                }
            }
        }
    };
}

impl_primitive_wrapper!(U2, u8, 2);
impl_primitive_wrapper!(U3, u8, 3);
impl_primitive_wrapper!(U4, u8, 4);
impl_primitive_wrapper!(U5, u8, 5);
impl_primitive_wrapper!(U6, u8, 6);
impl_primitive_wrapper!(U7, u8, 7);
impl_primitive_wrapper!(U8, u8, 8);

impl_primitive_wrapper!(U9, u16, 9);
impl_primitive_wrapper!(U10, u16, 10);
impl_primitive_wrapper!(U11, u16, 11);
impl_primitive_wrapper!(U12, u16, 12);
impl_primitive_wrapper!(U13, u16, 13);
impl_primitive_wrapper!(U14, u16, 14);
impl_primitive_wrapper!(U15, u16, 15);
impl_primitive_wrapper!(U16, u16, 16);

impl_primitive_wrapper!(U17, u32, 17);
impl_primitive_wrapper!(U18, u32, 18);
impl_primitive_wrapper!(U19, u32, 19);
impl_primitive_wrapper!(U20, u32, 20);
impl_primitive_wrapper!(U21, u32, 21);
impl_primitive_wrapper!(U22, u32, 22);
impl_primitive_wrapper!(U23, u32, 23);
impl_primitive_wrapper!(U24, u32, 24);
impl_primitive_wrapper!(U25, u32, 25);
impl_primitive_wrapper!(U26, u32, 26);
impl_primitive_wrapper!(U27, u32, 27);
impl_primitive_wrapper!(U28, u32, 28);
impl_primitive_wrapper!(U29, u32, 29);
impl_primitive_wrapper!(U30, u32, 30);
impl_primitive_wrapper!(U31, u32, 31);
impl_primitive_wrapper!(U32, u32, 32);

impl_primitive_wrapper!(U33, u64, 33);
impl_primitive_wrapper!(U34, u64, 34);
impl_primitive_wrapper!(U35, u64, 35);
impl_primitive_wrapper!(U36, u64, 36);
impl_primitive_wrapper!(U37, u64, 37);
impl_primitive_wrapper!(U38, u64, 38);
impl_primitive_wrapper!(U39, u64, 39);
impl_primitive_wrapper!(U40, u64, 40);
impl_primitive_wrapper!(U41, u64, 41);
impl_primitive_wrapper!(U42, u64, 42);
impl_primitive_wrapper!(U43, u64, 43);
impl_primitive_wrapper!(U44, u64, 44);
impl_primitive_wrapper!(U45, u64, 45);
impl_primitive_wrapper!(U46, u64, 46);
impl_primitive_wrapper!(U47, u64, 47);
impl_primitive_wrapper!(U48, u64, 48);
impl_primitive_wrapper!(U49, u64, 49);
impl_primitive_wrapper!(U50, u64, 50);
impl_primitive_wrapper!(U51, u64, 51);
impl_primitive_wrapper!(U52, u64, 52);
impl_primitive_wrapper!(U53, u64, 53);
impl_primitive_wrapper!(U54, u64, 54);
impl_primitive_wrapper!(U55, u64, 55);
impl_primitive_wrapper!(U56, u64, 56);
impl_primitive_wrapper!(U57, u64, 57);
impl_primitive_wrapper!(U58, u64, 58);
impl_primitive_wrapper!(U59, u64, 59);
impl_primitive_wrapper!(U60, u64, 60);
impl_primitive_wrapper!(U61, u64, 61);
impl_primitive_wrapper!(U62, u64, 62);
impl_primitive_wrapper!(U63, u64, 63);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_u2_bounds(a in 0..4u8, b in 0..4u8) {
            let u_a = U2(a);
            let u_b = U2(b);
            prop_assert!(u_a.wrapping_add(u_b).0 <= U2::MASK);
            prop_assert!(u_a.wrapping_sub(u_b).0 <= U2::MASK);
            prop_assert_eq!(u_a.as_usize(), a as usize);
        }

        #[test]
        fn prop_u6_bounds(a in 0..64u8, b in 0..64u8) {
            let u_a = U6(a);
            let u_b = U6(b);
            prop_assert!(u_a.wrapping_add(u_b).0 <= U6::MASK);
            prop_assert!(u_a.wrapping_sub(u_b).0 <= U6::MASK);
            prop_assert_eq!(u_a.as_usize(), a as usize);
        }

        #[test]
        fn prop_u10_bounds(a in 0..1024u16, b in 0..1024u16) {
            let u_a = U10(a);
            let u_b = U10(b);
            prop_assert!(u_a.wrapping_add(u_b).0 <= U10::MASK);
            prop_assert!(u_a.wrapping_sub(u_b).0 <= U10::MASK);
            prop_assert_eq!(u_a.as_usize(), a as usize);
        }

        #[test]
        fn prop_u32_bounds(a in any::<u32>()) {
            let val = a & U32::MASK;
            let u_a = U32(val);
            prop_assert_eq!(u_a.as_usize(), val as usize);
        }

        #[test]
        fn prop_u63_bounds(a in any::<u64>()) {
            let val = a & U63::MASK;
            let u_a = U63(val);
            prop_assert_eq!(u_a.as_usize(), val as usize);
        }
    }
}
