pub use sys::sleep;

#[cfg(feature = "std")]
mod sys {
    use crate::{Clock, Duration};
    extern crate std;

    pub fn sleep<C: Clock>(dur: Duration<C>) {
        if dur <= Duration::from_millis(0u64) {
            return;
        }
        let std_dur = core::time::Duration::from(dur);
        std::thread::sleep(std_dur);
    }
}

#[cfg(all(
    not(feature = "std"),
    not(feature = "timer-clock"),
    target_arch = "avr"
))]
mod sys {
    use crate::native::SystemClock;
    use crate::{Clock, Duration};

    pub fn sleep<C: Clock>(dur: Duration<C>) {
        if dur <= Duration::from_millis(0u64) {
            return;
        }
        let ms = dur.as_millis() as u32;
        arduino_hal::delay_ms(ms);
        SystemClock::advance(ms);
    }
}

#[cfg(all(not(feature = "std"), feature = "timer-clock", target_arch = "avr"))]
mod sys {
    use crate::native::SystemClock;
    use crate::{Clock, Duration, Instant};

    pub fn sleep<C: Clock>(dur: Duration<C>) {
        if dur <= Duration::from_millis(0u64) {
            return;
        }

        let target = Instant::<C>::now() + dur;
        let dp = unsafe { arduino_hal::pac::Peripherals::steal() };
        let cpu = &dp.CPU;

        // Configure Sleep Mode to Idle: SE=1, SM=000
        cpu.smcr().write(|w| w.se().set_bit().sm().idle());

        while Instant::<C>::now().is_before(target) {
            avr_device::asm::sleep();
        }
    }
}
