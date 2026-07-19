pub mod arena;
pub mod global;
pub mod slab;

pub use arena::ArenaAllocator;
pub use global::Global;
pub use slab::SlabAllocator;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
pub type StdAllocator = Global<std::alloc::System>;
