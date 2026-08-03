# Encoding Benchmark Size Results

| Dataset | Encoding Scheme | Sample Count | Uncompressed Size | Compressed Size | Bits Per Sample | Compression Ratio |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Constant | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 12504 bits | 1.25 BPS | **6.40x** |
| Constant | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Constant | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25008 bits | 2.50 BPS | **3.20x** |
| Constant | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 25008 bits | 2.50 BPS | **6.40x** |
| Constant | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Constant | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Constant | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 27512 bits | 2.75 BPS | **5.82x** |
| Constant | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 50016 bits | 5.00 BPS | **6.40x** |
| Constant | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 69719 bits | 6.97 BPS | **9.04x** |
| Constant | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 42223 bits | 4.22 BPS | **14.92x** |
| Constant + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 82656 bits | 8.27 BPS | **0.97x** |
| Constant + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 39136 bits | 3.91 BPS | **2.04x** |
| Constant + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Constant + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 117072 bits | 11.71 BPS | **0.68x** |
| Constant + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Constant + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 131744 bits | 13.17 BPS | **0.76x** |
| Constant + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Constant + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Constant + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Constant + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Linear Ramp | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 14480 bits | 1.45 BPS | **5.52x** |
| Linear Ramp | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22504 bits | 2.25 BPS | **3.55x** |
| Linear Ramp | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25008 bits | 2.50 BPS | **3.20x** |
| Linear Ramp | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 45008 bits | 4.50 BPS | **3.55x** |
| Linear Ramp | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25008 bits | 2.50 BPS | **4.00x** |
| Linear Ramp | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 27536 bits | 2.75 BPS | **5.81x** |
| Linear Ramp | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Linear Ramp | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Linear Ramp | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 85104 bits | 8.51 BPS | **0.94x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 39624 bits | 3.96 BPS | **2.02x** |
| Linear Ramp + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Linear Ramp + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 121408 bits | 12.14 BPS | **0.66x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179920 bits | 17.99 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 199920 bits | 19.99 BPS | **0.80x** |
| Linear Ramp + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 131920 bits | 13.19 BPS | **0.76x** |
| Linear Ramp + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 259880 bits | 25.99 BPS | **0.62x** |
| Linear Ramp + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 359840 bits | 35.98 BPS | **0.89x** |
| Linear Ramp + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 679685 bits | 67.97 BPS | **0.93x** |
| Linear Ramp + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 729645 bits | 72.96 BPS | **0.86x** |
| Sine Wave | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 62312 bits | 6.23 BPS | **1.28x** |
| Sine Wave | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 22632 bits | 2.26 BPS | **3.53x** |
| Sine Wave | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 25008 bits | 2.50 BPS | **3.20x** |
| Sine Wave | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179296 bits | 17.93 BPS | **0.89x** |
| Sine Wave | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 197152 bits | 19.72 BPS | **0.81x** |
| Sine Wave | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 25056 bits | 2.51 BPS | **3.99x** |
| Sine Wave | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 255248 bits | 25.52 BPS | **0.63x** |
| Sine Wave | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Sine Wave | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Sine Wave | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Sine Wave + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 69672 bits | 6.97 BPS | **1.15x** |
| Sine Wave + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 50416 bits | 5.04 BPS | **1.59x** |
| Sine Wave + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 42504 bits | 4.25 BPS | **1.88x** |
| Sine Wave + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 123568 bits | 12.36 BPS | **0.65x** |
| Sine Wave + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 179232 bits | 17.92 BPS | **0.89x** |
| Sine Wave + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 196336 bits | 19.63 BPS | **0.81x** |
| Sine Wave + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 131744 bits | 13.17 BPS | **0.76x** |
| Sine Wave + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 244688 bits | 24.47 BPS | **0.65x** |
| Sine Wave + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Sine Wave + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Sine Wave + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
| Random Walk | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 87352 bits | 8.74 BPS | **0.92x** |
| Random Walk | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 91792 bits | 9.18 BPS | **0.87x** |
| Random Walk | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 79448 bits | 7.94 BPS | **1.01x** |
| Random Walk | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 87008 bits | 8.70 BPS | **0.92x** |
| Random Walk | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 178240 bits | 17.82 BPS | **0.90x** |
| Random Walk | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 198240 bits | 19.82 BPS | **0.81x** |
| Random Walk | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 160064 bits | 16.01 BPS | **0.62x** |
| Random Walk | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 257360 bits | 25.74 BPS | **0.62x** |
| Random Walk | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 356480 bits | 35.65 BPS | **0.90x** |
| Random Walk | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 673070 bits | 67.31 BPS | **0.94x** |
| Random Walk | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 722190 bits | 72.22 BPS | **0.87x** |
| Random Walk + Noise | `DiffEncoding<u8, U1>` | 10000 | 80000 bits | 87408 bits | 8.74 BPS | **0.92x** |
| Random Walk + Noise | `DiffEncoding<u8, U2>` | 10000 | 80000 bits | 92280 bits | 9.23 BPS | **0.87x** |
| Random Walk + Noise | `DiffEncoding<u8, U4>` | 10000 | 80000 bits | 79704 bits | 7.97 BPS | **1.00x** |
| Random Walk + Noise | `GradientEncoding<u8, U2, u8>` | 10000 | 80000 bits | 139472 bits | 13.95 BPS | **0.57x** |
| Random Walk + Noise | `DiffEncoding<u16, U2>` | 10000 | 160000 bits | 180000 bits | 18.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<u16, U4>` | 10000 | 160000 bits | 200000 bits | 20.00 BPS | **0.80x** |
| Random Walk + Noise | `GradientEncoding<U10, U2, U6>` | 10000 | 100000 bits | 176032 bits | 17.60 BPS | **0.57x** |
| Random Walk + Noise | `GradientEncoding<u16, U2, u8>` | 10000 | 160000 bits | 260000 bits | 26.00 BPS | **0.62x** |
| Random Walk + Noise | `DiffEncoding<U32, U4>` | 10000 | 320000 bits | 360000 bits | 36.00 BPS | **0.89x** |
| Random Walk + Noise | `DiffEncoding<U63, U5>` | 10000 | 630000 bits | 680000 bits | 68.00 BPS | **0.93x** |
| Random Walk + Noise | `GradientEncoding<U63, U2, u8>` | 10000 | 630000 bits | 730000 bits | 73.00 BPS | **0.86x** |
