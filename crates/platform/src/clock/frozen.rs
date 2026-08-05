use aether_core::clock::{Clock, Duration, Instant, SignedDuration};

/// Deterministic Zero-Sized Type (ZST) test clock.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrozenClock;

impl Clock for FrozenClock {
    type InstantRepr = u64;
    type DurationRepr = u64;
    type SignedDurationRepr = i64;

    const DURATION_ZERO: Self::DurationRepr = 0;

    #[inline]
    fn now(&self) -> Instant<Self> {
        Instant::from_inner(0)
    }

    #[inline]
    fn duration_since(lhs: Instant<Self>, rhs: Instant<Self>) -> Duration<Self> {
        Duration::from_inner(lhs.into_inner().saturating_sub(rhs.into_inner()))
    }

    #[inline]
    fn offset_from(lhs: Instant<Self>, rhs: Instant<Self>) -> SignedDuration<Self> {
        let (lhs_val, rhs_val) = (lhs.into_inner(), rhs.into_inner());
        let diff = (lhs_val as i128) - (rhs_val as i128);
        let clamped = diff.clamp(i64::MIN as i128, i64::MAX as i128) as i64;
        SignedDuration::from_inner(clamped)
    }

    #[inline]
    fn add_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner().saturating_add(rhs.into_inner()))
    }

    #[inline]
    fn sub_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner().saturating_sub(rhs.into_inner()))
    }

    #[inline]
    fn from_duration(duration: core::time::Duration) -> Self::DurationRepr {
        duration.as_millis() as u64
    }

    #[inline]
    fn into_duration(repr: Self::DurationRepr) -> core::time::Duration {
        core::time::Duration::from_millis(repr)
    }

    #[inline]
    fn is_negative(repr: Self::SignedDurationRepr) -> bool {
        repr < 0
    }
}
