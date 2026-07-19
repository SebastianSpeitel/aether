use crate::allocator::Allocator;
use crate::guard::{Guard, GuardMut};
use core::ptr::NonNull;

#[cfg(feature = "std")]
extern crate alloc;
#[cfg(feature = "std")]
use alloc::boxed::Box;

/// Extension trait providing pointer-like methods for Allocator tokens.
#[allow(clippy::missing_errors_doc)]
pub trait Token<T: ?Sized, A: Allocator + ?Sized> {
    /// Safely acquires an immutable borrow guard to the memory.
    fn read<'a>(&self, alloc: &'a A) -> Result<impl Guard<T> + 'a, A::Error>
    where
        T: 'a;

    /// Safely acquires a mutable borrow guard to the memory.
    fn write<'a>(&mut self, alloc: &'a A) -> Result<impl GuardMut<T> + 'a, A::Error>
    where
        T: 'a;
}

/// Extension trait specifically for assuming initialization of `MaybeUninit` raw tokens.
pub trait TokenAssumeInit<U, A: Allocator + ?Sized> {
    /// Assumes the uninitialized memory represents a valid initialized value of type U.
    ///
    /// # Safety
    /// The caller must ensure the memory has been fully initialized.
    unsafe fn assume_init(self, alloc: &A) -> A::RawToken<U>;
}

// Implement TokenAssumeInit only for raw tokens containing MaybeUninit<U>
impl<U, A: Allocator + ?Sized> TokenAssumeInit<U, A> for A::RawToken<core::mem::MaybeUninit<U>> {
    #[inline]
    unsafe fn assume_init(self, alloc: &A) -> A::RawToken<U> {
        unsafe { alloc.cast(self) }
    }
}

// Implement Token for NonNull when the Allocator uses NonNull as its RawToken
impl<T: ?Sized, A> Token<T, A> for NonNull<T>
where
    A: Allocator<RawToken<T> = Self>,
{
    #[inline]
    fn read<'a>(&self, alloc: &'a A) -> Result<impl Guard<T> + 'a, A::Error>
    where
        T: 'a,
    {
        alloc.read(*self)
    }

    #[inline]
    fn write<'a>(&mut self, alloc: &'a A) -> Result<impl GuardMut<T> + 'a, A::Error>
    where
        T: 'a,
    {
        alloc.write(*self)
    }
}

// Implement Token for Box when the Allocator uses Box as its Token and NonNull as its RawToken
#[cfg(feature = "std")]
impl<T: ?Sized, A> Token<T, A> for Box<T>
where
    A: Allocator<Token<T> = Self, RawToken<T> = NonNull<T>>,
{
    #[inline]
    fn read<'a>(&self, alloc: &'a A) -> Result<impl Guard<T> + 'a, A::Error>
    where
        T: 'a,
    {
        let raw = NonNull::from(&**self);
        alloc.read(raw)
    }

    #[inline]
    fn write<'a>(&mut self, alloc: &'a A) -> Result<impl GuardMut<T> + 'a, A::Error>
    where
        T: 'a,
    {
        let raw = NonNull::from(&mut **self);
        alloc.write(raw)
    }
}
