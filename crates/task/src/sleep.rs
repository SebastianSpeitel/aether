use aether_core::Kernel;
use aether_core::time::{Clock, Instant};
use core::task::Poll;

use crate::task::Task;

pub use aether_core::time::sleep_async::{Sleep, sleep_async};

impl<K: Kernel, C: Clock> Task<K> for Sleep<C> {
    type Output = ();

    fn poll(&mut self, kernel: &mut K) -> Poll<Self::Output> {
        let now = Instant::now();
        if now.is_before(self.end_time) {
            let diff = self.end_time.duration_since(now);
            kernel.yield_for(diff);
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
