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
    fn yield_for<CLK: Clock>(&self, dur: Duration<CLK>) {
        let dur_ms = dur.as_millis() as u64;
        self.earliest_wake.set(core::cmp::min(
            self.earliest_wake.get(),
            Instant::<C>::now() + Duration::<C>::from_millis(dur_ms),
        ));
    }
    #[inline]
    fn r#yield(&self) {
        self.earliest_wake.set(Instant::<C>::now());
    }
}
