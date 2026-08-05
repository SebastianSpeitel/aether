use core::task::Poll;

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
