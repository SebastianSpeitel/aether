use crate::kernel::Kernel;
use core::task::Poll;

/// A unit of execution that can be polled to completion by a Kernel.
pub trait Task<K: Kernel + ?Sized> {
    /// The value produced when the task finishes executing.
    type Output;

    /// Attempts to resolve the task, returning `Poll::Ready(Output)` or `Poll::Pending`.
    fn poll(&mut self, kernel: &K) -> Poll<Self::Output>;
}

/// `None` is idle; `Some(task)` polls the inner task and auto-clears to `None` on completion.
impl<K: Kernel, T: Task<K>> Task<K> for Option<T> {
    type Output = T::Output;

    #[inline]
    fn poll(&mut self, cx: &K) -> Poll<Self::Output> {
        let result = self.as_mut().map_or(Poll::Pending, |task| task.poll(cx));
        if result.is_ready() {
            *self = None;
        }
        result
    }
}

macro_rules! impl_tuple_task {
    ($($T:ident),*) => {
        impl<K: Kernel, $($T),*> Task<K> for ($($T,)*)
        where
            $($T: Task<K>),*
        {
            type Output = core::convert::Infallible;

            #[inline]
            fn poll(&mut self, kernel: &K) -> Poll<Self::Output> {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
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

use aether_time::{Clock, Instant, sleep_async::Sleep};

impl<K: Kernel, C: Clock> Task<K> for Sleep<C> {
    type Output = ();

    fn poll(&mut self, kernel: &K) -> Poll<Self::Output> {
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
