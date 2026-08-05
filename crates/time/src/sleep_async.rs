use aether_core::clock::{Clock, Duration};
pub use aether_core::task::Sleep;

/// Helper function to create a cooperative Sleep task.
#[inline]
pub const fn sleep_async<C: Clock>(duration: Duration<C>) -> Sleep<C> {
    Sleep::new(duration)
}
