#![no_std]

pub use aether_core::clock::{Clock, Duration, HasClock, Instant, SignedDuration};
pub use aether_platform::clock::{FrozenClock, SystemClock};

#[cfg(feature = "std")]
pub use aether_platform::clock::StdClock;
