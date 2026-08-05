use aether_core::clock::{Clock, Duration, Instant, SignedDuration};

/// Standard library host system clock (`std::time::Instant`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StdClock;

impl Clock for StdClock {
    type InstantRepr = std::time::Instant;
    type DurationRepr = core::time::Duration;
    type SignedDurationRepr = (bool, core::time::Duration);

    const DURATION_ZERO: Self::DurationRepr = core::time::Duration::ZERO;

    #[inline]
    fn now(&self) -> Instant<Self> {
        Instant::from_inner(std::time::Instant::now())
    }

    #[inline]
    fn duration_since(lhs: Instant<Self>, rhs: Instant<Self>) -> Duration<Self> {
        Duration::from_inner(lhs.into_inner().duration_since(rhs.into_inner()))
    }

    #[inline]
    fn offset_from(lhs: Instant<Self>, rhs: Instant<Self>) -> SignedDuration<Self> {
        let l = lhs.into_inner();
        let r = rhs.into_inner();
        if l >= r {
            SignedDuration::from_inner((false, l.duration_since(r)))
        } else {
            SignedDuration::from_inner((true, r.duration_since(l)))
        }
    }

    #[inline]
    fn add_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner() + rhs.into_inner())
    }

    #[inline]
    fn sub_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner() - rhs.into_inner())
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
        repr.0 && repr.1 > core::time::Duration::ZERO
    }
}
