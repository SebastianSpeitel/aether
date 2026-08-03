# Encoding Benchmark Size Results

## Legend & Encoding Scheme Conventions

- **`DiffEncoding<T, F>`**: Difference (zero-order) delta encoding.
  - `T`: Raw sample value type (e.g. `u8`, `U10`, `u16`, `U32`, `U63`).
  - `F`: Flag/delta bit-width (e.g. `U1` = 1 bit, `U2` = 2 bits, `U4` = 4 bits). `F::KEY_FLAG` indicates a full keyframe write.
- **`GradientEncoding<T, F, V>`**: First-order (velocity tracking) gradient encoding.
  - `T`: Raw sample value type.
  - `F`: Residual difference flag bit-width.
  - `V`: Signed velocity bit-width stored inside keyframes.
- **`[Denoise=N]`**: Deadband filtering threshold `N` LSB counts.
  - Clamps residual step differences within `[MIN_DELTA - N, MAX_DELTA + N]` into valid `[MIN_DELTA, MAX_DELTA]` encoding steps.

| Dataset | Encoding Scheme | Sample Count | Uncompressed Size | Compressed Size | Bits Per Sample | Compression Ratio |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Constant | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 12504 bits | 1.25 BPS | **6.40x** |
| Constant | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Constant | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Constant | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Constant | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Constant | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Constant | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Constant | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 25008 bits | 2.50 BPS | **6.40x** |
| Constant | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 25008 bits | 2.50 BPS | **6.40x** |
| Constant | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Constant | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Constant | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Constant | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Constant | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 27536 bits | 2.75 BPS | **5.81x** |
| Constant | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 27536 bits | 2.75 BPS | **5.81x** |
| Constant | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 27536 bits | 2.75 BPS | **5.81x** |
| Constant | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 47536 bits | 4.75 BPS | **3.37x** |
| Constant | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 47536 bits | 4.75 BPS | **3.37x** |
| Constant | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 47536 bits | 4.75 BPS | **3.37x** |
| Constant | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 47536 bits | 4.75 BPS | **3.37x** |
| Constant | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 50016 bits | 5.00 BPS | **6.40x** |
| Constant | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 50016 bits | 5.00 BPS | **6.40x** |
| Constant | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 69719 bits | 6.97 BPS | **9.04x** |
| Constant | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 69719 bits | 6.97 BPS | **9.04x** |
| Constant | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 42294 bits | 4.23 BPS | **14.90x** |
| Constant | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 42294 bits | 4.23 BPS | **14.90x** |
| Constant + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 82456 bits | 8.25 BPS | **0.97x** |
| Constant + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 37256 bits | 3.73 BPS | **2.15x** |
| Constant + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Constant + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 115312 bits | 11.53 BPS | **0.69x** |
| Constant + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Constant + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Constant + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Constant + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Constant + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 26368 bits | 2.64 BPS | **3.79x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant + Noise | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 150400 bits | 15.04 BPS | **0.66x** |
| Constant + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 64816 bits | 6.48 BPS | **1.54x** |
| Constant + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Constant + Noise | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Constant + Noise | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Constant + Noise | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Constant + Noise | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Constant + Noise | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Constant + Noise | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Constant + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Constant + Noise | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Constant + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Constant + Noise | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Linear Ramp | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 14080 bits | 1.41 BPS | **5.68x** |
| Linear Ramp | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25008 bits | 2.50 BPS | **3.20x** |
| Linear Ramp | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25008 bits | 2.50 BPS | **3.20x** |
| Linear Ramp | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Linear Ramp | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Linear Ramp | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Linear Ramp | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 45008 bits | 4.50 BPS | **2.22x** |
| Linear Ramp | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 45008 bits | 4.50 BPS | **2.22x** |
| Linear Ramp | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 27536 bits | 2.75 BPS | **5.81x** |
| Linear Ramp | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 27536 bits | 2.75 BPS | **5.81x** |
| Linear Ramp | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 132392 bits | 13.24 BPS | **1.21x** |
| Linear Ramp | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 47512 bits | 4.75 BPS | **3.37x** |
| Linear Ramp | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 47512 bits | 4.75 BPS | **3.37x** |
| Linear Ramp | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 47512 bits | 4.75 BPS | **3.37x** |
| Linear Ramp | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 47512 bits | 4.75 BPS | **3.37x** |
| Linear Ramp | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Linear Ramp | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Linear Ramp | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Linear Ramp | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 84968 bits | 8.50 BPS | **0.94x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 37744 bits | 3.77 BPS | **2.12x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 119776 bits | 11.98 BPS | **0.67x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 27504 bits | 2.75 BPS | **2.91x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179920 bits | 17.99 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 179920 bits | 17.99 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 199920 bits | 19.99 BPS | **0.80x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 199920 bits | 19.99 BPS | **0.80x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 130576 bits | 13.06 BPS | **0.77x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 130432 bits | 13.04 BPS | **0.77x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 130352 bits | 13.04 BPS | **0.77x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 130192 bits | 13.02 BPS | **0.77x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 150048 bits | 15.00 BPS | **0.67x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 66384 bits | 6.64 BPS | **1.51x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 259976 bits | 26.00 BPS | **0.62x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 259976 bits | 26.00 BPS | **0.62x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 259928 bits | 25.99 BPS | **0.62x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 279976 bits | 28.00 BPS | **0.57x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 279976 bits | 28.00 BPS | **0.57x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 279928 bits | 27.99 BPS | **0.57x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 279928 bits | 27.99 BPS | **0.57x** |
| Linear Ramp + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 359840 bits | 35.98 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 359840 bits | 35.98 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 679685 bits | 67.97 BPS | **0.93x** |
| Linear Ramp + Noise | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 679685 bits | 67.97 BPS | **0.93x** |
| Linear Ramp + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 729929 bits | 72.99 BPS | **0.86x** |
| Linear Ramp + Noise | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 729929 bits | 72.99 BPS | **0.86x** |
| Sine Wave | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 61424 bits | 6.14 BPS | **1.30x** |
| Sine Wave | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22600 bits | 2.26 BPS | **3.54x** |
| Sine Wave | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Sine Wave | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Sine Wave | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Sine Wave | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179248 bits | 17.92 BPS | **0.89x** |
| Sine Wave | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 178080 bits | 17.81 BPS | **0.90x** |
| Sine Wave | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 197072 bits | 19.71 BPS | **0.81x** |
| Sine Wave | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 195888 bits | 19.59 BPS | **0.82x** |
| Sine Wave | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25040 bits | 2.50 BPS | **3.99x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Sine Wave | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Sine Wave | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 254336 bits | 25.43 BPS | **0.63x** |
| Sine Wave | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 200792 bits | 20.08 BPS | **0.80x** |
| Sine Wave | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 193568 bits | 19.36 BPS | **0.83x** |
| Sine Wave | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 184768 bits | 18.48 BPS | **0.87x** |
| Sine Wave | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 184768 bits | 18.48 BPS | **0.87x** |
| Sine Wave | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 184768 bits | 18.48 BPS | **0.87x** |
| Sine Wave | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 184000 bits | 18.40 BPS | **0.87x** |
| Sine Wave | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Sine Wave | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Sine Wave | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Sine Wave | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Sine Wave | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Sine Wave | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Sine Wave + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 69032 bits | 6.90 BPS | **1.16x** |
| Sine Wave + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 48896 bits | 4.89 BPS | **1.64x** |
| Sine Wave + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22632 bits | 2.26 BPS | **3.53x** |
| Sine Wave + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 121968 bits | 12.20 BPS | **0.66x** |
| Sine Wave + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 28048 bits | 2.80 BPS | **2.85x** |
| Sine Wave + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179216 bits | 17.92 BPS | **0.89x** |
| Sine Wave + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 177616 bits | 17.76 BPS | **0.90x** |
| Sine Wave + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 196240 bits | 19.62 BPS | **0.82x** |
| Sine Wave + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 194528 bits | 19.45 BPS | **0.82x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 130400 bits | 13.04 BPS | **0.77x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 31984 bits | 3.20 BPS | **3.13x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave + Noise | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 150400 bits | 15.04 BPS | **0.66x** |
| Sine Wave + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 66592 bits | 6.66 BPS | **1.50x** |
| Sine Wave + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 244688 bits | 24.47 BPS | **0.65x** |
| Sine Wave + Noise | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 231464 bits | 23.15 BPS | **0.69x** |
| Sine Wave + Noise | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 230216 bits | 23.02 BPS | **0.69x** |
| Sine Wave + Noise | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 251248 bits | 25.12 BPS | **0.64x** |
| Sine Wave + Noise | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 251056 bits | 25.11 BPS | **0.64x** |
| Sine Wave + Noise | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 250408 bits | 25.04 BPS | **0.64x** |
| Sine Wave + Noise | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 249592 bits | 24.96 BPS | **0.64x** |
| Sine Wave + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Sine Wave + Noise | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Sine Wave + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Sine Wave + Noise | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Sine Wave + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Sine Wave + Noise | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Random Walk | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 87264 bits | 8.73 BPS | **0.92x** |
| Random Walk | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 91544 bits | 9.15 BPS | **0.87x** |
| Random Walk | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 83552 bits | 8.36 BPS | **0.96x** |
| Random Walk | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 78176 bits | 7.82 BPS | **1.02x** |
| Random Walk | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 70632 bits | 7.06 BPS | **1.13x** |
| Random Walk | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 84032 bits | 8.40 BPS | **0.95x** |
| Random Walk | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 68032 bits | 6.80 BPS | **1.18x** |
| Random Walk | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 85744 bits | 8.57 BPS | **0.93x** |
| Random Walk | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 84352 bits | 8.44 BPS | **0.95x** |
| Random Walk | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 178176 bits | 17.82 BPS | **0.90x** |
| Random Walk | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 178176 bits | 17.82 BPS | **0.90x** |
| Random Walk | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 198176 bits | 19.82 BPS | **0.81x** |
| Random Walk | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 198176 bits | 19.82 BPS | **0.81x** |
| Random Walk | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 160784 bits | 16.08 BPS | **0.62x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 140240 bits | 14.02 BPS | **0.71x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 131088 bits | 13.11 BPS | **0.76x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 119648 bits | 11.96 BPS | **0.84x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 109616 bits | 10.96 BPS | **0.91x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 88608 bits | 8.86 BPS | **1.13x** |
| Random Walk | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 107680 bits | 10.77 BPS | **0.93x** |
| Random Walk | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 97568 bits | 9.76 BPS | **1.02x** |
| Random Walk | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Random Walk | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 259304 bits | 25.93 BPS | **0.62x** |
| Random Walk | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 259304 bits | 25.93 BPS | **0.62x** |
| Random Walk | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 279304 bits | 27.93 BPS | **0.57x** |
| Random Walk | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 279304 bits | 27.93 BPS | **0.57x** |
| Random Walk | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 279304 bits | 27.93 BPS | **0.57x** |
| Random Walk | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 279304 bits | 27.93 BPS | **0.57x** |
| Random Walk | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 356352 bits | 35.64 BPS | **0.90x** |
| Random Walk | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 356352 bits | 35.64 BPS | **0.90x** |
| Random Walk | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 672818 bits | 67.28 BPS | **0.94x** |
| Random Walk | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 672818 bits | 67.28 BPS | **0.94x** |
| Random Walk | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Random Walk | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Random Walk + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 87304 bits | 8.73 BPS | **0.92x** |
| Random Walk + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 91944 bits | 9.19 BPS | **0.87x** |
| Random Walk + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 82424 bits | 8.24 BPS | **0.97x** |
| Random Walk + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 78392 bits | 7.84 BPS | **1.02x** |
| Random Walk + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 69216 bits | 6.92 BPS | **1.16x** |
| Random Walk + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 138256 bits | 13.83 BPS | **0.58x** |
| Random Walk + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 83824 bits | 8.38 BPS | **0.95x** |
| Random Walk + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 86016 bits | 8.60 BPS | **0.93x** |
| Random Walk + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 84768 bits | 8.48 BPS | **0.94x** |
| Random Walk + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Random Walk + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 174656 bits | 17.47 BPS | **0.57x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 159984 bits | 16.00 BPS | **0.63x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 153152 bits | 15.32 BPS | **0.65x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 143968 bits | 14.40 BPS | **0.69x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 105872 bits | 10.59 BPS | **0.94x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 94272 bits | 9.43 BPS | **1.06x** |
| Random Walk + Noise | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 164560 bits | 16.46 BPS | **0.61x** |
| Random Walk + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 110592 bits | 11.06 BPS | **0.90x** |
| Random Walk + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Random Walk + Noise | `GradientEncoding<u16, U2, u8> [Denoise=4]` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Random Walk + Noise | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Random Walk + Noise | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Random Walk + Noise | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Random Walk + Noise | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Random Walk + Noise | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 280000 bits | 28.00 BPS | **0.57x** |
| Random Walk + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<U32, U4> [Denoise=8]` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Random Walk + Noise | `DiffEncoding<U63, U5> [Denoise=16]` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Random Walk + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Random Walk + Noise | `GradientEncoding<U63, U2, u8> [Denoise=16]` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
