# `delta_encoding`

A real-time, zero-allocation delta compression library for embedded sensor streams and ring buffers.

Designed for **hard real-time embedded systems** (`#![no_std]` compatible), this crate provides sample-by-sample compression using arbitrary bit-width primitives and adaptive keyframing — with no heap allocations, no block buffering, and no decoder changes required for denoising.

---

## Overview

The crate provides two complementary encoding strategies, unified under the `Encoding` trait:

### `DiffEncoding<T, F>` — Zero-Order Delta Encoding

Compresses a signal by storing differences relative to the previous value:

$$\Delta_i = X_i - X_{i-1}$$

If $\Delta_i$ fits within the $F$-bit delta field, a compact **deltaframe** is emitted.  
If it overflows, a full-value **keyframe** is emitted at a slight overhead cost.

Best for: **flat, slowly-varying, or step-change signals** (e.g. status flags, slow sensors).

### `GradientEncoding<T, F, V>` — First-Order Velocity Tracking

Predicts the next sample using a tracked velocity $V_i$:

$$\text{base}_i = X_{i-1} + V_{i-1}$$
$$\Delta_{\text{grad},i} = X_i - \text{base}_i$$

The residual is typically **much smaller** than a raw difference, allowing tight delta fields to survive fast-moving signals. On constant ramps or smooth curves, the residual is exactly **0**, enabling near-perfect compression.

Best for: **linear ramps, parabolic arcs, smooth sensor dynamics** (e.g. temperature, pressure, position).

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

- Residuals within $[\text{MIN\_DELTA} - N, \text{MAX\_DELTA} + N]$ are clamped into valid delta range.
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

Benchmarks are run on 10,000 samples across 8 datasets (Constant, Linear Ramp, Sine Wave, Random Walk, each with and without noise). See [`BENCHMARK_RESULTS.md`](./BENCHMARK_RESULTS.md) for the full table.

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
- **Not a General Archive Format**: Does not produce a self-describing binary format. The decoder must know the exact `Encoding` type and bit-width parameters used.

---

## Usage

```toml
[dependencies]
delta_encoding = { path = "../delta_encoding" }
```

```rust
use delta_encoding::{DiffEncoding, GradientEncoding, EncodedRing};
use delta_encoding::primitive::U2;

// Lossless compression of a u10 ADC stream with 2-bit delta fields
let mut ring = EncodedRing::<65536, GradientEncoding<u16, U2, u8>>::new();
ring.push(1023);
ring.push(1024);
ring.push(1025);

// Noisy ADC: clamp residuals within ±4 LSB into valid delta range
ring.push_denoised::<4>(1027); // noise spike clamped to +2
```
