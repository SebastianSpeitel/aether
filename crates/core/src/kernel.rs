use aether_time as time;
use core::error::Error;

/// Trait representing an execution context passed to tasks during polling.
pub trait Kernel {
    /// Registers that the task wants to be polled again
    fn yield_for<C: time::Clock>(&mut self, dur: time::Duration<C>);

    #[inline]
    fn r#yield(&mut self) {
        self.yield_for(time::Duration::<time::FrozenClock>::ZERO);
    }
}

#[allow(clippy::missing_errors_doc)]
pub trait Reactor {
    type Error: Error;
    type Operation;

    fn submit(&mut self, op: Self::Operation) -> Result<(), Self::Error>;
}
