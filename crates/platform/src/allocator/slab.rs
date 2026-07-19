use aether_core::allocator::Allocator;
use aether_core::guard::{Guard, GuardMut};
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, Ordering};

/// A thread-safe allocator for fixed-size blocks.
///
/// Uses an atomic free list and a spinlock to manage contention, providing
/// extremely fast allocation and deallocation of uniform types.
pub struct SlabAllocator<const BLOCK_SIZE: usize, const TOTAL_SIZE: usize> {
    buffer: UnsafeCell<[MaybeUninit<u8>; TOTAL_SIZE]>,
    free_head: AtomicPtr<FreeNode>,
    state: AtomicU8, // 0: Uninitialized, 1: Initializing, 2: Initialized
    lock: AtomicBool,
}

struct FreeNode {
    next: Option<NonNull<Self>>,
}

unsafe impl<const BLOCK_SIZE: usize, const TOTAL_SIZE: usize> Sync
    for SlabAllocator<BLOCK_SIZE, TOTAL_SIZE>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlabError {
    OutOfMemory,
    InvalidLayout,
    InvalidToken,
}

impl core::fmt::Display for SlabError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("Slab allocator out of memory"),
            Self::InvalidLayout => {
                f.write_str("Requested layout exceeds slab block size or alignment")
            }
            Self::InvalidToken => f.write_str("Invalid token for this slab allocator"),
        }
    }
}

impl core::error::Error for SlabError {}

impl<const BLOCK_SIZE: usize, const TOTAL_SIZE: usize> Default
    for SlabAllocator<BLOCK_SIZE, TOTAL_SIZE>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const BLOCK_SIZE: usize, const TOTAL_SIZE: usize> SlabAllocator<BLOCK_SIZE, TOTAL_SIZE> {
    #[must_use]
    #[allow(clippy::manual_is_multiple_of)]
    pub const fn new() -> Self {
        const {
            assert!(
                BLOCK_SIZE >= core::mem::size_of::<FreeNode>(),
                "BLOCK_SIZE must be at least pointer sized"
            );
            assert!(
                TOTAL_SIZE % BLOCK_SIZE == 0,
                "TOTAL_SIZE must be a multiple of BLOCK_SIZE"
            );
        }
        Self {
            buffer: UnsafeCell::new([MaybeUninit::uninit(); TOTAL_SIZE]),
            free_head: AtomicPtr::new(core::ptr::null_mut()),
            state: AtomicU8::new(0),
            lock: AtomicBool::new(false),
        }
    }

    #[inline]
    fn acquire_lock(&self) {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
    }

    #[inline]
    fn release_lock(&self) {
        self.lock.store(false, Ordering::Release);
    }

    #[allow(clippy::cast_ptr_alignment)]
    fn lazy_init(&self) {
        if self.state.load(Ordering::Acquire) == 2 {
            return;
        }

        if self
            .state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            unsafe {
                let buffer_ptr = self.buffer.get().cast::<u8>();
                let mut head = core::ptr::null_mut::<FreeNode>();

                let num_blocks = TOTAL_SIZE / BLOCK_SIZE;
                for i in (0..num_blocks).rev() {
                    let block_ptr = buffer_ptr.add(i * BLOCK_SIZE).cast::<FreeNode>();
                    block_ptr.write(FreeNode {
                        next: NonNull::new(head),
                    });
                    head = block_ptr;
                }

                self.free_head.store(head, Ordering::Release);
            }
            self.state.store(2, Ordering::Release);
        } else {
            while self.state.load(Ordering::Acquire) != 2 {
                core::hint::spin_loop();
            }
        }
    }

    fn validate<T: ?Sized>(&self, token: NonNull<T>) -> Result<(), SlabError> {
        let start = self.buffer.get().addr();
        let end = start + TOTAL_SIZE;
        let ptr = token.as_ptr().addr();
        if ptr >= start && ptr < end && (ptr - start).is_multiple_of(BLOCK_SIZE) {
            Ok(())
        } else {
            core::hint::cold_path();
            Err(SlabError::InvalidToken)
        }
    }
}

impl<const BLOCK_SIZE: usize, const TOTAL_SIZE: usize> Allocator
    for SlabAllocator<BLOCK_SIZE, TOTAL_SIZE>
{
    type Error = SlabError;
    type RawToken<T: ?Sized> = NonNull<T>;
    type Token<T: ?Sized> = NonNull<T>;

    #[inline]
    unsafe fn cast<T: ?Sized, U>(&self, token: Self::RawToken<T>) -> Self::RawToken<U> {
        token.cast()
    }

    #[inline]
    fn downgrade<T: ?Sized>(&self, owned: Self::Token<T>) -> Self::RawToken<T> {
        owned
    }

    #[inline]
    fn upgrade<T: ?Sized>(&self, token: Self::RawToken<T>) -> Result<Self::Token<T>, Self::Error> {
        self.validate(token)?;
        Ok(token)
    }

    fn allocate_raw(&self, layout: Layout) -> Result<Self::Token<[MaybeUninit<u8>]>, Self::Error> {
        if layout.size() > BLOCK_SIZE || layout.align() > BLOCK_SIZE {
            core::hint::cold_path();
            return Err(SlabError::InvalidLayout);
        }

        self.lazy_init();
        self.acquire_lock();

        let head = self.free_head.load(Ordering::Relaxed);
        if head.is_null() {
            core::hint::cold_path();
            self.release_lock();
            return Err(SlabError::OutOfMemory);
        }

        unsafe {
            let next = (*head).next;
            let next_raw = next.map_or(core::ptr::null_mut(), NonNull::as_ptr);
            self.free_head.store(next_raw, Ordering::Relaxed);
            self.release_lock();

            let data_ptr = head.cast::<MaybeUninit<u8>>();
            let slice_ptr = core::ptr::slice_from_raw_parts_mut(data_ptr, layout.size());
            Ok(NonNull::new_unchecked(slice_ptr))
        }
    }

    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn deallocate_raw(
        &self,
        token: Self::RawToken<MaybeUninit<u8>>,
        _layout: Layout,
    ) -> Result<(), Self::Error> {
        self.acquire_lock();
        unsafe {
            let node_ptr = token.as_ptr().cast::<FreeNode>();
            let old_head = self.free_head.load(Ordering::Relaxed);
            node_ptr.write(FreeNode {
                next: NonNull::new(old_head),
            });
            self.free_head.store(node_ptr, Ordering::Relaxed);
            self.release_lock();
            Ok(())
        }
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
