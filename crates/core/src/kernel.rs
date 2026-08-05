use core::error::Error;
use core::task::Poll;

/// Trait representing an execution context passed to tasks during polling.
pub trait Kernel {
    /// Registers that the task wants to yield CPU time, returning `Poll::Pending`.
    #[inline]
    #[must_use = "yielding returns Poll::Pending and must be returned from task poll()"]
    fn r#yield<T>(&self) -> Poll<T> {
        Poll::Pending
    }

    /// Notifies the kernel that an asynchronous event has occurred and tasks should be polled.
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
