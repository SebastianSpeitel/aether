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

### DiffEncoding Results

<details>
<summary>Click to expand DiffEncoding benchmark results table</summary>

| Dataset | Encoding Scheme | Sample Count | Uncompressed Size | Compressed Size | Bits Per Sample | Compression Ratio |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Constant | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 12504 bits | 1.25 BPS | **6.40x** |
| Constant | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Constant | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Constant | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 25008 bits | 2.50 BPS | **6.40x** |
| Constant | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 25008 bits | 2.50 BPS | **6.40x** |
| Constant | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Constant | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Constant | `DiffEncoding<u16, U6>` | 10000 | 160000 bits | 65008 bits | 6.50 BPS | **2.46x** |
| Constant | `DiffEncoding<u16, U6> [Denoise=32]` | 10000 | 160000 bits | 65008 bits | 6.50 BPS | **2.46x** |
| Constant | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 90016 bits | 9.00 BPS | **3.55x** |
| Constant | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 90016 bits | 9.00 BPS | **3.55x** |
| Constant | `DiffEncoding<U32, U18>` | 10000 | 320000 bits | 190016 bits | 19.00 BPS | **1.68x** |
| Constant | `DiffEncoding<U32, U18> [Denoise=512]` | 10000 | 320000 bits | 190016 bits | 19.00 BPS | **1.68x** |
| Constant | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 139719 bits | 13.97 BPS | **4.51x** |
| Constant | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 139719 bits | 13.97 BPS | **4.51x** |
| Constant | `DiffEncoding<U63, U24>` | 10000 | 630000 bits | 259719 bits | 25.97 BPS | **2.43x** |
| Constant | `DiffEncoding<U63, U24> [Denoise=2048]` | 10000 | 630000 bits | 259719 bits | 25.97 BPS | **2.43x** |
| Constant | `DiffEncoding<U63, U30>` | 10000 | 630000 bits | 319719 bits | 31.97 BPS | **1.97x** |
| Constant | `DiffEncoding<U63, U30> [Denoise=8192]` | 10000 | 630000 bits | 319719 bits | 31.97 BPS | **1.97x** |
| Constant + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 82456 bits | 8.25 BPS | **0.97x** |
| Constant + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 37256 bits | 3.73 BPS | **2.15x** |
| Constant + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Constant + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Constant + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Constant + Noise | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Constant + Noise | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Constant + Noise | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Constant + Noise | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Linear Ramp | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 14080 bits | 1.41 BPS | **5.68x** |
| Linear Ramp | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u16, U6>` | 10000 | 160000 bits | 65008 bits | 6.50 BPS | **2.46x** |
| Linear Ramp | `DiffEncoding<u16, U6> [Denoise=32]` | 10000 | 160000 bits | 65008 bits | 6.50 BPS | **2.46x** |
| Linear Ramp | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Linear Ramp | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Linear Ramp | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Linear Ramp | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 84968 bits | 8.50 BPS | **0.94x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 37744 bits | 3.77 BPS | **2.12x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179920 bits | 17.99 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 179920 bits | 17.99 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 199920 bits | 19.99 BPS | **0.80x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 199920 bits | 19.99 BPS | **0.80x** |
| Linear Ramp + Noise | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 399840 bits | 39.98 BPS | **0.80x** |
| Linear Ramp + Noise | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 399840 bits | 39.98 BPS | **0.80x** |
| Linear Ramp + Noise | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 749685 bits | 74.97 BPS | **0.84x** |
| Linear Ramp + Noise | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 749685 bits | 74.97 BPS | **0.84x** |
| Sawtooth | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 90000 bits | 9.00 BPS | **0.89x** |
| Sawtooth | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 100000 bits | 10.00 BPS | **0.80x** |
| Sawtooth | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 100000 bits | 10.00 BPS | **0.80x** |
| Sawtooth | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sawtooth | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sawtooth | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Sawtooth | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Sawtooth | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Sawtooth | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Sawtooth | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sawtooth | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sawtooth | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Sawtooth | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Sawtooth + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 90000 bits | 9.00 BPS | **0.89x** |
| Sawtooth + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 100000 bits | 10.00 BPS | **0.80x** |
| Sawtooth + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 84504 bits | 8.45 BPS | **0.95x** |
| Sawtooth + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sawtooth + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sawtooth + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Sawtooth + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Sawtooth + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Sawtooth + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Sawtooth + Noise | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sawtooth + Noise | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sawtooth + Noise | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Sawtooth + Noise | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Sine Wave | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 61424 bits | 6.14 BPS | **1.30x** |
| Sine Wave | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22600 bits | 2.26 BPS | **3.54x** |
| Sine Wave | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Sine Wave | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179248 bits | 17.92 BPS | **0.89x** |
| Sine Wave | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 178080 bits | 17.81 BPS | **0.90x** |
| Sine Wave | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 197072 bits | 19.71 BPS | **0.81x** |
| Sine Wave | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 195888 bits | 19.59 BPS | **0.82x** |
| Sine Wave | `DiffEncoding<u16, U6> [Denoise=32]` | 10000 | 160000 bits | 199424 bits | 19.94 BPS | **0.80x** |
| Sine Wave | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sine Wave | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sine Wave | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Sine Wave | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Sine Wave + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 69032 bits | 6.90 BPS | **1.16x** |
| Sine Wave + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 48896 bits | 4.89 BPS | **1.64x** |
| Sine Wave + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 22632 bits | 2.26 BPS | **3.53x** |
| Sine Wave + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179216 bits | 17.92 BPS | **0.89x** |
| Sine Wave + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 177616 bits | 17.76 BPS | **0.90x** |
| Sine Wave + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 196240 bits | 19.62 BPS | **0.82x** |
| Sine Wave + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 194528 bits | 19.45 BPS | **0.82x** |
| Sine Wave + Noise | `DiffEncoding<u16, U6> [Denoise=32]` | 10000 | 160000 bits | 190832 bits | 19.08 BPS | **0.84x** |
| Sine Wave + Noise | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sine Wave + Noise | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Sine Wave + Noise | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Sine Wave + Noise | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Random Walk | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 87264 bits | 8.73 BPS | **0.92x** |
| Random Walk | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 91544 bits | 9.15 BPS | **0.87x** |
| Random Walk | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 83552 bits | 8.36 BPS | **0.96x** |
| Random Walk | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 78176 bits | 7.82 BPS | **1.02x** |
| Random Walk | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 70632 bits | 7.06 BPS | **1.13x** |
| Random Walk | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 178176 bits | 17.82 BPS | **0.90x** |
| Random Walk | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 178176 bits | 17.82 BPS | **0.90x** |
| Random Walk | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 198176 bits | 19.82 BPS | **0.81x** |
| Random Walk | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 198176 bits | 19.82 BPS | **0.81x** |
| Random Walk | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 396352 bits | 39.64 BPS | **0.81x** |
| Random Walk | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 396352 bits | 39.64 BPS | **0.81x** |
| Random Walk | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 742818 bits | 74.28 BPS | **0.85x** |
| Random Walk | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 742818 bits | 74.28 BPS | **0.85x** |
| Random Walk + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 87304 bits | 8.73 BPS | **0.92x** |
| Random Walk + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 91944 bits | 9.19 BPS | **0.87x** |
| Random Walk + Noise | `DiffEncoding<u8, U2> [Denoise=2]` | 10000 | 80000 bits | 82424 bits | 8.24 BPS | **0.97x** |
| Random Walk + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 78392 bits | 7.84 BPS | **1.02x** |
| Random Walk + Noise | `DiffEncoding<u8, U4> [Denoise=2]` | 10000 | 80000 bits | 69216 bits | 6.92 BPS | **1.16x** |
| Random Walk + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<u16, U2> [Denoise=4]` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Random Walk + Noise | `DiffEncoding<u16, U4> [Denoise=4]` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Random Walk + Noise | `DiffEncoding<U32, U8>` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Random Walk + Noise | `DiffEncoding<U32, U8> [Denoise=128]` | 10000 | 320000 bits | 400000 bits | 40.00 BPS | **0.80x** |
| Random Walk + Noise | `DiffEncoding<U63, U12>` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |
| Random Walk + Noise | `DiffEncoding<U63, U12> [Denoise=512]` | 10000 | 630000 bits | 750000 bits | 75.00 BPS | **0.84x** |

</details>

### GradientEncoding Results

<details>
<summary>Click to expand GradientEncoding benchmark results table</summary>

| Dataset | Encoding Scheme | Sample Count | Uncompressed Size | Compressed Size | Bits Per Sample | Compression Ratio |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Constant | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Constant | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Constant | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Constant | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
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
| Constant | `GradientEncoding<u16, U6, u8>` | 10000 | 160000 bits | 67536 bits | 6.75 BPS | **2.37x** |
| Constant | `GradientEncoding<u16, U6, u8> [Denoise=32]` | 10000 | 160000 bits | 67536 bits | 6.75 BPS | **2.37x** |
| Constant | `GradientEncoding<U32, U8, u16>` | 10000 | 320000 bits | 95072 bits | 9.51 BPS | **3.37x** |
| Constant | `GradientEncoding<U32, U8, u16> [Denoise=128]` | 10000 | 320000 bits | 95072 bits | 9.51 BPS | **3.37x** |
| Constant | `GradientEncoding<U32, U18, u16>` | 10000 | 320000 bits | 195024 bits | 19.50 BPS | **1.64x** |
| Constant | `GradientEncoding<U32, U18, u16> [Denoise=512]` | 10000 | 320000 bits | 195024 bits | 19.50 BPS | **1.64x** |
| Constant | `GradientEncoding<U63, U12, u16>` | 10000 | 630000 bits | 144806 bits | 14.48 BPS | **4.35x** |
| Constant | `GradientEncoding<U63, U12, u16> [Denoise=512]` | 10000 | 630000 bits | 144806 bits | 14.48 BPS | **4.35x** |
| Constant | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 269830 bits | 26.98 BPS | **2.33x** |
| Constant | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 269830 bits | 26.98 BPS | **2.33x** |
| Constant + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Constant + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Constant + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 26368 bits | 2.64 BPS | **3.79x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Constant + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 64816 bits | 6.48 BPS | **1.54x** |
| Constant + Noise | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Constant + Noise | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Linear Ramp | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25008 bits | 2.50 BPS | **3.20x** |
| Linear Ramp | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25008 bits | 2.50 BPS | **3.20x** |
| Linear Ramp | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Linear Ramp | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
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
| Linear Ramp | `GradientEncoding<u16, U6, u8>` | 10000 | 160000 bits | 67512 bits | 6.75 BPS | **2.37x** |
| Linear Ramp | `GradientEncoding<u16, U6, u8> [Denoise=32]` | 10000 | 160000 bits | 67512 bits | 6.75 BPS | **2.37x** |
| Linear Ramp | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Linear Ramp | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 27504 bits | 2.75 BPS | **2.91x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 66384 bits | 6.64 BPS | **1.51x** |
| Linear Ramp + Noise | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141372 bits | 14.14 BPS | **4.46x** |
| Linear Ramp + Noise | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141372 bits | 14.14 BPS | **4.46x** |
| Sawtooth | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Sawtooth | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Sawtooth | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Sawtooth | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Sawtooth | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sawtooth | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sawtooth | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sawtooth | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sawtooth | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sawtooth | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sawtooth | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Sawtooth | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Sawtooth | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Sawtooth | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Sawtooth + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 29648 bits | 2.96 BPS | **2.70x** |
| Sawtooth + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Sawtooth + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45008 bits | 4.50 BPS | **1.78x** |
| Sawtooth + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 107360 bits | 10.74 BPS | **0.93x** |
| Sawtooth + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sawtooth + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 67184 bits | 6.72 BPS | **1.49x** |
| Sawtooth + Noise | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Sawtooth + Noise | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Sine Wave | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Sine Wave | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 25024 bits | 2.50 BPS | **3.20x** |
| Sine Wave | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25040 bits | 2.50 BPS | **3.99x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=1]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=2]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Sine Wave | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 45024 bits | 4.50 BPS | **2.22x** |
| Sine Wave | `GradientEncoding<u16, U2, u8> [Denoise=16]` | 10000 | 160000 bits | 193568 bits | 19.36 BPS | **0.83x** |
| Sine Wave | `GradientEncoding<u16, U4, u8>` | 10000 | 160000 bits | 184768 bits | 18.48 BPS | **0.87x** |
| Sine Wave | `GradientEncoding<u16, U4, u8> [Denoise=2]` | 10000 | 160000 bits | 184768 bits | 18.48 BPS | **0.87x** |
| Sine Wave | `GradientEncoding<u16, U4, u8> [Denoise=8]` | 10000 | 160000 bits | 184768 bits | 18.48 BPS | **0.87x** |
| Sine Wave | `GradientEncoding<u16, U4, u8> [Denoise=16]` | 10000 | 160000 bits | 184000 bits | 18.40 BPS | **0.87x** |
| Sine Wave | `GradientEncoding<u16, U6, u8>` | 10000 | 160000 bits | 180960 bits | 18.10 BPS | **0.88x** |
| Sine Wave | `GradientEncoding<u16, U6, u8> [Denoise=32]` | 10000 | 160000 bits | 157320 bits | 15.73 BPS | **1.02x** |
| Sine Wave | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Sine Wave | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Sine Wave + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 28048 bits | 2.80 BPS | **2.85x** |
| Sine Wave + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 45024 bits | 4.50 BPS | **1.78x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 31984 bits | 3.20 BPS | **3.13x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 25024 bits | 2.50 BPS | **4.00x** |
| Sine Wave + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 66592 bits | 6.66 BPS | **1.50x** |
| Sine Wave + Noise | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Sine Wave + Noise | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Random Walk | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 84032 bits | 8.40 BPS | **0.95x** |
| Random Walk | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 68032 bits | 6.80 BPS | **1.18x** |
| Random Walk | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 85744 bits | 8.57 BPS | **0.93x** |
| Random Walk | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 84352 bits | 8.44 BPS | **0.95x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=4]` | 10000 | 100000 bits | 119648 bits | 11.96 BPS | **0.84x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 109616 bits | 10.96 BPS | **0.91x** |
| Random Walk | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 88608 bits | 8.86 BPS | **1.13x** |
| Random Walk | `GradientEncoding<U10, U4, U6>` | 10000 | 100000 bits | 107680 bits | 10.77 BPS | **0.93x** |
| Random Walk | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 97568 bits | 9.76 BPS | **1.02x** |
| Random Walk | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Random Walk | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Random Walk + Noise | `GradientEncoding<u8, U2, u8> [Denoise=2]` | 10000 | 80000 bits | 83824 bits | 8.38 BPS | **0.95x** |
| Random Walk + Noise | `GradientEncoding<u8, U4, u8>` | 10000 | 80000 bits | 86016 bits | 8.60 BPS | **0.93x** |
| Random Walk + Noise | `GradientEncoding<u8, U4, u8> [Denoise=2]` | 10000 | 80000 bits | 84768 bits | 8.48 BPS | **0.94x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6> [Denoise=8]` | 10000 | 100000 bits | 105872 bits | 10.59 BPS | **0.94x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6> [Denoise=16]` | 10000 | 100000 bits | 94272 bits | 9.43 BPS | **1.06x** |
| Random Walk + Noise | `GradientEncoding<U10, U4, U6> [Denoise=4]` | 10000 | 100000 bits | 110592 bits | 11.06 BPS | **0.90x** |
| Random Walk + Noise | `GradientEncoding<U63, U24, U32>` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |
| Random Walk + Noise | `GradientEncoding<U63, U24, U32> [Denoise=2048]` | 10000 | 630000 bits | 141491 bits | 14.15 BPS | **4.45x** |

</details>
