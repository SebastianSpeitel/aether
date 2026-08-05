use core::task::Poll;

use crate::capability::HasClock;
use crate::clock::{Clock, Duration, Instant};
use crate::kernel::Kernel;

/// The fundamental task trait in Aether.
///
/// A task represents a unit of work executed within an environment parameterized by `K`.
pub trait Task<K> {
    type Output;

    /// Polls the task to completion.
    fn poll(&mut self, kernel: &K) -> Poll<Self::Output>;
}

/// Helper macro to implement `Task` for tuples of tasks.
macro_rules! impl_tuple_task {
    ($($T:ident),+) => {
        impl<K: Kernel, $($T: Task<K, Output = ()>),+> Task<K> for ($($T,)+) {
            type Output = ();

            #[inline]
            fn poll(&mut self, kernel: &K) -> Poll<Self::Output> {
                #[allow(non_snake_case)]
                let ($($T,)+) = self;
                $(
                    let _ = $T.poll(kernel);
                )*
                Poll::Pending
            }
        }
    };
}

impl_tuple_task!(A, B);
impl_tuple_task!(A, B, C);
impl_tuple_task!(A, B, C, D);
impl_tuple_task!(A, B, C, D, E);
impl_tuple_task!(A, B, C, D, E, F);

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
