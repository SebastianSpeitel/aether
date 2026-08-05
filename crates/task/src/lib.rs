#![cfg_attr(not(feature = "std"), no_std)]

pub mod context;
pub mod executor;
#[cfg(feature = "extensions")]
pub mod extensions;
pub mod sleep;
#[cfg(feature = "extensions")]
mod task;

pub use context::TaskContext;
pub use executor::Executor;
pub use sleep::{Sleep, sleep_async};
#[cfg(feature = "extensions")]
pub use task::TaskExt;

pub mod prelude {
    pub use crate::executor::Executor;
    pub use crate::sleep::{Sleep, sleep_async};
    #[cfg(feature = "extensions")]
    pub use crate::task::TaskExt;
}
