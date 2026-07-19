# Aether

Aether is an experimental Rust project exploring custom memory allocators tailored for performance, thread safety, and token-based resource management.

## Overview

Aether provides fundamental building blocks for specialized allocation strategies in contexts where `std::alloc::GlobalAlloc` might not be sufficient or where distinct memory regions are required. It introduces an `Allocator` trait that uses abstract `Token` types to track and validate memory regions safely.

### Crates

- **`aether-core`**: Defines the core abstractions, including the `Allocator` trait, `AllocError`, and the `Guard` / `GuardMut` traits for scoped access to memory.
- **`aether-platform`**: Provides concrete implementations of the allocator abstractions:
  - **`ArenaAllocator`**: A fast, thread-safe bump allocator. It uses atomic counters (`AtomicUsize`) for lock-free allocation from a contiguous block of memory. Ideal for short-lived, phase-based allocations.
  - **`SlabAllocator`**: A thread-safe allocator for fixed-size blocks. Uses an atomic free list and a spinlock to manage contention, providing extremely fast allocation and deallocation of uniform types.

## Features

- **Token-based Allocation**: Allocators return abstract tokens rather than raw pointers, which can then be validated and upgraded to references. This approach helps encapsulate the memory location and bounds.
- **Thread Safety**: The provided `ArenaAllocator` and `SlabAllocator` are `Sync` and use atomics for safe concurrent access across threads.
- **Fast Error Paths**: Uses `core::hint::cold_path()` to inform the compiler about unlikely error conditions, optimizing the instruction cache for the hot paths.
- **Zero-Sized Errors**: `AllocError` is a zero-sized type (ZST) to ensure `Result<Token, AllocError>` takes no more space than a pointer.

## Architecture

At its core, Aether models memory access with the `Allocator` trait:

```rust
pub trait Allocator<T> {
    type Token;
    type RawToken;

    fn allocate(&self, value: T) -> Result<Self::Token, AllocError>;
    fn deallocate(&self, token: Self::RawToken) -> Result<(), AllocError>;
    
    fn read<'a>(&'a self, token: Self::RawToken) -> Result<impl Guard<'a, T>, AllocError>;
    fn write<'a>(&'a self, token: Self::RawToken) -> Result<impl GuardMut<'a, T>, AllocError>;
    fn upgrade<'a>(&'a self, token: Self::RawToken) -> Result<Self::Token, AllocError>;
}
```

This model separates allocation (getting a token), access (read/write guards), and deallocation. It allows allocators to enforce strict validation rules before allowing access to the underlying memory.
