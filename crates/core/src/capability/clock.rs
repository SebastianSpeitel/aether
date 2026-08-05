use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::task::Poll;

/// Top-level capability trait for hardware clocks and time sources.
pub trait Clock {
    type InstantRepr: Copy + core::fmt::Debug;
    type DurationRepr: Copy + core::fmt::Debug;
    type SignedDurationRepr: Copy + core::fmt::Debug;

    const DURATION_ZERO: Self::DurationRepr;

    /// Reads the current time instant from this clock.
    fn now(&self) -> Instant<Self>;

    /// Calculates duration between two instants produced by this clock.
    fn duration_since(lhs: Instant<Self>, rhs: Instant<Self>) -> Duration<Self>;

    /// Calculates signed offset between two instants produced by this clock.
    fn offset_from(lhs: Instant<Self>, rhs: Instant<Self>) -> SignedDuration<Self>;

    /// Adds duration to an instant.
    fn add_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self>;

    /// Subtracts duration from an instant.
    fn sub_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self>;

    /// Converts standard `core::time::Duration` to `Self::DurationRepr`.
    fn from_duration(duration: core::time::Duration) -> Self::DurationRepr;

    /// Converts `Self::DurationRepr` to standard `core::time::Duration`.
    fn into_duration(repr: Self::DurationRepr) -> core::time::Duration;

    /// Checks if a signed duration representation is negative.
    fn is_negative(repr: Self::SignedDurationRepr) -> bool;

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
}

#[repr(transparent)]
pub struct Instant<C: Clock + ?Sized> {
    repr: C::InstantRepr,
}

impl<C: Clock + ?Sized> Clone for Instant<C> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Clock + ?Sized> Copy for Instant<C> {}

impl<C: Clock + ?Sized> core::fmt::Debug for Instant<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Instant").field(&self.repr).finish()
    }
}

impl<C: Clock + ?Sized> PartialEq for Instant<C>
where
    C::InstantRepr: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.repr.eq(&other.repr)
    }
}

impl<C: Clock + ?Sized> Eq for Instant<C> where C::InstantRepr: Eq {}

impl<C: Clock + ?Sized> PartialOrd for Instant<C>
where
    C::InstantRepr: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.repr.partial_cmp(&other.repr)
    }
}

impl<C: Clock + ?Sized> Ord for Instant<C>
where
    C::InstantRepr: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.repr.cmp(&other.repr)
    }
}

impl<C: Clock + ?Sized> Instant<C> {
    #[inline]
    #[must_use]
    pub const fn from_inner(repr: C::InstantRepr) -> Self {
        Self { repr }
    }

    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> C::InstantRepr {
        self.repr
    }

    #[inline]
    #[must_use]
    pub fn duration_since(self, earlier: Self) -> Duration<C> {
        C::duration_since(self, earlier)
    }

    #[inline]
    #[must_use]
    pub fn offset_from(self, target: Self) -> SignedDuration<C> {
        C::offset_from(self, target)
    }

    #[inline]
    #[must_use]
    pub fn is_before(self, other: Self) -> bool {
        C::is_negative(self.offset_from(other).repr)
    }
}

impl<C: Clock + ?Sized> Instant<C> {
    #[inline]
    #[must_use]
    pub fn now(clock: &C) -> Self {
        clock.now()
    }
}

#[repr(transparent)]
pub struct Duration<C: Clock + ?Sized> {
    repr: C::DurationRepr,
}

impl<C: Clock + ?Sized> Clone for Duration<C> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Clock + ?Sized> Copy for Duration<C> {}

impl<C: Clock + ?Sized> core::fmt::Debug for Duration<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("Duration").field(&self.repr).finish()
    }
}

impl<C: Clock + ?Sized> PartialEq for Duration<C>
where
    C::DurationRepr: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.repr.eq(&other.repr)
    }
}

impl<C: Clock + ?Sized> Eq for Duration<C> where C::DurationRepr: Eq {}

impl<C: Clock + ?Sized> PartialOrd for Duration<C>
where
    C::DurationRepr: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.repr.partial_cmp(&other.repr)
    }
}

impl<C: Clock + ?Sized> Ord for Duration<C>
where
    C::DurationRepr: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.repr.cmp(&other.repr)
    }
}

impl<C: Clock + ?Sized> Duration<C> {
    pub const ZERO: Self = Self::from_inner(C::DURATION_ZERO);

    #[inline]
    #[must_use]
    pub const fn from_inner(repr: C::DurationRepr) -> Self {
        Self { repr }
    }

    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> C::DurationRepr {
        self.repr
    }

    #[inline]
    #[must_use]
    pub fn from_millis(ms: u64) -> Self
    where
        C: Sized,
    {
        Self {
            repr: C::from_duration(core::time::Duration::from_millis(ms)),
        }
    }

    #[inline]
    #[must_use]
    pub fn as_millis(&self) -> u128
    where
        C: Sized,
    {
        C::into_duration(self.repr).as_millis()
    }
}

impl<C: Clock + ?Sized> From<core::time::Duration> for Duration<C> {
    #[inline]
    fn from(duration: core::time::Duration) -> Self {
        Self {
            repr: C::from_duration(duration),
        }
    }
}

impl<C: Clock + ?Sized> From<Duration<C>> for core::time::Duration {
    #[inline]
    fn from(d: Duration<C>) -> Self {
        C::into_duration(d.repr)
    }
}

/// Strongly-typed signed duration representation bound to clock `C`.
#[repr(transparent)]
pub struct SignedDuration<C: Clock + ?Sized> {
    repr: C::SignedDurationRepr,
}

impl<C: Clock + ?Sized> Clone for SignedDuration<C> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Clock + ?Sized> Copy for SignedDuration<C> {}

impl<C: Clock + ?Sized> core::fmt::Debug for SignedDuration<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SignedDuration").field(&self.repr).finish()
    }
}

impl<C: Clock + ?Sized> PartialEq for SignedDuration<C>
where
    C::SignedDurationRepr: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.repr.eq(&other.repr)
    }
}

impl<C: Clock + ?Sized> Eq for SignedDuration<C> where C::SignedDurationRepr: Eq {}

impl<C: Clock + ?Sized> PartialOrd for SignedDuration<C>
where
    C::SignedDurationRepr: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.repr.partial_cmp(&other.repr)
    }
}

impl<C: Clock + ?Sized> Ord for SignedDuration<C>
where
    C::SignedDurationRepr: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.repr.cmp(&other.repr)
    }
}

impl<C: Clock + ?Sized> SignedDuration<C> {
    #[inline]
    pub const fn from_inner(repr: C::SignedDurationRepr) -> Self {
        Self { repr }
    }

    #[inline]
    pub const fn into_inner(self) -> C::SignedDurationRepr {
        self.repr
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        C::is_negative(self.repr)
    }
}

impl<C: Clock + ?Sized> Add<Duration<C>> for Instant<C> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Duration<C>) -> Self {
        C::add_duration(self, rhs)
    }
}

impl<C: Clock + ?Sized> Sub<Duration<C>> for Instant<C> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Duration<C>) -> Self {
        C::sub_duration(self, rhs)
    }
}

impl<C: Clock + ?Sized> Add for Duration<C> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        C::add_durations(self, rhs)
    }
}

impl<C: Clock + ?Sized> Sub for Duration<C> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        C::sub_durations(self, rhs)
    }
}

impl<C: Clock + ?Sized> AddAssign for Duration<C> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<C: Clock + ?Sized> SubAssign for Duration<C> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<C: Clock + ?Sized> Default for Instant<C>
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

impl<C: Clock + ?Sized> Default for Duration<C> {
    #[inline]
    fn default() -> Self {
        Self {
            repr: C::DURATION_ZERO,
        }
    }
}

impl<C: Clock + ?Sized> Default for SignedDuration<C>
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

/// First-class kernel capability trait for accessing clock `C`.
pub trait HasClock<C: Clock + ?Sized>: crate::Kernel {
    type Clock<'a>: core::ops::Deref<Target = C> + 'a
    where
        Self: 'a,
        C: 'a;

    /// Gets a reference to the clock capability instance `C`.
    fn get_clock<'a>(&'a self) -> Self::Clock<'a>;

    /// Yields CPU execution for duration `dur` on clock `C`.
    #[must_use = "yielding for a duration returns Poll::Pending and must be returned from poll()"]
    fn yield_for<T>(&self, dur: Duration<C>) -> Poll<T> {
        let _ = dur;
        self.r#yield()
    }
}
