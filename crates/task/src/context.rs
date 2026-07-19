use aether_core::Kernel;
use aether_core::time::{Clock, Duration, Instant};

pub struct TaskContext<C: Clock> {
    pub earliest_wake: Instant<C>,
}

impl<C: Clock> TaskContext<C> {
    #[inline]
    pub const fn new(earliest_wake: Instant<C>) -> Self {
        Self { earliest_wake }
    }
}

impl<C: Clock> Kernel for TaskContext<C> {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn yield_for<CLK: Clock>(&mut self, dur: Duration<CLK>) {
        let dur_ms = dur.as_millis() as u64;
        self.earliest_wake = core::cmp::min(
            self.earliest_wake,
            Instant::<C>::now() + Duration::<C>::from_millis(dur_ms),
        );
    }
    #[inline]
    fn r#yield(&mut self) {
        self.earliest_wake = Instant::<C>::now();
    }
}
