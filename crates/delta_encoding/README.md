# `delta_encoding`

A real-time, zero-allocation delta compression library for embedded sensor streams and ring buffers.

Designed for **hard real-time embedded systems** (`#![no_std]` compatible), this crate provides sample-by-sample compression using arbitrary bit-width primitives and adaptive keyframing — with no heap allocations, no block buffering, and no decoder changes required for denoising.

---

## Overview

The crate provides two complementary encoding strategies, unified under the [`Encoding`](#the-encoding-trait) trait:

### `DiffEncoding<T, F>` — Zero-Order Delta Encoding

Compresses a signal by storing differences relative to the previous value:

$$\Delta_i = X_i - X_{i-1}$$

If $\Delta_i$ fits within the $F$-bit delta field, a compact **deltaframe** is emitted.  
If it overflows, a full-value **keyframe** is emitted at a slight overhead cost.

Best for: **flat, slowly-varying, or step-change signals** (e.g. status flags, slow sensors).

**Frame layout (`DiffEncoding<T, F>`):**

```
Deltaframe: [ F bits: encoded delta ]
Keyframe:   [ F bits: all-ones flag | T bits: full value ]
```

The encoded delta is `(Δ + MAX_DELTA)` stored unsigned in `F` bits.  
All-ones in the flag field (`KEY_FLAG = 2^F - 1`) signals a keyframe.

### `GradientEncoding<T, F, V>` — First-Order Velocity Tracking

Predicts the next sample using a tracked velocity $V_i$:

$$\text{base}_i = X_{i-1} + V_{i-1}$$
$$\Delta_{\text{grad},i} = X_i - \text{base}_i$$

The residual is typically **much smaller** than a raw difference, allowing tight delta fields to survive fast-moving signals. On constant ramps or smooth curves, the residual is exactly **0**, enabling near-perfect compression.

Best for: **linear ramps, parabolic arcs, smooth sensor dynamics** (e.g. temperature, pressure, position).

**Frame layout (`GradientEncoding<T, F, V>`):**

```
Deltaframe: [ F bits: encoded grad-residual ]
Keyframe:   [ F bits: all-ones flag | T bits: full value | V bits: new velocity ]
```

The velocity field stores the signed raw difference between the current and previous value, clamped to the representable range of `V`.

### `GradientEncodingU10` — Convenience Alias

```rust
pub type GradientEncodingU10 = GradientEncoding<U10, U2, U6>;
```

A pre-tuned alias for 10-bit ADC streams. It uses a 2-bit delta field (±1 residual) and a 6-bit velocity field (±31 steps/sample). This alias achieves **4.00x compression** on noisy 10-bit ADC data with `push_denoised::<16>()`.

---

## What Makes This Different

### 1. Zero Latency, Constant Memory (`O(1)` Space & Time)

General-purpose compressors (Zstd, Gzip, LZ4) buffer full blocks before compressing, introducing variable latency and requiring kilobytes of dictionary state.

This crate compresses **sample-by-sample in O(1) time** with **zero allocations**. It is safe to call from MCU interrupt handlers (ARM Cortex-M, RISC-V, AVR).

### 2. Superior Compression on Dynamic Slopes

Traditional zero-order delta encoding overflows keyframes on any fast-rising signal (e.g. temperature at +5°C/s), requiring wide delta fields and eating into compression ratios.

`GradientEncoding` tracks velocity momentum, so constant ramps yield a residual of **exactly 0** — compressing smooth dynamics down to **2.50 BPS (5.81x compression)** with only a 2-bit delta field.

### 3. Built-In Lossless or Lossy Denoising

ADC noise ($\pm 2$–$\pm 16$ LSB counts) typically breaks lossless compression, forcing frequent keyframes and causing bitstream **expansion**.

The crate provides a deadband denoising filter at the strategy level via `push_denoised::<DEADBAND>()`:

- Residuals within $[\text{MIN\_DELTA} - N,\ \text{MAX\_DELTA} + N]$ are clamped into valid delta range.
- **The decoder requires no changes** — denoised samples emit fully standard deltaframes.
- On noisy 10-bit ADC data, compression improves from ~0.77x (expansion) to **4.00x**.

### 4. Non-Byte-Aligned Sub-Bit Field Packing

Standard compressors are bound to 8-, 16-, or 32-bit boundaries. This crate uses custom primitives `U1`..`U63` to pack delta fields at exact bit widths (e.g. a 2-bit flag followed by a 10-bit residual) directly across byte boundaries using 128-bit shift registers with zero padding waste.

### 5. Circular Buffer Compatibility

Standard archives cannot drop early samples from a ring buffer without full recompression. `EncodedRing` maintains keyframe boundary metadata, so when the ring is full it drops old keyframe blocks at bit boundaries while keeping the remainder of the stream fully decodable.

---

## Comparison

| Feature / Metric | **`delta_encoding`** | **Gorilla (FB TSDB)** | **Zstd / Gzip** | **Classic Delta** |
| :--- | :--- | :--- | :--- | :--- |
| Real-Time Stream Push | **Sample-by-sample O(1)** | Block/Chunk based | Large block buffering | Sample-by-sample |
| Memory Footprint | **~0 bytes (`#![no_std]`)** | KBs per stream | 64 KB – 8 MB window | ~0 bytes |
| Ramp/Slope Tracking | **Velocity momentum (5.8x)** | Poor on ramps | Good (dictionary) | Poor (overflows) |
| ADC Noise Resilience | **Built-in deadband (4.0x)** | Expands on noise | Expands on noise | Expands on noise |
| Arbitrary Bit Primitives | **`U1`..`U63` native** | XOR float bits | Byte-aligned | Integer-aligned |
| Circular Buffer Support | **Yes (keyframe boundaries)** | No | No | No |

---

## Benchmark Highlights

Benchmarks run on 10,000 samples across 8 datasets (Constant, Linear Ramp, Sine Wave, Random Walk, each with and without noise). See [`BENCHMARK_RESULTS.md`](./BENCHMARK_RESULTS.md) for the full table.

### Top Results

| Dataset | Encoding Scheme | Compression Ratio |
| :--- | :--- | :--- |
| Constant | `DiffEncoding<u8, U1>` | **6.40x** |
| Constant | `DiffEncoding<u16, U2>` | **6.40x** |
| Linear Ramp | `GradientEncoding<u16, U2, u8>` | **5.81x** |
| Linear Ramp | `GradientEncoding<U63, U24, U32>` | **4.45x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | **4.00x** |
| Constant | `GradientEncoding<U63, U12, u16>` | **4.35x** |
| Random Walk | `DiffEncoding<u8, U4> [Denoise=2]` | **1.13x** |

> Ratios below **0.80x** are excluded from benchmark output as not worth using.

### Key Design Tradeoffs

| Encoding | Optimal Signal | Key Driver |
| :--- | :--- | :--- |
| `DiffEncoding` | Flat / slowly varying | Near-zero consecutive differences |
| `GradientEncoding` | Linear ramps, smooth arcs | Velocity tracking eliminates residuals |
| Wide delta (`U18`..`U30`) | High-range 32/63-bit telemetry | Replaces 64-bit writes with 18–30 bit deltaframes |
| `[Denoise=N]` | Noisy ADC sensor streams | Prevents LSB jitter from triggering keyframe cascades |

---

## Limitations

- **Uncorrelated / Random Data**: Performs poorly (expansion of 1.1x–1.5x) on data with no temporal correlation such as binary files, encrypted streams, or uniformly random bytes. Use a general-purpose compressor (Zstd, LZ4) for such data.
- **Temporal Only**: This is a 1D time-series compressor. It exploits temporal correlation only — no spatial or frequency-domain compression.
- **Not a General Archive Format**: Does not produce a self-describing binary format. The decoder must know the exact `Encoding` type and bit-width parameters used at encode time.
- **Power-of-Two Buffer Size**: The underlying `BitRing<N>` requires `N` to be a power of two. `EncodedRing<N, E>` inherits this constraint. `N = 65536` (64 KiB) is a typical choice.
- **Forced Keyframe Interval**: A keyframe is forced every 32 deltaframes regardless of signal behavior. This bounds seek cost but adds a small overhead on very long constant runs.

---

## Usage

```toml
[dependencies]
delta_encoding = { path = "../delta_encoding" }
```

### Encoding into an `EncodedRing`

```rust
use delta_encoding::{DiffEncoding, GradientEncoding, EncodedRing};
use delta_encoding::primitive::U2;

// 64 KiB ring buffer, gradient-encoded u16 ADC stream, 2-bit delta field
let mut ring = EncodedRing::<65536, GradientEncoding<u16, U2, u8>>::new();

ring.push(1023);
ring.push(1024);
ring.push(1025);

// Raw ADC reads 1029 (grad_diff=+3, overflows the ±1 delta field).
// DEADBAND=4 clamps +3 → +1: emits a deltaframe instead of a keyframe.
// Decoded value is 1027, not 1029 — lossy by design.
ring.push_denoised::<4>(1029);

// Iterate all retained samples (oldest keyframe block to newest)
for sample in ring.iter() {
    // sample: u16
}

// EncodedRing also implements Extend
ring.extend([1026u16, 1027, 1028]);
```

### Low-Level Encode / Decode

Use `Encoding::encode` and `Encoding::decode` directly when you manage your own `BitRing`:

```rust
use delta_encoding::encoding::{DiffEncoding, Encoding};
use delta_encoding::bitring::BitRing;
use delta_encoding::primitive::U4;

// 8-byte ring = 64 bits
let mut ring = BitRing::<8>::new();
let mut enc_state: u8 = 0;

// Encode three u8 samples with 4-bit delta fields
DiffEncoding::<u8, U4>::encode(100, true,  &mut enc_state, &mut ring);
DiffEncoding::<u8, U4>::encode(103, false, &mut enc_state, &mut ring);
DiffEncoding::<u8, U4>::encode(106, false, &mut enc_state, &mut ring);

// Decode them back
let mut reader = ring.iter(0);
let mut dec_state: u8 = 0;
let (v0, _) = DiffEncoding::<u8, U4>::decode(&mut reader, &mut dec_state); // 100
let (v1, _) = DiffEncoding::<u8, U4>::decode(&mut reader, &mut dec_state); // 103
let (v2, _) = DiffEncoding::<u8, U4>::decode(&mut reader, &mut dec_state); // 106
```

---

## Type Parameter Reference

### `DiffEncoding<T, F>`

| Parameter | Meaning | Constraints |
| :--- | :--- | :--- |
| `T` | Value type (e.g. `u8`, `u16`, `U10`) | Must implement `Primitive` |
| `F` | Delta field width (e.g. `U2`, `U4`) | Must implement `Primitive`; `F::BITS ≥ 1` |

Delta range: `MIN_DELTA = -(2^(F::BITS-1) - 1)`, `MAX_DELTA = 2^(F::BITS-1) - 1`.

### `GradientEncoding<T, F, V>`

| Parameter | Meaning | Constraints |
| :--- | :--- | :--- |
| `T` | Value type | Must implement `Primitive` |
| `F` | Grad-residual field width | Must implement `Primitive`; `F::BITS ≥ 1` |
| `V` | Velocity field width (keyframe only) | Must implement `Primitive`; `V::BITS ≥ 1` |

Velocity range: `-(2^(V::BITS-1) - 1)` to `2^(V::BITS-1) - 1`.  
Keyframe cost: `F::BITS + T::BITS + V::BITS` bits.  
Deltaframe cost: `F::BITS` bits.

---

## The `Encoding` Trait

```rust
pub trait Encoding {
    type Value: Copy;
    type State: Default + Copy;

    const MAX_BITS: usize;   // worst-case bits per sample (keyframe size)
    const KEY_FLAG: usize;   // all-ones pattern in F bits
    const MIN_DELTA: isize;
    const MAX_DELTA: isize;

    fn encode<const N: usize>(value, force_keyframe, state, writer) -> bool /* is_keyframe */;
    fn decode<const N: usize>(reader, state) -> (Value, bool /* is_keyframe */);
    fn is_keyframe(reader) -> bool;
    fn denoise<const DEADBAND: usize>(value, state) -> Value;
}
```

Implement this trait to define a custom encoding strategy compatible with `EncodedRing`.

---

## Primitive Types

The crate exposes newtype wrappers `U2`..`U63` (e.g. `U10(val: u16)`) for sub-byte-aligned bit fields. All implement the `Primitive` trait, which provides wrapping arithmetic and signed-difference conversion across byte boundaries.

Standard Rust integer types `bool` (`U1`), `u8`, and `u16` also implement `Primitive` directly.
