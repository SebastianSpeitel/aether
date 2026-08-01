use core::{error::Error, fmt::Arguments, mem::MaybeUninit};

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
    type RawToken<T: ?Sized>: Token<T, Self, true> + Copy;

    /// The safe, owned, memory-managed smart pointer (like Box).
    type Token<T: ?Sized>: Token<T, Self>;

    /// Casts a raw token from one type to another.
    ///
    /// # Safety
    /// The caller must ensure that the casted pointer matches the type's alignment
    /// and size invariants.
    unsafe fn cast<T: ?Sized, U>(&self, token: Self::RawToken<T>) -> Self::RawToken<U>;

    /// Downgrades an owned token to a raw copyable token.
    fn downgrade<T: ?Sized>(&self, owned: &Self::Token<T>) -> Self::RawToken<T>;

    /// Upgrades a raw token to an owned token.
    fn upgrade<T: ?Sized>(&self, token: Self::RawToken<T>) -> Result<Self::Token<T>, Self::Error>;

    /// Handles a fatal error or contract violation.
    ///
    /// In debug builds (`cfg!(debug_assertions)`), panics with diagnostic details.
    /// In release builds (`cfg!(not(debug_assertions))`), hints to the compiler that the code path is unreachable.
    #[inline]
    fn handle_error(&self, error: Self::Error, context: Option<Arguments<'_>>) -> ! {
        #[cfg(debug_assertions)]
        {
            if let Some(ctx) = context {
                panic!("allocator error ({:?}): {}", error, ctx);
            } else {
                panic!("allocator error ({:?})", error);
            }
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = (error, context);
            unsafe { core::hint::unreachable_unchecked() }
        }
    }

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
            let raw_token = self.downgrade(&raw_owned);
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
        let raw_uninit = self.downgrade(&owned_uninit);
        unsafe {
            let ptr =
                core::ptr::from_mut::<MaybeUninit<T>>(&mut *self.write_unchecked(raw_uninit)).cast::<T>();
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

    /// Safely acquires an immutable borrow guard to the raw token's memory.
    ///
    /// This is the primary required method for validated read operations.
    fn read_raw<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl Guard<T> + 'a, Self::Error>;

    /// Safely acquires an immutable borrow guard, accepting any token kind.
    ///
    /// Accepts both `A::RawToken<T>` (RAW=true) and `A::Token<T>` (RAW=false)
    /// transparently; the latter is downgraded via [`downgrade`](Self::downgrade)
    /// before the read.
    #[inline]
    fn read<'a, T: ?Sized + 'a, const RAW: bool>(
        &'a self,
        token: &impl Token<T, Self, RAW>,
    ) -> Result<impl Guard<T> + 'a, Self::Error> {
        self.read_raw(token.as_raw(self))
    }

    /// Safely acquires a mutable borrow guard to the raw token's memory.
    ///
    /// This is the primary required method for validated write operations.
    fn write_raw<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl GuardMut<T> + 'a, Self::Error>;

    /// Safely acquires a mutable borrow guard, accepting any token kind.
    #[inline]
    fn write<'a, T: ?Sized + 'a, const RAW: bool>(
        &'a self,
        token: &impl Token<T, Self, RAW>,
    ) -> Result<impl GuardMut<T> + 'a, Self::Error> {
        self.write_raw(token.as_raw(self))
    }

    /// Reads the value without checking validity or lifetime.
    ///
    /// Delegates to [`read_raw`](Self::read_raw) and calls [`handle_error`](Self::handle_error) on failure.
    /// In release builds, [`handle_error`](Self::handle_error) optimizes to `unreachable_unchecked()`.
    ///
    /// # Safety
    /// The caller must ensure the token is valid and currently allocated.
    #[inline]
    unsafe fn read_unchecked<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> impl Guard<T> + 'a {
        self.read_raw(token)
            .unwrap_or_else(|err| self.handle_error(err, Some(format_args!("read_unchecked validation failed"))))
    }

    /// Writes to the value without checking validity or lifetime.
    ///
    /// Delegates to [`write_raw`](Self::write_raw) and calls [`handle_error`](Self::handle_error) on failure.
    /// In release builds, [`handle_error`](Self::handle_error) optimizes to `unreachable_unchecked()`.
    ///
    /// # Safety
    /// The caller must ensure the token is valid, currently allocated, and that this
    /// is the exclusive reference to the underlying memory.
    #[inline]
    unsafe fn write_unchecked<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> impl GuardMut<T> + 'a {
        self.write_raw(token)
            .unwrap_or_else(|err| self.handle_error(err, Some(format_args!("write_unchecked validation failed"))))
    }
}

/// Capability trait for kernels or contexts that provide access to an `Allocator`.
pub trait HasAllocator {
    type Alloc<'a>: core::ops::Deref<Target: Allocator + Sized> + 'a
    where
        Self: 'a;

    fn get_allocator<'a>(&'a self) -> Self::Alloc<'a>;
}
