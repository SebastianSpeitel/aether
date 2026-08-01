# Aether Architecture & Design Principles

> **Directive for AI Coding Assistants**: Always ask the user for explicit approval before performing any code or file modifications.

Aether is a high-performance, `#![no_std]` capability-based kernel and resource management architecture written in Rust. It is designed to scale seamlessly from tiny 8-bit microcontrollers (e.g. 2KB RAM AVR ATmega328P Arduino Nano) to multi-core host systems.

---

## 1. Core Architectural Principles

1. **Capability-Driven Contexts**: Kernels pass capabilities to tasks using explicit trait bounds (`HasAllocator<A>`, `HasDriver<D>`). Tasks declare their exact resource requirements at compile time.
2. **Zero-Overhead Abstractions**: Hardware pins, singletons, and static drivers use Zero-Sized Types (ZSTs: `type Handle = ()`, `type OpenOptions = ()`). They take **0 bytes of RAM** and inline down to direct hardware register assembly.
3. **Safe by Default**: Resources use owned handles that automatically clean up on `Drop` (matching `std::fs::File` and `std::os::fd::OwnedFd`). Manual `close()` functions and double-close bugs are eliminated.
4. **Compile-Time Sub-Capabilities**: Instead of runtime error codes (`ESPIPE` for illegal seek on a UART), sub-capability traits (`ReadDriver`, `WriteDriver`, `BlockDriver`, `CloneDriver`) enforce valid operations at compile time.

---

## 2. Memory Subsystem (`Allocator` & `Token`)

Memory in Aether is managed via the `Allocator` trait and `Token` abstractions:

```rust
pub trait Allocator {
    type Error: Error;
    type RawToken<T: ?Sized>: Token<T, Self, true> + Copy;
    type Token<T: ?Sized>: Token<T, Self, false>;

    // Core methods...
}
```

### Key Token & Allocator Rules:

- **Token Trait (`Token<T, A, const RAW: bool = false>`)**:
  - Blanket implementation for `RAW = true` (`A::RawToken<T>`) provides identity `as_raw()`.
  - Blanket implementation for `RAW = false` (`A::Token<T>`) downgrades via `alloc.downgrade(self)`.
  - Uses `#[diagnostic::on_unimplemented]` for human-readable compiler diagnostics.
- **Error Handling (`handle_error`)**:
  - In debug builds (`cfg(debug_assertions)`), panics with `format_args!` diagnostic context.
  - In release builds (`cfg(not(debug_assertions))`), uses `core::hint::unreachable_unchecked()` for zero overhead.
- **Unchecked Guards**:
  - `get_ref_unchecked` and `get_mut_unchecked` return `impl Guard<T> + 'a` and `impl GuardMut<T> + 'a` directly (using `&T` or `&mut T` blanket guard implementations) without heap allocation or transmutes.
- **Memory vs Device I/O Terminology**:
  - Memory references on `Allocator` use `get_ref` and `get_mut` (returning `Guard<T>` / `GuardMut<T>`).
  - Device I/O byte transfers on `Driver` use `read` and `write` (transferring byte buffers).

---

## 3. Resource & Driver Capability System (`Driver` & `Handle`)

Drivers manage hardware peripherals, character/block devices, filesystems, and network sockets:

```rust
pub trait Driver {
    type Error: Error;
    type OpenOptions;
    type Handle;

    fn open(&self, options: Self::OpenOptions) -> Result<Self::Handle, Self::Error>;
}
```

### Driver Sub-Capability Traits:

- **`ReadDriver`**: Sequential streaming reads (`read(handle, buf)`). Used for UARTs, Sockets, FIFOs, Pipes.
- **`WriteDriver`**: Sequential streaming writes (`write(handle, buf)` & `flush`). Used for UARTs, LEDs, Logs.
- **`PositionedReadDriver` & `PositionedWriteDriver`**: Positional storage I/O (`read_at`, `write_at`). Used for Files and Flash memory.
- **`BlockDriver`**: Fixed sector I/O (`sector_size`, `read_sector`, `write_sector`). Used for SD Cards, NVMe, Sector Flash.
- **`IoctlDriver`**: Control commands (`ioctl(handle, cmd, arg)`).
- **`CloneDriver`**: Handle duplication (`try_clone(handle)`), analogous to Linux `dup()` or `std::fs::File::try_clone`.

### Hardware Driver Implementations:

- **`PinDriver<P>` (`aether-platform::driver::pin`)**: Wraps any `embedded-hal` 1.0 `OutputPin` (implementing `WriteDriver`) or `InputPin` (implementing `ReadDriver`). Uses `UnsafeCell<P>` for interior mutability with **0 bytes of RAM overhead** when `P` is a ZST pin.

---

## 4. Task & Kernel Execution Model

Tasks implement `Task<K>` where `K: Kernel`:

```rust
pub trait Task<K: Kernel> {
    type Output;
    fn poll(&mut self, kernel: &K) -> Poll<Self::Output>;
}
```

### Yielding & Ergonomics:

- `Kernel::r#yield<T>(&self) -> Poll<T>` and `Kernel::yield_for<C, T>(&self, duration) -> Poll<T>` return `Poll::Pending` (`Poll<T>`) annotated with `#[must_use]`.
- Enables one-liner task yielding:
  ```rust
  // Directly yield CPU time and return Poll::Pending:
  return kernel.r#yield();
  ```
- The `#[must_use]` attribute causes the Rust compiler to emit a warning if a developer calls `kernel.r#yield()` and forgets to return from `poll()`.

---

## 5. Bare-Metal & Hardware Constraints

- **AVR Constraints (ATmega328P / Arduino Nano)**:
  - AVR has no 16-bit/32-bit hardware atomic pointer instructions (`AtomicPtr`, `AtomicUsize`).
  - Interrupt safety on AVR uses `avr_device::interrupt::free` or single-byte atomic indices (`u8`).
- **Progmem Utility (`aether-platform::progmem`)**:
  - Provides `read_byte`, `PStr`, and `ProgPtr` for reading constants from AVR Flash (`PROGMEM`) memory.
