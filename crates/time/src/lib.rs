#![no_std]

pub mod sleep;
pub mod sleep_async;

pub use aether_core::clock::{Clock, Duration, HasClock, Instant, SignedDuration};
pub use aether_platform::clock::{FrozenClock, SystemClock};

#[cfg(feature = "std")]
pub use aether_platform::clock::StdClock;

pub use sleep::sleep;
pub use sleep_async::{Sleep, sleep_async};
