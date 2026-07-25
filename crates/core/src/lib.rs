#![no_std]

pub use aether_time as time;

pub mod allocator;
pub mod guard;
pub mod kernel;
pub mod scheduler;
pub mod task;
pub mod token;

pub use allocator::Allocator;
pub use guard::{Guard, GuardMut};
pub use kernel::Kernel;
pub use task::Task;
pub use token::{Token, TokenAssumeInit};

pub mod prelude {
    pub use crate::allocator::Allocator;
    pub use crate::guard::{Guard, GuardMut};
    pub use crate::kernel::Kernel;
    pub use crate::task::Task;
    pub use crate::token::{Token, TokenAssumeInit};
}
