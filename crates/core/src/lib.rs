#![no_std]

pub mod builder;
pub mod capability;
pub mod kernel;
pub mod scheduler;
pub mod task;

pub use builder::{CompositeKernel, KernelBuilder};
pub use capability::allocator::{self, Allocator, HasAllocator};
pub use capability::clock::{self, Clock, Duration, HasClock, Instant, SignedDuration};
pub use capability::driver::{
    self, BlockDriver, CloneDriver, Driver, HasDriver, IoctlDriver, PositionedReadDriver,
    PositionedWriteDriver, ReadDriver, WriteDriver,
};
pub use capability::guard::{self, Guard, GuardMut};
pub use capability::token::{self, Token, TokenAssumeInit};
pub use kernel::Kernel;
pub use task::Task;

pub mod prelude {
    pub use crate::capability::allocator::{Allocator, HasAllocator};
    pub use crate::capability::driver::{Driver, HasDriver};
    pub use crate::capability::guard::{Guard, GuardMut};
    pub use crate::capability::token::{Token, TokenAssumeInit};
    pub use crate::kernel::Kernel;
    pub use crate::task::Task;
}
