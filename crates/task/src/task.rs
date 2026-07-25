use aether_core::Kernel;
use aether_core::time::{Clock, Duration};

use crate::combinators::{Map, RepeatWith, Timeout};

pub use aether_core::task::Task;

/// Extension trait for `Task` to support combinators.
pub trait TaskExt<K: Kernel + ?Sized>: Task<K> {
    #[inline]
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> B,
    {
        Map { task: self, f }
    }

    #[inline]
    fn timeout<C: Clock>(self, limit: Duration<C>) -> Timeout<Self, C>
    where
        Self: Sized,
    {
        Timeout {
            task: self,
            start_time: None,
            limit,
        }
    }

    #[inline]
    fn repeat_with<F>(self, factory: F) -> RepeatWith<F, Self>
    where
        Self: Sized,
        F: FnMut() -> Self,
    {
        RepeatWith {
            factory,
            task: self,
        }
    }
}

impl<K: Kernel + ?Sized, T: Task<K> + ?Sized> TaskExt<K> for T {}
