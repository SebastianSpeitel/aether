# Aether

Aether is a high-performance, `#![no_std]` capability-based kernel, memory allocator, and hardware resource management architecture written in Rust. It scales from 8-bit microcontrollers (e.g. 2KB RAM AVR ATmega328P Arduino Nano) to multi-core host OS kernels.

> For deep architectural details, design rules, and guidelines for AI agents, see [ARCHITECTURE.md](ARCHITECTURE.md).

## Core Crates

- **`aether-core`**: Defines the foundational abstractions:
  - **`Allocator` & `Token`**: Abstract memory allocation with scoped `Guard` / `GuardMut` access (`get_ref`, `get_mut`) and zero-overhead debug error handling.
  - **`Driver` & Sub-Capabilities**: Resource management via safe owned handles and compile-time sub-capability traits (`ReadDriver`, `WriteDriver`, `PositionedReadDriver`, `PositionedWriteDriver`, `BlockDriver`, `IoctlDriver`, `CloneDriver`).
  - **`Clock` & Time Capabilities**: Top-level capability system for hardware time sources (`Clock`, `Instant<C>`, `Duration<C>`, `SignedDuration<C>`, `HasClock<C>`).
  - **`Kernel` & `Task`**: Capability-driven task execution with ergonomic, compiler-checked `Poll<T>` yielding (`return kernel.r#yield();`).
- **`aether-platform`**: Platform-specific allocators, drivers, and utilities:
  - **`ArenaAllocator` & `SlabAllocator`**: Thread-safe, lock-free bump and fixed-block allocators.
  - **`PinDriver<P>`**: Universal `embedded-hal` 1.0 GPIO pin wrapper (0-byte RAM ZST).
  - **`SystemClock`**: Hardware tick clock capability implementation.
  - **`progmem`**: AVR Flash memory utilities (`read_byte`, `PStr`, `ProgPtr`).
- **`aether-compat`**: Compatibility layers bridging `aether::Task` to standard Rust `std::future::Future` and `WakerKernel`.

## Features & Design Highlights

1. **Capability-Based OS Architecture**: Kernel passes capabilities to tasks using generic trait bounds (`HasAllocator`, `HasDriver<D>`, `HasClock<C>`).
2. **Zero-Overhead / Bare-Metal Safety**: Zero-Sized Types (ZSTs: `type Handle = ()`, `type OpenOptions = ()`) take **0 bytes of RAM** and inline directly to hardware register writes.
3. **Safe by Default**: Driver handles automatically clean up on `Drop`, matching `std::fs::File` and `std::os::fd::OwnedFd`.
4. **Compile-Time Sub-Capabilities**: Enforces valid operations at compile-time (e.g. attempting to seek a UART or write to a button fails at compile time).
5. **Standard Library Adapters**: Bridge `ReadDriver` and `WriteDriver` handles directly to `std::io::Read` and `std::io::Write` using `std::io::Error::other`.

## Examples

Run any of the included workspace examples:

```bash
# Embedded PinDriver (Button + LED) ZST Example:
cargo run -p aether-platform --example embedded_button_led --features embedded

# Sub-Allocator Arena Example:
cargo run --example sub_allocators

# Futures Compatibility Layer Example:
cargo run -p aether-compat --example compat_futures
```
