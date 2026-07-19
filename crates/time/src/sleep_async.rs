use super::{Clock, Duration, Instant};

/// A cooperative delay task that completes after a fixed duration.
#[derive(Debug, Clone, Copy)]
pub struct Sleep<C: Clock> {
    pub end_time: Instant<C>,
}

impl<C: Clock> Sleep<C> {
    pub fn after(duration: Duration<C>) -> Self {
        Self {
            end_time: Instant::now() + duration,
        }
    }
}

/// Helper function to create a cooperative Sleep task.
pub fn sleep_async<C: Clock>(duration: Duration<C>) -> Sleep<C> {
    Sleep::after(duration)
}
