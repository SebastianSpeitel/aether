use crate::lock::{Lock, Token};
use aether_time::{Clock, Duration, Instant, SignedDuration};

static CACHED_NOW: Lock<u32, 1> = Lock::new(0);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct SystemClock;

impl SystemClock {
    #[inline]
    pub fn tick() {
        Self::advance(1);
    }

    #[inline]
    pub fn advance(ms: u32) {
        let mut ctx = unsafe { Token::<1>::new_unchecked() };
        let guard = CACHED_NOW.get_mut(&mut ctx);
        *guard = guard.wrapping_add(ms);
    }

    #[cfg(all(target_arch = "avr", feature = "timer-clock"))]
    pub fn init(tc2: &arduino_hal::pac::TC2) {
        avr_device::interrupt::disable();

        tc2.tccr2a().write(|w| unsafe { w.bits(0x02) });
        tc2.ocr2a().write(|w| unsafe { w.bits(249) });
        tc2.timsk2().write(|w| unsafe { w.bits(0x02) });
        tc2.tccr2b().write(|w| unsafe { w.bits(0x04) });

        unsafe { avr_device::interrupt::enable() };
    }
}

#[cfg(all(target_arch = "avr", feature = "timer-clock"))]
#[avr_device::interrupt(atmega328p)]
fn TIMER2_COMPA() {
    SystemClock::tick();
}

impl Clock for SystemClock {
    type InstantRepr = u32;
    type DurationRepr = u32;
    type SignedDurationRepr = i32;

    const DURATION_ZERO: Self::DurationRepr = 0;

    #[inline]
    fn now() -> Self::InstantRepr {
        CACHED_NOW.read()
    }

    #[inline]
    fn duration_since(lhs: Instant<Self>, rhs: Instant<Self>) -> Duration<Self> {
        Duration::from_inner(lhs.into_inner().wrapping_sub(rhs.into_inner()))
    }

    #[inline]
    fn offset_from(lhs: Instant<Self>, rhs: Instant<Self>) -> SignedDuration<Self> {
        let diff = lhs.into_inner().wrapping_sub(rhs.into_inner());
        SignedDuration::from_inner(i32::from_ne_bytes(diff.to_ne_bytes()))
    }

    #[inline]
    fn add_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner().wrapping_add(rhs.into_inner()))
    }

    #[inline]
    fn sub_duration(lhs: Instant<Self>, rhs: Duration<Self>) -> Instant<Self> {
        Instant::from_inner(lhs.into_inner().wrapping_sub(rhs.into_inner()))
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn from_duration(duration: core::time::Duration) -> Self::DurationRepr {
        duration.as_millis() as u32
    }

    #[inline]
    fn into_duration(repr: Self::DurationRepr) -> core::time::Duration {
        core::time::Duration::from_millis(repr.into())
    }

    #[inline]
    fn is_negative(repr: Self::SignedDurationRepr) -> bool {
        repr < 0
    }
}
