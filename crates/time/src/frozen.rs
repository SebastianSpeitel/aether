use crate::{Clock, Duration, Instant, SignedDuration};

/// A dummy clock that does not progress.
/// Used primarily for immediate task yielding where time tracking is not required.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct FrozenClock;

impl Clock for FrozenClock {
    type DurationRepr = ();
    type SignedDurationRepr = ();
    type InstantRepr = ();

    const DURATION_ZERO: Self::DurationRepr = ();

    #[inline]
    fn now() -> Self::InstantRepr {}

    #[inline]
    fn duration_since(_lhs: Instant<Self>, _rhs: Instant<Self>) -> Duration<Self> {
        Duration::from_inner(())
    }

    #[inline]
    fn offset_from(_lhs: Instant<Self>, _rhs: Instant<Self>) -> SignedDuration<Self> {
        SignedDuration::from_inner(())
    }

    #[inline]
    fn add_duration(_lhs: Instant<Self>, _rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(())
    }

    #[inline]
    fn sub_duration(_lhs: Instant<Self>, _rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(())
    }

    #[inline]
    fn from_duration(_duration: core::time::Duration) -> Self::DurationRepr {}

    #[inline]
    fn into_duration(_repr: Self::DurationRepr) -> core::time::Duration {
        core::time::Duration::default()
    }

    #[inline]
    fn is_negative(_repr: Self::SignedDurationRepr) -> bool {
        false
    }
}
