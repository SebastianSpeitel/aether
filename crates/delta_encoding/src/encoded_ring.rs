#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use crate::bitring::{BitRing, RingIter};
use crate::encoding::Encoding;

const KEYFRAME_INTERVAL: usize = 32;

/// Fixed-capacity ring buffer container storing delta-encoded time-series values.
pub struct EncodedRing<const N: usize, E: Encoding> {
    ring: BitRing<N>,
    tail: usize,
    item_count: usize,
    samples_since_keyframe: usize,
    last_state: E::State,
}

impl<const N: usize, E: Encoding> Default for EncodedRing<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, E: Encoding> EncodedRing<N, E> {
    /// Create a new `EncodedRing`.
    ///
    /// # Panics
    ///
    /// Panics if `N * 8 < E::MAX_BITS`, i.e. if the byte buffer is too small
    /// to hold even a single keyframe.
    #[must_use]
    pub const fn new() -> Self {
        assert!(
            N * 8 >= E::MAX_BITS,
            "Buffer must fit at least one keyframe"
        );
        Self {
            ring: BitRing::new(),
            tail: 0,
            item_count: 0,
            samples_since_keyframe: 0,
            last_state: E::DEFAULT_STATE,
        }
    }

    /// Return total ring capacity in bits (`N * 8`).
    #[inline]
    pub const fn capacity_bits(&self) -> usize {
        N * 8
    }

    /// Return current length of written data in bits.
    #[inline]
    pub const fn bit_len(&self) -> usize {
        if self.item_count == 0 {
            0
        } else if self.ring.head > self.tail {
            self.ring.head - self.tail
        } else if self.ring.head < self.tail {
            self.capacity_bits() - self.tail + self.ring.head
        } else {
            self.capacity_bits()
        }
    }

    /// Return remaining unwritten space in bits.
    #[inline]
    pub const fn available_bits(&self) -> usize {
        self.capacity_bits() - self.bit_len()
    }

    /// Push a sample value into the encoded ring buffer (exact lossless by default).
    #[inline]
    pub fn push(&mut self, value: E::Value) {
        while self.available_bits() < E::MAX_BITS {
            self.drop_to_next_keyframe();

            if self.item_count == 0 {
                self.ring.head = 0;
                self.tail = 0;
                self.samples_since_keyframe = 0;
                self.last_state = E::State::default();
                break;
            }
        }

        let force_keyframe =
            self.item_count == 0 || self.samples_since_keyframe >= KEYFRAME_INTERVAL;
        let is_key = E::encode(value, force_keyframe, &mut self.last_state, &mut self.ring);
        self.item_count += 1;
        if is_key {
            self.samples_since_keyframe = 1;
        } else {
            self.samples_since_keyframe += 1;
        }
    }

    /// Push a sample value into the encoded ring buffer with a const generic `DENOISE` threshold.
    #[inline]
    pub fn push_denoised<const DENOISE: usize>(&mut self, value: E::Value) {
        let denoised_val = E::denoise::<DENOISE>(value, &self.last_state);
        self.push(denoised_val);
    }

    #[inline]
    fn drop_to_next_keyframe(&mut self) {
        if self.item_count == 0 {
            return;
        }

        let mut reader = self.ring.iter(self.tail);
        let mut dummy_state = E::State::default();

        let (_, is_key) = E::decode(&mut reader, &mut dummy_state);
        debug_assert!(is_key, "Tail did not point to a keyframe!");
        let mut dropped = 1;

        while dropped < self.item_count {
            if E::is_keyframe(&reader) {
                break;
            }
            E::decode(&mut reader, &mut dummy_state);
            dropped += 1;
        }

        self.tail = reader.pos;
        self.item_count -= dropped;
    }

    /// Return an iterator over decoded values in this ring buffer.
    #[inline]
    pub fn iter(&self) -> EncodedIter<'_, N, E> {
        EncodedIter {
            reader: self.ring.iter(self.tail),
            state: E::State::default(),
            items_left: self.item_count,
            _marker: core::marker::PhantomData,
        }
    }

    /// Check if the container holds no items.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.item_count == 0
    }

    /// Return total item count stored in the ring buffer.
    #[inline]
    pub const fn len(&self) -> usize {
        self.item_count
    }
}

impl<const N: usize, E: Encoding> Extend<E::Value> for EncodedRing<N, E> {
    #[inline]
    fn extend<I: IntoIterator<Item = E::Value>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

/// Iterator over decoded sample values in an `EncodedRing`.
pub struct EncodedIter<'a, const N: usize, E: Encoding> {
    reader: RingIter<'a, N>,
    state: E::State,
    items_left: usize,
    _marker: core::marker::PhantomData<E>,
}

impl<const N: usize, E: Encoding> Iterator for EncodedIter<'_, N, E> {
    type Item = E::Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.items_left == 0 {
            None
        } else {
            let (val, _) = E::decode(&mut self.reader, &mut self.state);
            self.items_left -= 1;
            Some(val)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.items_left, Some(self.items_left))
    }
}

impl<const N: usize, E: Encoding> ExactSizeIterator for EncodedIter<'_, N, E> {}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::encoding::{DiffEncoding, GradientEncoding};
    use crate::primitive::{
        U1, U2, U3, U4, U5, U6, U7, U9, U10, U11, U12, U13, U14, U15, U17, U24, U32, U63,
    };
    use proptest::prelude::*;
    use std::vec::Vec;

    fn assert_prop_deterministic_iter<const N: usize, E: Encoding>(vals: &[E::Value])
    where
        E::Value: std::fmt::Debug + PartialEq,
    {
        let mut ring = EncodedRing::<N, E>::new();
        for &v in vals {
            ring.push(v);
        }

        let decoded1: Vec<_> = ring.iter().collect();
        let decoded2: Vec<_> = ring.iter().collect();

        assert_eq!(
            decoded1, decoded2,
            "Multiple iterators on same ring produced different outputs"
        );
    }

    fn assert_prop_capacity_bounds<const N: usize, E: Encoding>(vals: &[E::Value]) {
        let mut ring = EncodedRing::<N, E>::new();
        for &v in vals {
            ring.push(v);
            assert!(
                ring.bit_len() <= N * 8,
                "bit_len {} exceeded total capacity {}",
                ring.bit_len(),
                N * 8
            );
            assert!(
                ring.len() <= vals.len(),
                "len {} exceeds pushed count {}",
                ring.len(),
                vals.len()
            );
        }
    }

    fn assert_prop_suffix_match<const N: usize, E: Encoding>(vals: &[E::Value])
    where
        E::Value: std::fmt::Debug + PartialEq,
    {
        let mut ring = EncodedRing::<N, E>::new();
        for &v in vals {
            ring.push(v);
        }

        let decoded: Vec<_> = ring.iter().collect();
        let count = decoded.len();
        let expected_suffix = &vals[vals.len() - count..];

        assert_eq!(
            decoded.as_slice(),
            expected_suffix,
            "Ring content did not match exact suffix of pushed values"
        );
    }

    fn assert_prop_extend<const N: usize, E: Encoding>(initial: &[E::Value], extended: &[E::Value])
    where
        E::Value: std::fmt::Debug + PartialEq,
    {
        let mut ring = EncodedRing::<N, E>::new();

        for &v in initial {
            ring.push(v);
        }

        let mut all_vals = Vec::from(initial);
        ring.extend(extended.iter().copied());
        all_vals.extend(extended.iter().copied());

        let decoded: Vec<_> = ring.iter().collect();
        let count = decoded.len();

        if count > 0 {
            let expected_suffix = &all_vals[all_vals.len() - count..];
            assert_eq!(
                decoded.as_slice(),
                expected_suffix,
                "Ring content after extend did not match exact suffix"
            );
        }
    }

    macro_rules! test_encoding_properties {
        ($mod_name:ident, $encoding:ty, $val_gen:expr, $std_n:expr, $agg_n:expr) => {
            mod $mod_name {
                use super::*;

                proptest! {
                    #![proptest_config(ProptestConfig::with_cases(1000))]

                    #[test]
                    fn prop_deterministic_iter(vals in proptest::collection::vec($val_gen, 0..100)) {
                        assert_prop_deterministic_iter::<$std_n, $encoding>(&vals);
                    }

                    #[test]
                    fn prop_capacity_bounds(vals in proptest::collection::vec($val_gen, 0..200)) {
                        assert_prop_capacity_bounds::<$agg_n, $encoding>(&vals);
                    }

                    #[test]
                    fn prop_suffix_standard(vals in proptest::collection::vec($val_gen, 0..100)) {
                        assert_prop_suffix_match::<$std_n, $encoding>(&vals);
                    }

                    #[test]
                    fn prop_suffix_aggressive(vals in proptest::collection::vec($val_gen, 0..200)) {
                        assert_prop_suffix_match::<$agg_n, $encoding>(&vals);
                    }

                    #[test]
                    fn prop_extend_standard(
                        initial in proptest::collection::vec($val_gen, 0..50),
                        extended in proptest::collection::vec($val_gen, 0..50)
                    ) {
                        assert_prop_extend::<$std_n, $encoding>(&initial, &extended);
                    }

                    #[test]
                    fn prop_extend_aggressive(
                        initial in proptest::collection::vec($val_gen, 0..100),
                        extended in proptest::collection::vec($val_gen, 0..100)
                    ) {
                        assert_prop_extend::<$agg_n, $encoding>(&initial, &extended);
                    }

                    #[test]
                    fn prop_empty_input(_dummy in 0..1u8) {
                        let empty: Vec<<$encoding as Encoding>::Value> = Vec::new();
                        assert_prop_suffix_match::<$std_n, $encoding>(&empty);
                    }

                    #[test]
                    fn prop_constant_sequence(val in $val_gen, count in 1..100usize) {
                        let vals = vec![val; count];
                        assert_prop_suffix_match::<$std_n, $encoding>(&vals);
                    }
                }
            }
        };
    }

    test_encoding_properties!(suite_diff_u8_f1, DiffEncoding<u8, U1>, any::<u8>(), 64, 2);
    test_encoding_properties!(suite_diff_u8_f2, DiffEncoding<u8, U2>, any::<u8>(), 64, 2);
    test_encoding_properties!(suite_diff_u8_f4, DiffEncoding<u8, U4>, any::<u8>(), 64, 4);
    test_encoding_properties!(suite_diff_u16_f1, DiffEncoding<u16, U1>, any::<u16>(), 64, 4);
    test_encoding_properties!(suite_diff_u16_f2, DiffEncoding<u16, U2>, any::<u16>(), 64, 4);
    test_encoding_properties!(suite_diff_u16_f4, DiffEncoding<u16, U4>, any::<u16>(), 64, 4);

    test_encoding_properties!(suite_diff_u3_f2, DiffEncoding<U3, U2>, (0..8u8).prop_map(U3), 64, 2);
    test_encoding_properties!(suite_diff_u5_f3, DiffEncoding<U5, U3>, (0..32u8).prop_map(U5), 64, 2);
    test_encoding_properties!(suite_diff_u7_f4, DiffEncoding<U7, U4>, (0..128u8).prop_map(U7), 64, 2);
    test_encoding_properties!(suite_diff_u9_f3, DiffEncoding<U9, U3>, (0..512u16).prop_map(U9), 64, 4);
    test_encoding_properties!(suite_diff_u10_f3, DiffEncoding<U10, U3>, (0..1024u16).prop_map(U10), 64, 4);
    test_encoding_properties!(suite_diff_u11_f4, DiffEncoding<U11, U4>, (0..2048u16).prop_map(U11), 64, 4);
    test_encoding_properties!(suite_diff_u12_f3, DiffEncoding<U12, U3>, (0..4096u16).prop_map(U12), 64, 4);
    test_encoding_properties!(suite_diff_u13_f4, DiffEncoding<U13, U4>, (0..8192u16).prop_map(U13), 64, 4);
    test_encoding_properties!(suite_diff_u14_f3, DiffEncoding<U14, U3>, (0..16384u16).prop_map(U14), 64, 4);
    test_encoding_properties!(suite_diff_u15_f4, DiffEncoding<U15, U4>, (0..32768u16).prop_map(U15), 64, 4);
    test_encoding_properties!(suite_diff_u17_f5, DiffEncoding<U17, U5>, (0..131072u32).prop_map(U17), 64, 4);
    test_encoding_properties!(suite_diff_u24_f4, DiffEncoding<U24, U4>, (0..16777216u32).prop_map(U24), 64, 8);
    test_encoding_properties!(suite_diff_u32_f4, DiffEncoding<U32, U4>, any::<u32>().prop_map(U32), 64, 8);
    test_encoding_properties!(suite_diff_u63_f5, DiffEncoding<U63, U5>, (0..0x7FFF_FFFF_FFFF_FFFFu64).prop_map(U63), 64, 16);

    test_encoding_properties!(suite_gradient_u8_f1_v8, GradientEncoding<u8, U1, u8>, any::<u8>(), 64, 4);
    test_encoding_properties!(suite_gradient_u8_v8, GradientEncoding<u8, U2, u8>, any::<u8>(), 64, 4);
    test_encoding_properties!(suite_gradient_u16_v8, GradientEncoding<u16, U2, u8>, any::<u16>(), 64, 4);
    test_encoding_properties!(suite_gradient_u3_v2, GradientEncoding<U3, U2, U2>, (0..8u8).prop_map(U3), 64, 2);
    test_encoding_properties!(suite_gradient_u5_v3, GradientEncoding<U5, U2, U3>, (0..32u8).prop_map(U5), 64, 2);
    test_encoding_properties!(suite_gradient_u5_f3_v3, GradientEncoding<U5, U3, U3>, (0..32u8).prop_map(U5), 64, 2);
    test_encoding_properties!(suite_gradient_u6_v4, GradientEncoding<U6, U2, U4>, (0..64u8).prop_map(U6), 64, 2);
    test_encoding_properties!(suite_gradient_u7_v4, GradientEncoding<U7, U2, U4>, (0..128u8).prop_map(U7), 64, 2);
    test_encoding_properties!(suite_gradient_u9_v5, GradientEncoding<U9, U2, U5>, (0..512u16).prop_map(U9), 64, 4);
    test_encoding_properties!(suite_gradient_u10_v6, GradientEncoding<U10, U2, U6>, (0..1024u16).prop_map(U10), 64, 4);
    test_encoding_properties!(suite_gradient_u10_f4_v6, GradientEncoding<U10, U4, U6>, (0..1024u16).prop_map(U10), 64, 4);
    test_encoding_properties!(suite_gradient_u12_v6, GradientEncoding<U12, U2, U6>, (0..4096u16).prop_map(U12), 64, 4);
    test_encoding_properties!(suite_gradient_u15_v7, GradientEncoding<U15, U2, U7>, (0..32768u16).prop_map(U15), 64, 4);
    test_encoding_properties!(suite_gradient_u17_v8, GradientEncoding<U17, U2, u8>, (0..131072u32).prop_map(U17), 64, 8);
    test_encoding_properties!(suite_gradient_u32_v8, GradientEncoding<U32, U2, u8>, any::<u32>().prop_map(U32), 64, 8);
    test_encoding_properties!(suite_gradient_u63_v8, GradientEncoding<U63, U2, u8>, (0..0x7FFF_FFFF_FFFF_FFFFu64).prop_map(U63), 64, 16);
}
