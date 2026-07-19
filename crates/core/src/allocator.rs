use core::{error::Error, mem::MaybeUninit};

use crate::guard::{Guard, GuardMut};
use crate::token::Token;

/// An abstract allocator that operates on tokens instead of raw pointers.
///
/// This trait defines a memory management interface that separates allocation,
/// access, and deallocation. It returns opaque `Token` and `RawToken` types
/// which must be presented back to the allocator for access (`read`, `write`, `upgrade`)
/// or deallocation.
#[allow(
    clippy::missing_errors_doc,
    clippy::wrong_self_convention,
    clippy::mut_from_ref
)]
pub trait Allocator {
    type Error: Error;

    /// The low-level, copyable, lifetimeless raw pointer representation.
    type RawToken<T: ?Sized>: Token<T, Self> + Copy;

    /// The safe, owned, memory-managed smart pointer (like Box).
    type Token<T: ?Sized>: Token<T, Self>;

    /// Casts a raw token from one type to another.
    ///
    /// # Safety
    /// The caller must ensure that the casted pointer matches the type's alignment
    /// and size invariants.
    unsafe fn cast<T: ?Sized, U>(&self, token: Self::RawToken<T>) -> Self::RawToken<U>;

    /// Downgrades an owned token to a raw copyable token.
    fn downgrade<T: ?Sized>(&self, owned: Self::Token<T>) -> Self::RawToken<T>;

    /// Upgrades a raw token to an owned token.
    fn upgrade<T: ?Sized>(&self, token: Self::RawToken<T>) -> Result<Self::Token<T>, Self::Error>;

    /// Allocates raw uninitialized memory of a given layout.
    fn allocate_raw(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<Self::Token<[MaybeUninit<u8>]>, Self::Error>;

    /// Allocates space for an uninitialized typed value.
    #[inline]
    fn allocate_uninit<T>(&self) -> Result<Self::Token<MaybeUninit<T>>, Self::Error>
    where
        Self: Sized,
    {
        let layout = core::alloc::Layout::new::<T>();
        self.allocate_raw(layout).and_then(|raw_owned| {
            let raw_token = self.downgrade(raw_owned);
            let typed_token = unsafe { self.cast(raw_token) };
            self.upgrade(typed_token)
        })
    }

    /// Allocates space and initializes it with a value.
    #[inline]
    fn new<T>(&self, value: T) -> Result<Self::Token<T>, Self::Error>
    where
        Self: Sized,
    {
        let owned_uninit = self.allocate_uninit::<T>()?;
        let raw_uninit = self.downgrade(owned_uninit);
        unsafe {
            let ptr =
                core::ptr::from_mut::<MaybeUninit<T>>(self.write_unchecked(raw_uninit)).cast::<T>();
            ptr.write(value);
            let raw_init = self.cast::<MaybeUninit<T>, T>(raw_uninit);
            self.upgrade(raw_init)
        }
    }

    /// Deallocates raw memory previously allocated with `allocate_raw`.
    ///
    /// # Safety
    /// The caller must ensure that the memory is no longer referenced.
    unsafe fn deallocate_raw(
        &self,
        token: Self::RawToken<MaybeUninit<u8>>,
        layout: core::alloc::Layout,
    ) -> Result<(), Self::Error>;

    /// Deallocates space for a typed value.
    ///
    /// # Safety
    /// The caller must ensure that the memory is no longer referenced.
    #[inline]
    unsafe fn deallocate<T>(&self, token: Self::RawToken<T>) -> Result<(), Self::Error> {
        let layout = core::alloc::Layout::new::<T>();
        let thin_token = unsafe { self.cast::<T, MaybeUninit<u8>>(token) };
        unsafe { self.deallocate_raw(thin_token, layout) }
    }

    /// Safely acquires an immutable borrow guard to the token's memory.
    fn read<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl Guard<T> + 'a, Self::Error>;

    /// Safely acquires a mutable borrow guard to the token's memory.
    fn write<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl GuardMut<T> + 'a, Self::Error>;

    /// Reads the value without checking validity or lifetime.
    ///
    /// # Safety
    /// The caller must ensure the token is valid and currently allocated.
    unsafe fn read_unchecked<T: ?Sized>(&self, token: Self::RawToken<T>) -> &T;

    /// Writes to the value without checking validity or lifetime.
    ///
    /// # Safety
    /// The caller must ensure the token is valid, currently allocated, and that this
    /// is the exclusive reference to the underlying memory.
    unsafe fn write_unchecked<T: ?Sized>(&self, token: Self::RawToken<T>) -> &mut T;
}
