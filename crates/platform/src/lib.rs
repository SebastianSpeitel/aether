#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(target_arch = "avr", feature(asm_experimental_arch, abi_avr_interrupt))]

pub mod allocator;
pub mod clock;
pub mod driver;
pub mod lock;
pub mod progmem;
pub mod sleep;

#[cfg(feature = "alloc")]
pub use allocator::Global;
#[cfg(not(target_arch = "avr"))]
pub use allocator::{ArenaAllocator, SlabAllocator};
#[cfg(feature = "std")]
pub use clock::StdClock;
#[cfg(target_arch = "avr")]
pub use clock::SystemClock;
#[cfg(feature = "embedded")]
pub use driver::PinDriver;
pub use lock::{Guard, Lock, Token};
pub use progmem::{PStr, ProgPtr};
