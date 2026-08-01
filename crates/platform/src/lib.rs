#![no_std]

pub mod allocator;
pub mod driver;
pub mod lock;
pub mod progmem;
pub mod time;

#[cfg(feature = "alloc")]
pub use allocator::Global;
pub use allocator::{ArenaAllocator, SlabAllocator};
#[cfg(feature = "embedded")]
pub use driver::PinDriver;
pub use lock::{Guard, Lock, Token};
pub use progmem::{read_byte, PStr, ProgPtr};
pub use time::SystemClock;

pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use crate::allocator::Global;
    pub use crate::allocator::{ArenaAllocator, SlabAllocator};
    #[cfg(feature = "embedded")]
    pub use crate::driver::PinDriver;
    pub use crate::lock::{Lock, Token};
    pub use crate::progmem::{read_byte, PStr, ProgPtr};
    pub use crate::time::SystemClock;
}
