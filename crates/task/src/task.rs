use aether_core::Kernel;
use aether_core::time::{Clock, Duration};
use core::task::Poll;

use crate::combinators::{Map, RepeatWith, Timeout};

pub trait Task<K: Kernel + ?Sized> {
    type Output;
    fn poll(&mut self, kernel: &mut K) -> Poll<Self::Output>;

    #[inline]
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized + Task<K>,
        F: FnMut(Self::Output) -> B,
    {
        Map { task: self, f }
    }

    #[inline]
    fn timeout<C: Clock>(self, limit: Duration<C>) -> Timeout<Self, C>
    where
        Self: Sized + Task<K>,
    {
        Timeout {
            task: self,
            start_time: None,
            limit,
        }
    }

    #[inline]
    fn repeat_with<F>(self, factory: F) -> RepeatWith<F, Self>
    where
        Self: Sized + Task<K>,
        F: FnMut() -> Self,
    {
        RepeatWith {
            factory,
            task: self,
        }
    }
}

/// `None` is idle; `Some(task)` polls the inner task and auto-clears to `None` on completion.
impl<K: Kernel, T: Task<K>> Task<K> for Option<T> {
    type Output = T::Output;

    #[inline]
    fn poll(&mut self, cx: &mut K) -> Poll<Self::Output> {
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
            fn poll(&mut self, kernel: &mut K) -> Poll<Self::Output> {
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
