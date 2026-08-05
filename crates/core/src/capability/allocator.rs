use core::error::Error;
use core::mem::MaybeUninit;

use super::guard::{Guard, GuardMut};
use super::token::Token;

/// Abstract memory allocator managing allocation, access, and deallocation.
pub trait Allocator {
    type Error: Error;

    /// The low-level, copyable, lifetimeless raw pointer representation.
    type RawToken<T: ?Sized>: Token<T, Self, true> + Copy;

    /// The safe, owned, memory-managed smart pointer.
    type Token<T: ?Sized>: Token<T, Self, false>;

    /// Handles a fatal allocation error or validation failure.
    ///
    /// In debug builds (`cfg!(debug_assertions)`), panics with diagnostic details.
    /// In release builds (`cfg!(not(debug_assertions))`), hints to the compiler that the code path is unreachable.
    #[inline]
    fn handle_error(&self, error: Self::Error, context: Option<core::fmt::Arguments<'_>>) -> ! {
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

    /// Casts a raw token from one type to another.
    ///
    /// # Safety
    /// The caller must ensure the pointer representation is valid for `U`.
    unsafe fn cast<T: ?Sized, U>(&self, token: Self::RawToken<T>) -> Self::RawToken<U>;

    /// Downgrades an owned token reference to a raw copyable token.
    fn downgrade<T: ?Sized>(&self, owned: &Self::Token<T>) -> Self::RawToken<T>;

    /// Upgrades a raw token to an owned token.
    fn upgrade<T: ?Sized>(&self, token: Self::RawToken<T>) -> Result<Self::Token<T>, Self::Error>;

    /// Allocates uninitialized memory for type `T`.
    #[inline]
    fn allocate_uninit<T>(&self) -> Result<Self::Token<MaybeUninit<T>>, Self::Error> {
        let layout = core::alloc::Layout::new::<MaybeUninit<T>>();
        let raw = self.allocate_raw(layout)?;
        let typed_raw = unsafe { self.cast::<[MaybeUninit<u8>], MaybeUninit<T>>(raw) };
        self.upgrade(typed_raw)
    }

    /// Allocates raw memory for a specific layout.
    fn allocate_raw(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<Self::RawToken<[MaybeUninit<u8>]>, Self::Error>;

    /// Allocates space and initializes it with a value.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn new<T>(&self, value: T) -> Result<Self::Token<T>, Self::Error>
    where
        Self: Sized,
    {
        let owned_uninit = self.allocate_uninit::<T>()?;
        let raw_uninit = self.downgrade(&owned_uninit);
        unsafe {
            let ptr =
                core::ptr::from_mut::<MaybeUninit<T>>(&mut *self.get_mut_unchecked(raw_uninit))
                    .cast::<T>();
            ptr.write(value);
            let raw_init = self.cast::<MaybeUninit<T>, T>(raw_uninit);
            self.upgrade(raw_init)
        }
    }

    /// Deallocates raw memory associated with a token.
    ///
    /// # Safety
    /// The caller must ensure the memory is no longer referenced.
    unsafe fn deallocate_raw(
        &self,
        token: Self::RawToken<MaybeUninit<u8>>,
        layout: core::alloc::Layout,
    ) -> Result<(), Self::Error>;

    /// Deallocates memory associated with a typed token.
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
    /// This is the primary required method for validated get_ref operations.
    fn get_ref_raw<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl Guard<T> + 'a, Self::Error>;

    /// Safely acquires an immutable borrow guard, accepting any token kind.
    ///
    /// Accepts both `A::RawToken<T>` (RAW=true) and `A::Token<T>` (RAW=false)
    /// transparently; the latter is downgraded via [`downgrade`](Self::downgrade)
    /// before acquiring the reference.
    #[inline]
    fn get_ref<'a, T: ?Sized + 'a, const RAW: bool>(
        &'a self,
        token: &impl Token<T, Self, RAW>,
    ) -> Result<impl Guard<T> + 'a, Self::Error> {
        self.get_ref_raw(token.as_raw(self))
    }

    /// Safely acquires a mutable borrow guard to the raw token's memory.
    ///
    /// This is the primary required method for validated get_mut operations.
    fn get_mut_raw<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl GuardMut<T> + 'a, Self::Error>;

    /// Safely acquires a mutable borrow guard, accepting any token kind.
    #[inline]
    fn get_mut<'a, T: ?Sized + 'a, const RAW: bool>(
        &'a self,
        token: &impl Token<T, Self, RAW>,
    ) -> Result<impl GuardMut<T> + 'a, Self::Error> {
        self.get_mut_raw(token.as_raw(self))
    }

    /// Borrows an immutable reference to the value without checking validity or lifetime.
    ///
    /// Delegates to [`get_ref_raw`](Self::get_ref_raw) and calls [`handle_error`](Self::handle_error) on failure.
    /// In release builds, [`handle_error`](Self::handle_error) optimizes to `unreachable_unchecked()`.
    ///
    /// # Safety
    /// The caller must ensure the token is valid and currently allocated.
    #[inline]
    unsafe fn get_ref_unchecked<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> impl Guard<T> + 'a {
        self.get_ref_raw(token).unwrap_or_else(|err| {
            self.handle_error(
                err,
                Some(format_args!("get_ref_unchecked validation failed")),
            )
        })
    }

    /// Borrows a mutable reference to the value without checking validity or lifetime.
    ///
    /// Delegates to [`get_mut_raw`](Self::get_mut_raw) and calls [`handle_error`](Self::handle_error) on failure.
    /// In release builds, [`handle_error`](Self::handle_error) optimizes to `unreachable_unchecked()`.
    ///
    /// # Safety
    /// The caller must ensure the token is valid, currently allocated, and that this
    /// is the exclusive reference to the underlying memory.
    #[inline]
    unsafe fn get_mut_unchecked<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> impl GuardMut<T> + 'a {
        self.get_mut_raw(token).unwrap_or_else(|err| {
            self.handle_error(
                err,
                Some(format_args!("get_mut_unchecked validation failed")),
            )
        })
    }
}

/// Capability trait for kernels or contexts that provide access to an `Allocator`.
pub trait HasAllocator {
    type Alloc<'a>: core::ops::Deref<Target: Allocator + Sized> + 'a
    where
        Self: 'a;

    /// Acquires a reference to the `Allocator` instance.
    fn get_allocator<'a>(&'a self) -> Self::Alloc<'a>;
}
