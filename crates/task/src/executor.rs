use aether_core::Task;
use aether_core::clock::{Clock, Duration};
use core::task::Poll;

use crate::context::TaskContext;

/// A uniform cooperative executor.
pub struct Executor<T, C: Clock> {
    task: T,
    clock: C,
    max_sleep: Duration<C>,
}

impl<T: Task<TaskContext<C>>, C: Clock + Copy> Executor<T, C> {
    pub const fn new(task: T, clock: C, max_sleep: Duration<C>) -> Self {
        Self {
            task,
            clock,
            max_sleep,
        }
    }

    /// Runs the executor loop until the task finishes.
    /// Calls the provided `sleep_fn` to yield execution when idle.
    pub fn run<S>(&mut self, mut sleep_fn: S) -> T::Output
    where
        S: FnMut(Duration<C>),
    {
        loop {
            let now = self.clock.now();
            let target = C::add_duration(now, self.max_sleep);
            let cx = TaskContext::new(self.clock, target);

            if let Poll::Ready(out) = self.task.poll(&cx) {
                return out;
            }

            let earliest = cx.earliest_wake.get();
            let diff = C::offset_from(earliest, now);
            if !diff.is_negative() {
                let dur = C::duration_since(earliest, now);
                let dur_diff = C::offset_from(
                    C::add_duration(now, dur),
                    C::add_duration(now, self.max_sleep),
                );
                let sleep_dur = if dur_diff.is_negative() {
                    dur
                } else {
                    self.max_sleep
                };
                sleep_fn(sleep_dur);
            }
        }
    }
}
