use core::ops::{Deref, DerefMut};

/// Semantic alias for an immutable memory guard.
pub trait Guard<T: ?Sized>: Deref<Target = T> {}
impl<G: ?Sized + Deref<Target = T>, T: ?Sized> Guard<T> for G {}

/// Semantic alias for a mutable memory guard.
pub trait GuardMut<T: ?Sized>: DerefMut<Target = T> {}
impl<G: ?Sized + DerefMut<Target = T>, T: ?Sized> GuardMut<T> for G {}
