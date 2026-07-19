extern crate std;
use super::{Clock, Duration, Instant, SignedDuration};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct StdClock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignedStdDuration {
    Positive(core::time::Duration),
    Negative(core::time::Duration),
}

impl Default for SignedStdDuration {
    #[inline]
    fn default() -> Self {
        Self::Positive(core::time::Duration::ZERO)
    }
}

impl Clock for StdClock {
    type InstantRepr = std::time::Instant;
    type DurationRepr = core::time::Duration;
    type SignedDurationRepr = SignedStdDuration;

    const DURATION_ZERO: Self::DurationRepr = core::time::Duration::ZERO;

    #[inline]
    fn now() -> Self::InstantRepr {
        std::time::Instant::now()
    }

    #[inline]
    fn duration_since(lhs: Instant<Self>, rhs: Instant<Self>) -> Duration<Self> {
        if lhs.into_inner() >= rhs.into_inner() {
            Duration::from_inner(lhs.into_inner().duration_since(rhs.into_inner()))
        } else {
            Duration::from_inner(core::time::Duration::from_millis(0))
        }
    }

    #[inline]
    fn offset_from(lhs: Instant<Self>, rhs: Instant<Self>) -> SignedDuration<Self> {
        if lhs.into_inner() >= rhs.into_inner() {
            SignedDuration::from_inner(SignedStdDuration::Positive(
                lhs.into_inner().duration_since(rhs.into_inner()),
            ))
        } else {
            SignedDuration::from_inner(SignedStdDuration::Negative(
                rhs.into_inner().duration_since(lhs.into_inner()),
            ))
        }
    }

    #[inline]
    fn add_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner() + rhs.into_inner())
    }

    #[inline]
    fn sub_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner().checked_sub(rhs.into_inner()).unwrap())
    }

    #[inline]
    fn from_duration(duration: core::time::Duration) -> Self::DurationRepr {
        duration
    }

    #[inline]
    fn into_duration(repr: Self::DurationRepr) -> core::time::Duration {
        repr
    }

    #[inline]
    fn is_negative(repr: Self::SignedDurationRepr) -> bool {
        matches!(repr, SignedStdDuration::Negative(_))
    }
}
