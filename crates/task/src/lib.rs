#![no_std]

pub mod combinators;
pub mod context;
pub mod executor;
pub mod sleep;
pub mod task;

pub use combinators::{Map, RepeatWith, ResettableTimeout, Timeout};
pub use context::TaskContext;
pub use executor::Executor;
pub use sleep::{Sleep, sleep_async};
pub use task::{Task, TaskExt};

pub mod prelude {
    pub use crate::executor::Executor;
    pub use crate::sleep::{Sleep, sleep_async};
    pub use crate::task::{Task, TaskExt};
}
