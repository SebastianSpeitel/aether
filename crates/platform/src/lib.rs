#![no_std]

pub mod allocator;
pub mod lock;
pub mod time;

pub use allocator::Global;
pub use allocator::{ArenaAllocator, SlabAllocator};
pub use lock::{Guard, Lock, Token};
pub use time::SystemClock;

pub mod prelude {
    pub use crate::allocator::Global;
    pub use crate::allocator::{ArenaAllocator, SlabAllocator};
    pub use crate::lock::{Lock, Token};
    pub use crate::time::SystemClock;
}
