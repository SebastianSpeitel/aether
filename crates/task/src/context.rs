use core::cell::Cell;
use core::task::Poll;

use aether_core::Kernel;
use aether_core::capability::HasClock;
use aether_core::clock::{Clock, Duration, Instant};

pub struct TaskContext<C: Clock> {
    pub clock: C,
    pub earliest_wake: Cell<Instant<C>>,
}

impl<C: Clock> TaskContext<C> {
    #[inline]
    pub const fn new(clock: C, earliest_wake: Instant<C>) -> Self {
        Self {
            clock,
            earliest_wake: Cell::new(earliest_wake),
        }
    }
}

impl<C: Clock> Kernel for TaskContext<C> {
    #[inline]
    fn r#yield<T>(&self) -> Poll<T> {
        self.earliest_wake.set(self.clock.now());
        Poll::Pending
    }
}

impl<C: Clock> HasClock<C> for TaskContext<C> {
    type Clock<'a>
        = &'a C
    where
        Self: 'a;

    #[inline]
    fn get_clock<'a>(&'a self) -> Self::Clock<'a> {
        &self.clock
    }

    #[inline]
    fn yield_for<T>(&self, dur: Duration<C>) -> Poll<T> {
        let now = self.clock.now();
        let target = C::add_duration(now, dur);
        let current = self.earliest_wake.get();
        let diff = C::offset_from(target, current);
        if diff.is_negative() {
            self.earliest_wake.set(target);
        }
        Poll::Pending
    }
}
