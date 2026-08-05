use core::task::Poll;

use aether_core::{
    HasClock, Instant, Kernel, Task,
    clock::{Clock, Duration},
};

/// An asynchronous sleep task for duration `dur` using clock capability `C`.
#[derive(Debug, Clone, Copy)]
pub struct Sleep<C: Clock> {
    end_time: Option<Instant<C>>,
    duration: Duration<C>,
}

impl<C: Clock> Sleep<C> {
    #[inline]
    pub const fn new(dur: Duration<C>) -> Self {
        Self {
            end_time: None,
            duration: dur,
        }
    }
}

impl<K, C: Clock> Task<K> for Sleep<C>
where
    K: Kernel + HasClock<C>,
{
    type Output = ();

    fn poll(&mut self, kernel: &K) -> Poll<Self::Output> {
        let clock = kernel.get_clock();
        let now = clock.now();
        let end = if let Some(e) = self.end_time {
            e
        } else {
            let target = C::add_duration(now, self.duration);
            self.end_time = Some(target);
            target
        };

        let diff = C::offset_from(end, now);
        if diff.is_negative() {
            Poll::Ready(())
        } else {
            kernel.yield_for(self.duration)
        }
    }
}

#[inline]
pub const fn sleep_async<C: Clock>(dur: Duration<C>) -> Sleep<C> {
    Sleep::new(dur)
}
