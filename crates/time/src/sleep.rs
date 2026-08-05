pub use sys::sleep;

#[cfg(feature = "std")]
mod sys {
    use aether_core::clock::{Clock, Duration};
    extern crate std;

    pub fn sleep<C: Clock>(dur: Duration<C>, _clock: &C) {
        let std_dur = C::into_duration(dur.into_inner());
        if std_dur > core::time::Duration::ZERO {
            std::thread::sleep(std_dur);
        }
    }
}

#[cfg(all(
    not(feature = "std"),
    not(feature = "timer-clock"),
    target_arch = "avr"
))]
mod sys {
    use aether_core::clock::{Clock, Duration};
    use aether_platform::clock::SystemClock;

    pub fn sleep<C: Clock>(dur: Duration<C>, _clock: &C) {
        let ms = dur.as_millis() as u32;
        if ms > 0 {
            arduino_hal::delay_ms(ms);
            SystemClock::advance(ms);
        }
    }
}

#[cfg(all(not(feature = "std"), feature = "timer-clock", target_arch = "avr"))]
mod sys {
    use aether_core::clock::{Clock, Duration};

    pub fn sleep<C: Clock>(dur: Duration<C>, clock: &C) {
        let now = clock.now();
        let target = C::add_duration(now, dur);
        let dp = unsafe { arduino_hal::pac::Peripherals::steal() };
        let cpu = &dp.CPU;

        // Configure Sleep Mode to Idle: SE=1, SM=000
        cpu.smcr().write(|w| w.se().set_bit().sm().idle());

        loop {
            let current = clock.now();
            let diff = C::offset_from(current, target);
            if !diff.is_negative() {
                break;
            }
            avr_device::asm::sleep();
        }
    }
}

#[cfg(all(not(feature = "std"), not(target_arch = "avr")))]
mod sys {
    use aether_core::clock::{Clock, Duration};

    pub fn sleep<C: Clock>(_dur: Duration<C>, _clock: &C) {}
}
