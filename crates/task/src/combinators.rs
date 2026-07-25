use aether_core::Kernel;
use aether_core::time::{Clock, Duration, Instant};
use core::task::Poll;

use crate::task::Task;

pub struct RepeatWith<F, T> {
    pub(crate) factory: F,
    pub(crate) task: T,
}

impl<K: Kernel, F, T> Task<K> for RepeatWith<F, T>
where
    F: FnMut() -> T,
    T: Task<K>,
{
    type Output = T::Output;

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        match self.task.poll(kern) {
            Poll::Ready(out) => {
                self.task = (self.factory)();
                Poll::Ready(out)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct ResettableTimeout<const D: usize, F, C: Clock> {
    start: Option<Instant<C>>,
    action: F,
}

impl<const D: usize, F, C: Clock> ResettableTimeout<D, F, C> {
    pub const fn new(action: F) -> Self {
        Self {
            start: None,
            action,
        }
    }
    pub fn start(&mut self, now: Instant<C>) {
        self.start.get_or_insert(now);
    }
    pub const fn cancel(&mut self) {
        self.start = None;
    }
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.start.is_some()
    }
    pub fn poll<A>(&mut self, now: Instant<C>, args: A)
    where
        F: FnMut(A),
    {
        if self
            .start
            .is_some_and(|s| now.duration_since(s) >= Duration::<C>::from_millis(D as u64))
        {
            self.start = None;
            (self.action)(args);
        }
    }
}

pub struct Map<T, F> {
    pub(crate) task: T,
    pub(crate) f: F,
}

impl<K: Kernel, T, F, B> Task<K> for Map<T, F>
where
    T: Task<K>,
    F: FnMut(T::Output) -> B,
{
    type Output = B;

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        match self.task.poll(kern) {
            Poll::Ready(out) => Poll::Ready((self.f)(out)),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Timeout<T, C: Clock> {
    pub(crate) task: T,
    pub(crate) start_time: Option<Instant<C>>,
    pub(crate) limit: Duration<C>,
}

impl<K: Kernel, T: Task<K>, C: Clock> Task<K> for Timeout<T, C> {
    type Output = Result<T::Output, ()>;

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        let now = Instant::<C>::now();
        if self.start_time.is_none() {
            self.start_time = Some(now);
        }

        match self.task.poll(kern) {
            Poll::Ready(out) => {
                self.start_time = None;
                Poll::Ready(Ok(out))
            }
            Poll::Pending => {
                if self
                    .start_time
                    .is_some_and(|start| now.duration_since(start) >= self.limit)
                {
                    self.start_time = None;
                    return Poll::Ready(Err(()));
                }
                Poll::Pending
            }
        }
    }
}
