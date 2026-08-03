#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use crate::bitring::{BitRing, RingIter};
pub use crate::primitive::{Primitive, U2, U6, U10};

/// Trait defining an encoding scheme for delta-compressed time-series data.
pub trait Encoding {
    /// Source value type.
    type Value: Copy;
    /// Compression state tracking type.
    type State: Default + Copy;

    /// Maximum bit count required for an encoded sample.
    const MAX_BITS: usize;
    /// Bit value representing a keyframe header flag.
    const KEY_FLAG: usize;
    /// Minimum delta step supported in delta frame mode.
    const MIN_DELTA: isize;
    /// Maximum delta step supported in delta frame mode.
    const MAX_DELTA: isize;

    /// Encode a single sample value into `writer`.
    fn encode<const N: usize>(
        value: Self::Value,
        force_keyframe: bool,
        state: &mut Self::State,
        writer: &mut BitRing<N>,
    ) -> bool;

    /// Decode a single sample value from `reader`.
    fn decode<const N: usize>(
        reader: &mut RingIter<'_, N>,
        state: &mut Self::State,
    ) -> (Self::Value, bool);

    /// Check if the next sample in `reader` is a keyframe.
    fn is_keyframe(reader: &impl crate::bitring::Peek) -> bool;

    /// Denoise a sample value against the current `state` given a `DEADBAND` threshold.
    #[inline]
    fn denoise<const DEADBAND: usize>(value: Self::Value, _state: &Self::State) -> Self::Value {
        value
    }
}

/// Generic difference encoding scheme parameterized over value type `T` and flag width `F`.
pub struct DiffEncoding<T, F = U2>(core::marker::PhantomData<(T, F)>);

impl<T: Primitive, F: Primitive> Encoding for DiffEncoding<T, F> {
    type Value = T;
    type State = T;

    const MAX_BITS: usize = (F::BITS as usize) + (T::BITS as usize);
    const KEY_FLAG: usize = (1usize << F::BITS) - 1;
    const MAX_DELTA: isize = (1isize << (F::BITS - 1)) - 1;
    const MIN_DELTA: isize = -Self::MAX_DELTA;

    #[inline]
    fn encode<const N: usize>(
        val: T,
        force_key: bool,
        state: &mut T,
        writer: &mut BitRing<N>,
    ) -> bool {
        let diff = val.difference_as_isize(*state);

        if !force_key && (Self::MIN_DELTA..=Self::MAX_DELTA).contains(&diff) {
            let encoded_delta = (diff + Self::MAX_DELTA) as usize;
            writer.push(F::from_usize(encoded_delta));
            *state = val;
            false
        } else {
            writer.push(F::from_usize(Self::KEY_FLAG));
            writer.push(val);
            *state = val;
            true
        }
    }

    #[inline]
    fn decode<const N: usize>(reader: &mut RingIter<'_, N>, state: &mut T) -> (T, bool) {
        let header = reader.read::<F>().as_usize();

        if header == Self::KEY_FLAG {
            *state = reader.read::<T>();
            (*state, true)
        } else {
            let diff = (header as isize) - Self::MAX_DELTA;
            *state = state.wrapping_add_signed(diff);
            (*state, false)
        }
    }

    #[inline]
    fn is_keyframe(reader: &impl crate::bitring::Peek) -> bool {
        let key_flag = Self::KEY_FLAG as u8;
        match F::BITS {
            1 => reader.peek_n::<1>() == key_flag,
            2 => reader.peek_n::<2>() == key_flag,
            3 => reader.peek_n::<3>() == key_flag,
            4 => reader.peek_n::<4>() == key_flag,
            5 => reader.peek_n::<5>() == key_flag,
            6 => reader.peek_n::<6>() == key_flag,
            7 => reader.peek_n::<7>() == key_flag,
            8 => reader.peek_n::<8>() == key_flag,
            _ => false,
        }
    }

    #[inline]
    fn denoise<const DEADBAND: usize>(val: T, state: &T) -> T {
        if DEADBAND == 0 {
            return val;
        }
        let diff = val.difference_as_isize(*state);
        let min_bound = Self::MIN_DELTA - (DEADBAND as isize);
        let max_bound = Self::MAX_DELTA + (DEADBAND as isize);

        if (min_bound..=max_bound).contains(&diff) {
            let clamped_diff = diff.clamp(Self::MIN_DELTA, Self::MAX_DELTA);
            state.wrapping_add_signed(clamped_diff)
        } else {
            val
        }
    }
}

/// State tracking value and velocity for gradient encoding.
#[derive(Copy, Clone, Default)]
pub struct GradientState<T> {
    pub value: T,
    pub velocity: isize,
}

/// Generic gradient encoding scheme parameterized over value type `T`, flag width `F`, and velocity width `V`.
pub struct GradientEncoding<T, F = U2, V = u8>(core::marker::PhantomData<(T, F, V)>);

impl<T: Primitive, F: Primitive, V: Primitive> Encoding for GradientEncoding<T, F, V> {
    type Value = T;
    type State = GradientState<T>;

    const MAX_BITS: usize = (F::BITS as usize) + (T::BITS as usize) + (V::BITS as usize);
    const KEY_FLAG: usize = (1usize << F::BITS) - 1;
    const MAX_DELTA: isize = (1isize << (F::BITS - 1)) - 1;
    const MIN_DELTA: isize = -Self::MAX_DELTA;

    #[inline]
    fn encode<const N: usize>(
        val: T,
        force_key: bool,
        state: &mut Self::State,
        writer: &mut BitRing<N>,
    ) -> bool {
        let v_max = (1isize << (V::BITS - 1)) - 1;
        let v_min = -v_max;

        let expected_base = state.value.wrapping_add_signed(state.velocity);
        let grad_diff = val.difference_as_isize(expected_base);

        if !force_key && (Self::MIN_DELTA..=Self::MAX_DELTA).contains(&grad_diff) {
            let encoded_flag = (grad_diff + Self::MAX_DELTA) as usize;
            writer.push(F::from_usize(encoded_flag));

            state.velocity += grad_diff;
            state.value = val;
            false
        } else {
            writer.push(F::from_usize(Self::KEY_FLAG));
            writer.push(val);

            let raw_vel = val.difference_as_isize(state.value);
            let new_vel = raw_vel.clamp(v_min, v_max);

            writer.push(V::from_usize(
                (new_vel as usize) & ((1usize << V::BITS) - 1),
            ));

            state.velocity = new_vel;
            state.value = val;
            true
        }
    }

    #[inline]
    fn decode<const N: usize>(reader: &mut RingIter<'_, N>, state: &mut Self::State) -> (T, bool) {
        let flag = reader.read::<F>().as_usize();

        if flag == Self::KEY_FLAG {
            state.value = reader.read::<T>();

            let raw_vel = reader.read::<V>();
            state.velocity = raw_vel.difference_as_isize(V::default());
            (state.value, true)
        } else {
            let grad_diff = (flag as isize) - Self::MAX_DELTA;
            state.velocity += grad_diff;
            state.value = state.value.wrapping_add_signed(state.velocity);
            (state.value, false)
        }
    }

    #[inline]
    fn is_keyframe(reader: &impl crate::bitring::Peek) -> bool {
        let key_flag = Self::KEY_FLAG as u8;
        match F::BITS {
            1 => reader.peek_n::<1>() == key_flag,
            2 => reader.peek_n::<2>() == key_flag,
            3 => reader.peek_n::<3>() == key_flag,
            4 => reader.peek_n::<4>() == key_flag,
            5 => reader.peek_n::<5>() == key_flag,
            6 => reader.peek_n::<6>() == key_flag,
            7 => reader.peek_n::<7>() == key_flag,
            8 => reader.peek_n::<8>() == key_flag,
            _ => false,
        }
    }

    #[inline]
    fn denoise<const DEADBAND: usize>(val: T, state: &GradientState<T>) -> T {
        if DEADBAND == 0 {
            return val;
        }
        let expected_base = state.value.wrapping_add_signed(state.velocity);
        let grad_diff = val.difference_as_isize(expected_base);
        let min_bound = Self::MIN_DELTA - (DEADBAND as isize);
        let max_bound = Self::MAX_DELTA + (DEADBAND as isize);

        if (min_bound..=max_bound).contains(&grad_diff) {
            let clamped_diff = grad_diff.clamp(Self::MIN_DELTA, Self::MAX_DELTA);
            expected_base.wrapping_add_signed(clamped_diff)
        } else {
            val
        }
    }
}

/// Optimized gradient encoding scheme for 10-bit integer values (`U10`).
pub type GradientEncodingU10 = GradientEncoding<U10, U2, U6>;

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_diff_encoding_u10(val in 0..1024u16) {
            let mut ring = BitRing::<16>::new();
            let mut state = U10(0);
            let input = U10(val);

            DiffEncoding::<U10>::encode(input, true, &mut state, &mut ring);

            let mut reader = ring.iter(0);
            let mut dec_state = U10(0);
            let (decoded, is_key) = DiffEncoding::<U10>::decode(&mut reader, &mut dec_state);

            prop_assert!(is_key);
            prop_assert_eq!(decoded, input);
        }
    }
}
