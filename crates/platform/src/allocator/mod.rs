pub mod arena;
#[cfg(feature = "alloc")]
pub mod global;
pub mod slab;

pub use arena::ArenaAllocator;
#[cfg(feature = "alloc")]
pub use global::Global;
pub use slab::SlabAllocator;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
pub type StdAllocator = Global<std::alloc::System>;
