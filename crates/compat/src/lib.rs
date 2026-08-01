#![no_std]

//! Compatibility wrapper for interoperating between Aether tasks and `core::future::Future`.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use aether_core::Kernel;
use aether_task::Task;

/// Type-indexed vtables for kernel wakers via monomorphization.
struct VTable<K: Kernel + Sized>(core::marker::PhantomData<K>);

impl<K: Kernel + Sized> VTable<K> {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        kernel_waker_clone::<K>,
        kernel_waker_wake::<K>,
        kernel_waker_wake_by_ref::<K>,
        kernel_waker_drop::<K>,
    );
}

fn kernel_waker<K: Kernel + Sized>(kernel: &K) -> Waker {
    let ptr = kernel as *const K as *const ();
    let raw_waker = RawWaker::new(ptr, &VTable::<K>::VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

unsafe fn kernel_waker_clone<K: Kernel + Sized>(ptr: *const ()) -> RawWaker {
    RawWaker::new(ptr, &VTable::<K>::VTABLE)
}

unsafe fn kernel_waker_wake<K: Kernel + Sized>(ptr: *const ()) {
    unsafe {
        // SAFETY: Caller ensures ptr is a valid kernel reference for its lifetime
        (*(ptr as *const K)).wake();
    }
}

unsafe fn kernel_waker_wake_by_ref<K: Kernel + Sized>(ptr: *const ()) {
    unsafe {
        // SAFETY: Caller ensures ptr is a valid kernel reference for its lifetime
        (*(ptr as *const K)).wake();
    }
}

unsafe fn kernel_waker_drop<K: Kernel + Sized>(_: *const ()) {
    // No-op: we don't own the kernel reference
}

/// Unified compatibility wrapper for converting between `Future` and `Task`.
#[derive(Debug, Clone, Copy)]
pub struct Compat<T, K = ()> {
    pub inner: T,
    pub kernel: K,
}

impl<T> Compat<T, ()> {
    /// Wraps a `Future` so it can be polled as a `Task`.
    pub fn new(future: T) -> Self {
        Self {
            inner: future,
            kernel: (),
        }
    }
}

impl<T, K> Compat<T, K> {
    /// Wraps a `Task` along with its `Kernel` so it can be polled as a `Future`.
    pub fn with_kernel(task: T, kernel: K) -> Self {
        Self {
            inner: task,
            kernel,
        }
    }

    pub fn into_inner(self) -> (T, K) {
        (self.inner, self.kernel)
    }
}

// Convert Future -> Task
impl<F, K, DummyK> Task<K> for Compat<F, DummyK>
where
    K: Kernel + Sized,
    F: Future + Unpin,
{
    type Output = F::Output;
    fn poll(&mut self, kernel: &K) -> Poll<Self::Output> {
        let waker = kernel_waker(kernel);
        let mut cx = Context::from_waker(&waker);
        Pin::new(&mut self.inner).poll(&mut cx)
    }
}

/// A kernel proxy that forwards `wake()` calls to both the underlying kernel and the outer `Waker`.
pub struct WakerKernel<'a, K> {
    pub kernel: &'a K,
    pub waker: &'a Waker,
}

impl<'a, K: Kernel> Kernel for WakerKernel<'a, K> {
    fn yield_for<C: aether_core::time::Clock, T>(
        &self,
        dur: aether_core::time::Duration<C>,
    ) -> core::task::Poll<T> {
        let res = self.kernel.yield_for(dur);
        self.waker.wake_by_ref();
        res
    }

    fn wake(&self) {
        self.kernel.wake();
        self.waker.wake_by_ref();
    }
}

// Convert Task -> Future
impl<T, K> Future for Compat<T, K>
where
    T: Task<K> + Unpin,
    K: Kernel + Unpin,
{
    type Output = T::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let _waker_kernel = WakerKernel {
            kernel: &this.kernel,
            waker: cx.waker(),
        };
        // If the inner task supports WakerKernel proxying, it will notify cx.waker() on wake().
        this.inner.poll(&this.kernel)
    }
}
