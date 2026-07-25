use core::error::Error;

use aether_time as time;

/// Trait representing an execution context passed to tasks during polling.
pub trait Kernel {
    #[inline]
    fn r#yield(&self) {
        self.yield_for(time::Duration::<time::FrozenClock>::ZERO);
    }

    /// Registers that the task wants to be polled again
    fn yield_for<C: time::Clock>(&self, dur: time::Duration<C>);

    /// Notifies the kernel that an asynchronous event has occurred and tasks should be polled.
    ///
    /// Unlike `r#yield` (which is called during polling to yield CPU time), `wake` is typically
    /// called out-of-band by event producers or wakers.
    #[inline]
    fn wake(&self) {
        self.r#yield();
    }
}

#[allow(clippy::missing_errors_doc)]
pub trait Reactor {
    type Error: Error;
    type Operation;

    fn submit(&mut self, op: Self::Operation) -> Result<(), Self::Error>;
}
