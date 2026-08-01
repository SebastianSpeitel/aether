use core::error::Error;
use core::task::Poll;

use aether_time as time;

/// Trait representing an execution context passed to tasks during polling.
pub trait Kernel {
    /// Registers that the task wants to yield CPU time, returning `Poll::Pending`.
    #[inline]
    #[must_use = "yielding returns Poll::Pending and must be returned from task poll()"]
    fn r#yield<T>(&self) -> Poll<T> {
        self.yield_for::<time::FrozenClock, T>(time::Duration::<time::FrozenClock>::ZERO)
    }

    /// Registers that the task wants to yield CPU time for `dur`, returning `Poll::Pending`.
    #[must_use = "yielding returns Poll::Pending and must be returned from task poll()"]
    fn yield_for<C: time::Clock, T>(&self, dur: time::Duration<C>) -> Poll<T>;

    /// Notifies the kernel that an asynchronous event has occurred and tasks should be polled.
    ///
    /// Unlike `r#yield` (which is called during polling to yield CPU time), `wake` is typically
    /// called out-of-band by event producers or wakers.
    #[inline]
    fn wake(&self) {
        let _: Poll<()> = self.r#yield();
    }
}

/// Interface for async event reactors (epoll, io_uring, interrupts).
pub trait Reactor {
    type Error: Error;
    type Operation;

    /// Submits an operation to the reactor queue.
    ///
    /// # Errors
    /// Returns `Self::Error` if submitting the operation to the underlying reactor fails.
    fn submit(&mut self, op: Self::Operation) -> Result<(), Self::Error>;
}
