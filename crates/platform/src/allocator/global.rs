extern crate alloc;
use aether_core::allocator::Allocator;
use aether_core::guard::{Guard, GuardMut};
use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};
use core::mem::MaybeUninit;
use core::ptr::NonNull;

/// An adapter that implements `Allocator` for any standard `GlobalAlloc`.
///
/// It uses `Box<T>` as the owned token and `NonNull<T>` as the raw token.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Global<A: GlobalAlloc>(A);

impl<A: GlobalAlloc> Global<A> {
    #[must_use]
    pub const fn new(allocator: A) -> Self {
        Self(allocator)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AllocError;

impl core::fmt::Display for AllocError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("Allocation failed")
    }
}

impl core::error::Error for AllocError {}

impl<A: GlobalAlloc> Allocator for Global<A> {
    type Error = AllocError;
    type RawToken<T: ?Sized> = NonNull<T>;
    type Token<T: ?Sized> = Box<T>;

    #[inline]
    unsafe fn cast<T: ?Sized, U>(&self, token: Self::RawToken<T>) -> Self::RawToken<U> {
        token.cast()
    }

    #[inline]
    fn downgrade<T: ?Sized>(&self, owned: &Self::Token<T>) -> Self::RawToken<T> {
        NonNull::from_ref(owned)
    }

    #[inline]
    fn upgrade<T: ?Sized>(&self, token: Self::RawToken<T>) -> Result<Self::Token<T>, Self::Error> {
        unsafe { Ok(Box::from_raw(token.as_ptr())) }
    }

    #[inline]
    fn allocate_raw(&self, layout: Layout) -> Result<Self::Token<[MaybeUninit<u8>]>, Self::Error> {
        unsafe {
            let ptr = self.0.alloc(layout);
            if ptr.is_null() {
                core::hint::cold_path();
                return Err(AllocError);
            }
            let slice_ptr =
                core::ptr::slice_from_raw_parts_mut(ptr.cast::<MaybeUninit<u8>>(), layout.size());
            Ok(Box::from_raw(slice_ptr))
        }
    }

    #[inline]
    unsafe fn deallocate_raw(
        &self,
        token: Self::RawToken<MaybeUninit<u8>>,
        layout: Layout,
    ) -> Result<(), Self::Error> {
        unsafe {
            self.0.dealloc(token.as_ptr().cast::<u8>(), layout);
            Ok(())
        }
    }

    #[inline]
    fn read<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl Guard<T> + 'a, Self::Error> {
        unsafe { Ok(token.as_ref()) }
    }

    #[inline]
    fn write<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl GuardMut<T> + 'a, Self::Error> {
        unsafe { Ok(&mut *token.as_ptr()) }
    }

    #[inline]
    unsafe fn read_unchecked<T: ?Sized>(&self, token: Self::RawToken<T>) -> &T {
        unsafe { token.as_ref() }
    }

    #[inline]
    unsafe fn write_unchecked<T: ?Sized>(&self, token: Self::RawToken<T>) -> &mut T {
        unsafe { &mut *token.as_ptr() }
    }
}
