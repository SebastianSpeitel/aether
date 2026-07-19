use aether_core::allocator::Allocator;
use aether_core::guard::{Guard, GuardMut};
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A fast, thread-safe bump allocator.
///
/// It uses atomic counters (`AtomicUsize`) for lock-free allocation from a
/// contiguous block of memory. Ideal for short-lived, phase-based allocations.
pub struct ArenaAllocator<const N: usize> {
    buffer: UnsafeCell<[u8; N]>,
    position: AtomicUsize,
}

unsafe impl<const N: usize> Sync for ArenaAllocator<N> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaError {
    OutOfMemory,
    InvalidToken,
}

impl core::fmt::Display for ArenaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("Arena out of memory"),
            Self::InvalidToken => f.write_str("Invalid token for this arena allocator"),
        }
    }
}

impl core::error::Error for ArenaError {}

impl<const N: usize> ArenaAllocator<N> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0; N]),
            position: AtomicUsize::new(0),
        }
    }

    pub fn reset(&self) {
        self.position.store(0, Ordering::Relaxed);
    }

    fn validate<T: ?Sized>(&self, token: NonNull<T>) -> Result<(), ArenaError> {
        let start = self.buffer.get().addr();
        let end = start + N;
        let ptr = token.as_ptr().addr();
        if ptr >= start && ptr < end {
            Ok(())
        } else {
            core::hint::cold_path();
            Err(ArenaError::InvalidToken)
        }
    }
}

impl<const N: usize> Default for ArenaAllocator<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Allocator for ArenaAllocator<N> {
    type Error = ArenaError;
    type RawToken<T: ?Sized> = NonNull<T>;
    type Token<T: ?Sized> = NonNull<T>;

    #[inline]
    unsafe fn cast<T: ?Sized, U>(&self, token: Self::RawToken<T>) -> Self::RawToken<U> {
        token.cast()
    }

    #[inline]
    fn downgrade<T: ?Sized>(&self, owned: &Self::Token<T>) -> Self::RawToken<T> {
        *owned
    }

    #[inline]
    fn upgrade<T: ?Sized>(&self, token: Self::RawToken<T>) -> Result<Self::Token<T>, Self::Error> {
        self.validate(token)?;
        Ok(token)
    }

    fn allocate_raw(&self, layout: Layout) -> Result<Self::Token<[MaybeUninit<u8>]>, Self::Error> {
        let mut current_pos = self.position.load(Ordering::Relaxed);
        loop {
            let align_mask = layout.align() - 1;
            let aligned_pos = (current_pos + align_mask) & !align_mask;
            let next_pos = aligned_pos + layout.size();

            if next_pos > N {
                core::hint::cold_path();
                return Err(ArenaError::OutOfMemory);
            }

            match self.position.compare_exchange_weak(
                current_pos,
                next_pos,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => unsafe {
                    let buffer_ptr = self.buffer.get().cast::<u8>();
                    let data_ptr = buffer_ptr.add(aligned_pos);
                    let slice_ptr = core::ptr::slice_from_raw_parts_mut(
                        data_ptr.cast::<MaybeUninit<u8>>(),
                        layout.size(),
                    );
                    return Ok(NonNull::new_unchecked(slice_ptr));
                },
                Err(actual) => current_pos = actual,
            }
        }
    }

    unsafe fn deallocate_raw(
        &self,
        _token: Self::RawToken<MaybeUninit<u8>>,
        _layout: Layout,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn read<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl Guard<T> + 'a, Self::Error> {
        self.validate(token)?;
        unsafe { Ok(token.as_ref()) }
    }

    #[inline]
    fn write<'a, T: ?Sized + 'a>(
        &'a self,
        token: Self::RawToken<T>,
    ) -> Result<impl GuardMut<T> + 'a, Self::Error> {
        self.validate(token)?;
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
