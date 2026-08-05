#[cfg(not(target_arch = "avr"))]
pub mod arena;
#[cfg(feature = "alloc")]
pub mod global;
#[cfg(not(target_arch = "avr"))]
pub mod slab;

#[cfg(not(target_arch = "avr"))]
pub use arena::ArenaAllocator;
#[cfg(feature = "alloc")]
pub use global::Global;
#[cfg(not(target_arch = "avr"))]
pub use slab::SlabAllocator;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
pub type StdAllocator = Global<std::alloc::System>;
