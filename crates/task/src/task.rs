use aether_core::Kernel;
use aether_core::clock::{Clock, Duration};

use crate::extensions::{Map, RepeatN, RepeatWith, Then, ThenState, Timeout};

use aether_core::task::Task;

/// Extension trait for `Task` to support combinators.
pub trait TaskExt<K: Kernel>: Task<K> {
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
    fn then<F, T2>(self, f: F) -> Then<Self, F, T2>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> T2,
        T2: Task<K>,
    {
        Then {
            state: ThenState::First(self, f),
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

    #[inline]
    fn repeat_n<F>(self, count: u8, factory: F) -> RepeatN<F, Self>
    where
        Self: Sized,
        F: FnMut() -> Self,
    {
        RepeatN {
            factory,
            task: self,
            remaining: count,
        }
    }
}

#[cfg(feature = "extensions")]
impl<K: Kernel, T: Task<K> + ?Sized> TaskExt<K> for T {}
