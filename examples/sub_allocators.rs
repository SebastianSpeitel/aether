use core::alloc::Layout;
use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::task::Poll;

use aether_core::allocator::{Allocator, HasAllocator};
use aether_core::guard::{Guard, GuardMut};
use aether_core::kernel::Kernel;
use aether_core::task::Task;
use aether_core::time::Duration;

// -----------------------------------------------------------------------------
// 1. Sub-Allocator: A lightweight arena carved out for a single task execution
// -----------------------------------------------------------------------------

// 1. Sub-Allocator: A lightweight arena created by the kernel allocator
// -----------------------------------------------------------------------------

pub struct TaskSubArena {
    buffer: UnsafeCell<[u8; 256]>,
    offset: Cell<usize>,
}

impl TaskSubArena {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0u8; 256]),
            offset: Cell::new(0),
        }
    }

    #[must_use]
    pub const fn used_bytes(&self) -> usize {
        self.offset.get()
    }
}

impl Default for TaskSubArena {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct AllocError;
impl core::fmt::Display for AllocError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Sub-arena out of memory")
    }
}
impl core::error::Error for AllocError {}

impl Allocator for TaskSubArena {
    type Error = AllocError;
    type RawToken<T: ?Sized> = NonNull<T>;
    type Token<T: ?Sized> = NonNull<T>;

    unsafe fn cast<T: ?Sized, U>(&self, token: Self::RawToken<T>) -> Self::RawToken<U> {
        token.cast()
    }

    fn downgrade<T: ?Sized>(&self, owned: &Self::Token<T>) -> Self::RawToken<T> {
        *owned
    }

    fn upgrade<T: ?Sized>(&self, token: Self::RawToken<T>) -> Result<Self::Token<T>, Self::Error> {
        Ok(token)
    }

    fn allocate_raw(&self, layout: Layout) -> Result<Self::Token<[MaybeUninit<u8>]>, Self::Error> {
        let current = self.offset.get();
        let align = layout.align();
        let aligned_offset = (current + align - 1) & !(align - 1);
        let end = aligned_offset + layout.size();

        let slice_ptr = self.buffer.get();
        if end > 256 {
            return Err(AllocError);
        }

        self.offset.set(end);
        let ptr = unsafe { (slice_ptr.cast::<u8>()).add(aligned_offset) };
        let raw_slice = core::ptr::slice_from_raw_parts_mut(ptr.cast::<MaybeUninit<u8>>(), layout.size());

        NonNull::new(raw_slice).ok_or(AllocError)
    }

    unsafe fn deallocate_raw(
        &self,
        _token: Self::RawToken<MaybeUninit<u8>>,
        _layout: Layout,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn read_raw<'b, T: ?Sized + 'b>(
        &'b self,
        token: Self::RawToken<T>,
    ) -> Result<impl Guard<T> + 'b, Self::Error> {
        unsafe { Ok(token.as_ref()) }
    }

    fn write_raw<'b, T: ?Sized + 'b>(
        &'b self,
        token: Self::RawToken<T>,
    ) -> Result<impl GuardMut<T> + 'b, Self::Error> {
        unsafe { Ok(&mut *token.as_ptr()) }
    }

    unsafe fn read_unchecked<'b, T: ?Sized + 'b>(
        &'b self,
        token: Self::RawToken<T>,
    ) -> impl Guard<T> + 'b {
        unsafe { token.as_ref() }
    }

    unsafe fn write_unchecked<'b, T: ?Sized + 'b>(
        &'b self,
        token: Self::RawToken<T>,
    ) -> impl GuardMut<T> + 'b {
        unsafe { &mut *token.as_ptr() }
    }
}

use aether_platform::allocator::ArenaAllocator;

// -----------------------------------------------------------------------------
// 2. Real-Time Kernel implementing HasAllocator with aether_platform::allocator::ArenaAllocator
// -----------------------------------------------------------------------------

pub struct RealTimeKernel {
    allocator: ArenaAllocator<1024>,
}

impl RealTimeKernel {
    pub fn new() -> Self {
        Self {
            allocator: ArenaAllocator::new(),
        }
    }
}

impl Default for RealTimeKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl Kernel for RealTimeKernel {
    fn yield_for<C: aether_core::time::Clock, T>(&self, _dur: Duration<C>) -> Poll<T> {
        Poll::Pending
    }
}

impl HasAllocator for RealTimeKernel {
    type Alloc<'a> = &'a ArenaAllocator<1024> where Self: 'a;

    fn get_allocator(&self) -> Self::Alloc<'_> {
        &self.allocator
    }
}

// -----------------------------------------------------------------------------
// 4. Task holding a Sub-Allocator Token (allocated via Kernel Allocator)
// -----------------------------------------------------------------------------

struct PacketHeader {
    packet_id: u32,
    _payload_len: usize,
}

/// Task demonstrating:
/// - Single-poll temporary values stay on the local stack
/// - Task persistent struct fields store inline state across polls
/// - Sub-allocator `TaskSubArena` is allocated via `kernel.get_allocator().new(...)`
/// - The resulting `Token<TaskSubArena>` is stored in the Task struct state
/// - Inside `poll()`, `kernel_alloc.write(raw)` upgrades the token into a `GuardMut` to perform sub-allocations!
struct PacketProcessingTask<Tok> {
    step: usize,
    header: PacketHeader,
    // Safe owned Token holding the Sub-Allocator allocated inside the Kernel's main allocator!
    sub_arena_token: Option<Tok>,
}

impl<K, Tok> Task<K> for PacketProcessingTask<Tok>
where
    K: Kernel + HasAllocator,
    Tok: 'static,
    for<'a> <K as HasAllocator>::Alloc<'a>: core::ops::Deref<Target: Allocator<Token<TaskSubArena> = Tok>>,
{
    type Output = u32;

    fn poll(&mut self, kernel: &K) -> Poll<Self::Output> {
        self.step += 1;
        println!("\n--- [Task] Polling PacketProcessingTask (Step {}) ---", self.step);

        let kernel_alloc = kernel.get_allocator();

        if self.step == 1 {
            println!("  [Task] Allocating SubAllocator (TaskSubArena) via Kernel Allocator...");

            // 1. Allocate the Sub-Allocator using the Kernel's main allocator, returning an owned Token
            let sub_arena_token = kernel_alloc
                .new(TaskSubArena::new())
                .expect("Failed to allocate Sub-Allocator in Kernel");

            // 2. Store the safe owned Token inside the Task state
            self.sub_arena_token = Some(sub_arena_token);

            println!("  [Task] SubAllocator allocated and owned Token stored in Task state.");
            Poll::Pending
        } else if self.step == 2 {
            println!("  [Task] Upgrading SubAllocator Token to GuardMut via Kernel Allocator...");

            if let Some(ref sub_arena_token) = self.sub_arena_token {
                // 3. Directly pass owned Token to write() via auto-downgrade
                let arena_guard = kernel_alloc
                    .write::<TaskSubArena, _>(sub_arena_token)
                    .expect("Failed to get GuardMut for SubAllocator");

                println!("  [Task] SubAllocator GuardMut acquired! Using SubAllocator to allocate long-living payload...");

                // 4. Use the acquired GuardMut (which derefs to TaskSubArena) to allocate long-living dynamic payload
                let _payload_token = arena_guard
                    .new([0xABu8; 64])
                    .expect("Failed to allocate payload in SubAllocator");

                println!(
                    "  [Task] Long-living payload allocated in SubAllocator (used bytes: {})",
                    arena_guard.used_bytes()
                );
            }

            Poll::Pending
        } else {
            println!(
                "  [Task] Completing Task. Struct field Header Packet ID: 0x{:X}",
                self.header.packet_id
            );
            Poll::Ready(self.header.packet_id)
        }
    }
}

// -----------------------------------------------------------------------------
// 5. Main Execution
// -----------------------------------------------------------------------------

fn main() {
    println!("=== Aether Sub-Allocator Example ===");

    let kernel = RealTimeKernel::new();
    let mut task = PacketProcessingTask {
        step: 0,
        header: PacketHeader {
            packet_id: 0xDEAD_BEEF,
            _payload_len: 64,
        },
        sub_arena_token: None,
    };

    // Poll Turn 1: Allocates SubAllocator in Kernel
    let poll1 = task.poll(&kernel);
    assert_eq!(poll1, Poll::Pending);

    // Poll Turn 2: Upgrades SubAllocator Token to GuardMut & allocates payload
    let poll2 = task.poll(&kernel);
    assert_eq!(poll2, Poll::Pending);

    // Poll Turn 3: Completion
    let poll3 = task.poll(&kernel);
    assert_eq!(poll3, Poll::Ready(0xDEAD_BEEF));

    println!("\n=== Execution Completed Successfully ===");
}
