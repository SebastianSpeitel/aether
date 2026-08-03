use crate::primitive::{Primitive, U2, U3, U4, U5, U6, U7};

/// Fixed-capacity ring buffer operating on arbitrary bit payloads.
#[derive(Clone, Copy)]
pub struct BitRing<const N: usize> {
    bytes: [u8; N],
    pub head: usize,
}

impl<const N: usize> Default for BitRing<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> BitRing<N> {
    /// Mask for byte array index calculation.
    const BYTE_INDEX_MASK: usize = N - 1;
    /// Mask for bit index calculation.
    const BIT_INDEX_MASK: usize = (N * 8) - 1;

    const _CHECK_POWER_OF_TWO: () = {
        assert!(
            N.is_power_of_two(),
            "BitRing byte capacity N must be a power of two"
        );
    };

    /// Create a new `BitRing`.
    ///
    /// # Panics
    ///
    /// Fails compilation if `N` is not a power of two.
    #[must_use]
    pub const fn new() -> Self {
        let () = Self::_CHECK_POWER_OF_TWO;
        Self {
            bytes: [0; N],
            head: 0,
        }
    }

    /// Return total capacity in bits (`N * 8`).
    #[inline]
    #[must_use]
    pub const fn capacity_bits(&self) -> usize {
        N * 8
    }

    /// Mutate a `Primitive` value at bit position `pos` without advancing `self.head`.
    #[inline]
    pub fn set<P: Primitive>(&mut self, pos: usize, val: P) {
        debug_assert!(u32::from(P::BITS) <= 128, "P::BITS exceeds 128");
        if P::BITS == 0 {
            return;
        }

        let byte_idx = (pos >> 3) & Self::BYTE_INDEX_MASK;
        let bit_offset = pos & 7;
        let bytes_needed = (bit_offset + usize::from(P::BITS)).div_ceil(8);

        let bits = val.as_usize() as u128;
        let mask = u128::MAX.checked_shr(128 - u32::from(P::BITS)).unwrap_or(0);

        let payload = (bits & mask) << bit_offset;
        let mask_payload = mask << bit_offset;

        for i in 0..bytes_needed {
            let target_idx = (byte_idx + i) & Self::BYTE_INDEX_MASK;
            let byte_val = ((payload >> (i * 8)) & 0xFF) as u8;
            let byte_mask = ((mask_payload >> (i * 8)) & 0xFF) as u8;

            self.bytes[target_idx] &= !byte_mask;
            self.bytes[target_idx] |= byte_val;
        }
    }

    /// Push any `Primitive` value into `BitRing` and advance `self.head`.
    #[inline]
    pub fn push<P: Primitive>(&mut self, val: P) {
        self.set::<P>(self.head, val);
        self.head = (self.head + usize::from(P::BITS)) & Self::BIT_INDEX_MASK;
    }

    /// Push multiple boolean items from an iterator into `BitRing`.
    #[inline]
    pub fn push_bits(&mut self, mut bits: impl Iterator<Item = bool>, count: u8) {
        for _ in 0..count {
            self.push(bits.next().unwrap_or(false));
        }
    }

    /// Retrieve a `Primitive` value starting at bit position `pos` without mutating `BitRing`.
    #[inline(always)]
    #[must_use]
    pub fn get<P: Primitive>(&self, pos: usize) -> P {
        debug_assert!(u32::from(P::BITS) <= 128, "P::BITS exceeds 128");

        let byte_idx = (pos >> 3) & Self::BYTE_INDEX_MASK;
        let bit_offset = pos & 7;
        let bytes_needed = (bit_offset + usize::from(P::BITS)).div_ceil(8);

        let mut raw = 0u128;
        for i in 0..bytes_needed {
            raw |= u128::from(self.bytes[(byte_idx + i) & Self::BYTE_INDEX_MASK]) << (i * 8);
        }

        let mask = u128::MAX.checked_shr(128 - u32::from(P::BITS)).unwrap_or(0);

        let val = (raw >> bit_offset) & mask;
        P::from_usize(val as usize)
    }

    /// Create an iterator over bit positions starting at `pos`.
    #[inline]
    #[must_use]
    pub const fn iter(&self, pos: usize) -> RingIter<'_, N> {
        RingIter { ring: self, pos }
    }
}

impl<const N: usize> Extend<bool> for BitRing<N> {
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = bool>,
    {
        for b in iter {
            self.push(b);
        }
    }
}

/// Trait providing multi-bit peeking operations on bit iterators.
pub trait Peek: Iterator<Item = bool> {
    /// Peek `N` bits as `u8` without consuming the iterator position.
    fn peek_n<const N: usize>(&self) -> u8;
}

/// Iterator over bits inside a `BitRing`.
pub struct RingIter<'a, const N: usize> {
    ring: &'a BitRing<N>,
    pub(crate) pos: usize,
}

impl<const N: usize> RingIter<'_, N> {
    /// Get primitive value at current position without advancing position.
    #[inline]
    pub fn get<P: Primitive>(&self) -> P {
        self.ring.get::<P>(self.pos)
    }

    /// Read primitive value at current position and advance position by `P::BITS`.
    #[inline]
    pub fn read<P: Primitive>(&mut self) -> P {
        let val = self.get::<P>();
        self.pos = (self.pos + usize::from(P::BITS)) & BitRing::<N>::BIT_INDEX_MASK;
        val
    }
}

impl<const N: usize> Iterator for RingIter<'_, N> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        Some(self.read::<bool>())
    }
}

impl<const RING: usize> Peek for RingIter<'_, RING> {
    #[inline]
    fn peek_n<const N: usize>(&self) -> u8 {
        debug_assert!(N <= 8, "peek_n supports up to 8 bits");
        match N {
            1 => self.get::<bool>() as u8,
            2 => self.get::<U2>().0,
            3 => self.get::<U3>().0,
            4 => self.get::<U4>().0,
            5 => self.get::<U5>().0,
            6 => self.get::<U6>().0,
            7 => self.get::<U7>().0,
            8 => self.get::<u8>(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use proptest::prelude::*;
    use std::vec::Vec;

    proptest! {
        #[test]
        fn prop_push_matches_push_bit(val in any::<u16>()) {
            let mut ring1 = BitRing::<16>::new();
            let mut ring2 = BitRing::<16>::new();

            ring1.push(val);

            for i in 0..16 {
                let bit = ((val >> i) & 1) != 0;
                ring2.push(bit);
            }

            let bits1: Vec<bool> = ring1.iter(0).take(16).collect();
            let bits2: Vec<bool> = ring2.iter(0).take(16).collect();
            prop_assert_eq!(bits1, bits2);
        }

        #[test]
        fn prop_read_get_equivalence(val in any::<u16>()) {
            let mut ring = BitRing::<16>::new();
            ring.push(val);

            let peeked: u16 = ring.get(0);
            let mut reader = ring.iter(0);
            let read_val: u16 = reader.read();

            prop_assert_eq!(peeked, val);
            prop_assert_eq!(read_val, val);
        }

        #[test]
        fn prop_set_get_equivalence(val in any::<u16>(), pos in 0..64usize) {
            let mut ring = BitRing::<16>::new();
            ring.set(pos, val);
            let read_back: u16 = ring.get(pos);
            prop_assert_eq!(read_back, val);
        }

        #[test]
        fn prop_bool_primitive(val in any::<bool>()) {
            let mut ring = BitRing::<2>::new();
            ring.push(val);
            let peeked: bool = ring.get(0);
            let mut reader = ring.iter(0);
            let read_val: bool = reader.read();
            prop_assert_eq!(peeked, val);
            prop_assert_eq!(read_val, val);
        }

        #[test]
        fn prop_cross_byte_boundary(val in 0..1024u16, offset in 1..15usize) {
            let mut ring = BitRing::<4>::new();
            let u_val = crate::primitive::U10(val);
            ring.set(offset, u_val);
            let read_back: crate::primitive::U10 = ring.get(offset);
            prop_assert_eq!(read_back, u_val);
        }

        #[test]
        fn prop_ring_wraparound(val1 in any::<u16>(), val2 in any::<u16>(), val3 in any::<u16>()) {
            let mut ring = BitRing::<4>::new();
            ring.push(val1); // head 0..16
            ring.push(val2); // head 16..32
            ring.push(val3); // head 32 % 32 = 0..16 (overwrites val1)

            let read_val2: u16 = ring.get(16);
            let read_val3: u16 = ring.get(0);
            prop_assert_eq!(read_val2, val2);
            prop_assert_eq!(read_val3, val3);
        }
    }
}
