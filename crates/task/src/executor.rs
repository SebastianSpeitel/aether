use aether_core::time::{Clock, Duration, Instant};
use core::task::Poll;

use crate::context::TaskContext;
use crate::task::Task;

/// A uniform cooperative executor.
pub struct Executor<T, C: Clock> {
    task: T,
    max_sleep: Duration<C>,
}

impl<T: Task<TaskContext<C>>, C: Clock> Executor<T, C> {
    pub const fn new(task: T, max_sleep: Duration<C>) -> Self {
        Self { task, max_sleep }
    }

    /// Runs the executor loop until the task finishes.
    /// Calls the provided `sleep_fn` to yield execution when idle.
    pub fn run<S>(&mut self, mut sleep_fn: S) -> T::Output
    where
        S: FnMut(Duration<C>),
    {
        loop {
            let now = Instant::<C>::now();
            let mut cx = TaskContext::new(now + self.max_sleep);

            if let Poll::Ready(out) = self.task.poll(&mut cx) {
                return out;
            }

            if now.is_before(cx.earliest_wake) {
                let diff = cx.earliest_wake.duration_since(now);
                sleep_fn(core::cmp::min(diff, self.max_sleep));
            }
        }
    }
}
