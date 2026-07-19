#![no_std]

use core::ops::{Add, AddAssign, Sub, SubAssign};

pub mod frozen;
pub mod sleep;
pub mod sleep_async;

#[cfg(feature = "std")]
pub mod std_clock;

pub use frozen::FrozenClock;
pub use sleep::sleep;
pub use sleep_async::{Sleep, sleep_async};

#[cfg(feature = "std")]
pub use std_clock::StdClock;

pub trait Clock: Copy + Default + PartialEq + PartialOrd + Eq + Ord {
    type InstantRepr: Copy + Ord + Eq + core::fmt::Debug;
    type DurationRepr: Copy + Ord + Eq + core::fmt::Debug;
    type SignedDurationRepr: Copy + Ord + Eq + core::fmt::Debug;

    const DURATION_ZERO: Self::DurationRepr;

    fn now() -> Self::InstantRepr;
    fn duration_since(lhs: Instant<Self>, rhs: Instant<Self>) -> Duration<Self>;
    fn offset_from(lhs: Instant<Self>, rhs: Instant<Self>) -> SignedDuration<Self>;
    fn add_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self>;
    fn sub_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self>;

    #[inline]
    fn add_durations(lhs: Duration<Self>, rhs: Duration<Self>) -> Duration<Self> {
        let lhs_std = Self::into_duration(lhs.into_inner());
        let rhs_std = Self::into_duration(rhs.into_inner());
        Duration::from_inner(Self::from_duration(lhs_std + rhs_std))
    }

    #[inline]
    fn sub_durations(lhs: Duration<Self>, rhs: Duration<Self>) -> Duration<Self> {
        let lhs_std = Self::into_duration(lhs.into_inner());
        let rhs_std = Self::into_duration(rhs.into_inner());
        Duration::from_inner(Self::from_duration(lhs_std.saturating_sub(rhs_std)))
    }

    fn from_duration(duration: core::time::Duration) -> Self::DurationRepr;
    fn into_duration(repr: Self::DurationRepr) -> core::time::Duration;
    fn is_negative(repr: Self::SignedDurationRepr) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Instant<C: Clock = FrozenClock> {
    repr: C::InstantRepr,
}

impl<C: Clock> Instant<C> {
    #[inline]
    pub const fn from_inner(repr: C::InstantRepr) -> Self {
        Self { repr }
    }

    #[inline]
    pub const fn into_inner(self) -> C::InstantRepr {
        self.repr
    }
}

impl<C: Clock> Default for Instant<C>
where
    C::InstantRepr: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            repr: C::InstantRepr::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Duration<C: Clock = FrozenClock> {
    repr: C::DurationRepr,
}

impl<C: Clock> Duration<C> {
    pub const ZERO: Self = Self::from_inner(C::DURATION_ZERO);

    #[inline]
    pub const fn from_inner(repr: C::DurationRepr) -> Self {
        Self { repr }
    }

    #[inline]
    pub const fn into_inner(self) -> C::DurationRepr {
        self.repr
    }
}

impl<C: Clock> Default for Duration<C>
where
    C::DurationRepr: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            repr: C::DurationRepr::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SignedDuration<C: Clock = FrozenClock> {
    repr: C::SignedDurationRepr,
}

impl<C: Clock> SignedDuration<C> {
    #[inline]
    pub const fn from_inner(repr: C::SignedDurationRepr) -> Self {
        Self { repr }
    }

    #[inline]
    #[allow(dead_code)]
    pub const fn into_inner(self) -> C::SignedDurationRepr {
        self.repr
    }
}

impl<C: Clock> Default for SignedDuration<C>
where
    C::SignedDurationRepr: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            repr: C::SignedDurationRepr::default(),
        }
    }
}

impl<C: Clock> Instant<C> {
    #[inline]
    #[must_use]
    pub fn now() -> Self {
        Self { repr: C::now() }
    }

    #[inline]
    pub fn duration_since(self, earlier: Self) -> Duration<C> {
        C::duration_since(self, earlier)
    }

    #[inline]
    pub fn offset_from(self, target: Self) -> SignedDuration<C> {
        C::offset_from(self, target)
    }

    #[inline]
    pub fn is_before(self, other: Self) -> bool {
        C::is_negative(self.offset_from(other).repr)
    }
}

impl<C: Clock> PartialOrd for Instant<C> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: Clock> Ord for Instant<C> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let diff = self.offset_from(*other);
        if C::is_negative(diff.repr) {
            core::cmp::Ordering::Less
        } else if C::is_negative(other.offset_from(*self).repr) {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl<C: Clock> Duration<C> {
    #[inline]
    #[must_use]
    pub fn from_millis(ms: u64) -> Self {
        Self {
            repr: C::from_duration(core::time::Duration::from_millis(ms)),
        }
    }

    #[inline]
    pub fn as_millis(&self) -> u128 {
        C::into_duration(self.repr).as_millis()
    }
}

impl<C: Clock> From<core::time::Duration> for Duration<C> {
    #[inline]
    fn from(duration: core::time::Duration) -> Self {
        Self {
            repr: C::from_duration(duration),
        }
    }
}

impl<C: Clock> From<Duration<C>> for core::time::Duration {
    #[inline]
    fn from(d: Duration<C>) -> Self {
        C::into_duration(d.repr)
    }
}

impl<C: Clock> Add<Duration<C>> for Instant<C> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Duration<C>) -> Self {
        C::add_duration(self, rhs)
    }
}

impl<C: Clock> Sub<Duration<C>> for Instant<C> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Duration<C>) -> Self {
        C::sub_duration(self, rhs)
    }
}

impl<C: Clock> Add for Duration<C> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        C::add_durations(self, rhs)
    }
}

impl<C: Clock> Sub for Duration<C> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        C::sub_durations(self, rhs)
    }
}

impl<C: Clock> AddAssign for Duration<C> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<C: Clock> SubAssign for Duration<C> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
