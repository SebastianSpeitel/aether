use core::cell::Cell;

use aether_core::Kernel;
use aether_core::time::{Clock, Duration, Instant};

pub struct TaskContext<C: Clock> {
    pub earliest_wake: Cell<Instant<C>>,
}

impl<C: Clock> TaskContext<C> {
    #[inline]
    pub const fn new(earliest_wake: Instant<C>) -> Self {
        Self {
            earliest_wake: Cell::new(earliest_wake),
        }
    }
}

impl<C: Clock> Kernel for TaskContext<C> {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn yield_for<CLK: Clock, T>(&self, dur: Duration<CLK>) -> core::task::Poll<T> {
        let dur_ms = dur.as_millis() as u64;
        self.earliest_wake.set(core::cmp::min(
            self.earliest_wake.get(),
            Instant::<C>::now() + Duration::<C>::from_millis(dur_ms),
        ));
        core::task::Poll::Pending
    }
    #[inline]
    fn r#yield<T>(&self) -> core::task::Poll<T> {
        self.earliest_wake.set(Instant::<C>::now());
        core::task::Poll::Pending
    }
}
