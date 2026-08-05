use aether_core::capability::HasClock;
use aether_core::clock::{Clock, Duration, Instant};
use aether_core::{Kernel, Task};
use core::task::Poll;

pub struct RepeatWith<F, T> {
    pub(crate) factory: F,
    pub(crate) task: T,
}

impl<K: Kernel, F, T> Task<K> for RepeatWith<F, T>
where
    F: FnMut() -> T,
    T: Task<K>,
{
    type Output = T::Output;

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        match self.task.poll(kern) {
            Poll::Ready(out) => {
                self.task = (self.factory)();
                Poll::Ready(out)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct ResettableTimeout<const D: usize, F, C: Clock> {
    start: Option<Instant<C>>,
    action: F,
}

impl<const D: usize, F, C: Clock> ResettableTimeout<D, F, C> {
    pub const fn new(action: F) -> Self {
        Self {
            start: None,
            action,
        }
    }
    pub fn start(&mut self, now: Instant<C>) {
        self.start.get_or_insert(now);
    }
    pub const fn cancel(&mut self) {
        self.start = None;
    }
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.start.is_some()
    }
    pub fn poll<A>(&mut self, now: Instant<C>, _clock: &C, args: A)
    where
        F: FnMut(A),
    {
        if let Some(s) = self.start {
            let target = C::add_duration(s, Duration::<C>::from_millis(D as u64));
            let diff = C::offset_from(now, target);
            if !diff.is_negative() {
                self.start = None;
                (self.action)(args);
            }
        }
    }
}

pub struct Map<T, F> {
    pub(crate) task: T,
    pub(crate) f: F,
}

impl<K: Kernel, T, F, B> Task<K> for Map<T, F>
where
    T: Task<K>,
    F: FnMut(T::Output) -> B,
{
    type Output = B;

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        match self.task.poll(kern) {
            Poll::Ready(out) => Poll::Ready((self.f)(out)),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Timeout<T, C: Clock> {
    pub(crate) task: T,
    pub(crate) start_time: Option<Instant<C>>,
    pub(crate) limit: Duration<C>,
}

impl<K, T: Task<K>, C: Clock> Task<K> for Timeout<T, C>
where
    K: Kernel + HasClock<C>,
{
    type Output = Result<T::Output, ()>;

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        let clock = kern.get_clock();
        let now = clock.now();
        if self.start_time.is_none() {
            self.start_time = Some(now);
        }

        match self.task.poll(kern) {
            Poll::Ready(out) => {
                self.start_time = None;
                Poll::Ready(Ok(out))
            }
            Poll::Pending => {
                if let Some(start) = self.start_time {
                    let deadline = C::add_duration(start, self.limit);
                    let diff = C::offset_from(now, deadline);
                    if !diff.is_negative() {
                        self.start_time = None;
                        return Poll::Ready(Err(()));
                    }
                }
                Poll::Pending
            }
        }
    }
}

pub struct Then<T1, F, T2> {
    pub(crate) state: ThenState<T1, F, T2>,
}

pub(crate) enum ThenState<T1, F, T2> {
    First(T1, F),
    Second(T2),
}

impl<K: Kernel, T1, F, T2> Task<K> for Then<T1, F, T2>
where
    T1: Task<K>,
    F: FnOnce(T1::Output) -> T2,
    T2: Task<K>,
{
    type Output = T2::Output;

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        loop {
            match &mut self.state {
                ThenState::First(t1, _) => match t1.poll(kern) {
                    Poll::Ready(out) => {
                        if let ThenState::First(_, f) =
                            core::mem::replace(&mut self.state, unsafe { core::mem::zeroed() })
                        {
                            let t2 = f(out);
                            self.state = ThenState::Second(t2);
                        }
                    }
                    Poll::Pending => return Poll::Pending,
                },
                ThenState::Second(t2) => return t2.poll(kern),
            }
        }
    }
}

pub struct RepeatN<F, T> {
    pub(crate) factory: F,
    pub(crate) task: T,
    pub(crate) remaining: u8,
}

impl<K: Kernel, F, T> Task<K> for RepeatN<F, T>
where
    F: FnMut() -> T,
    T: Task<K>,
{
    type Output = ();

    #[inline]
    fn poll(&mut self, kern: &K) -> Poll<Self::Output> {
        if self.remaining == 0 {
            return Poll::Ready(());
        }
        match self.task.poll(kern) {
            Poll::Ready(_) => {
                self.remaining -= 1;
                if self.remaining == 0 {
                    Poll::Ready(())
                } else {
                    self.task = (self.factory)();
                    Poll::Pending
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
