use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use delta_encoding::primitive::{U1, U2, U4, U6, U8, U10, U12, U18, U24, U30, U32, U63};
use delta_encoding::{DiffEncoding, EncodedRing, Encoding, GradientEncoding};

fn lcg(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005).wrapping_add(1)
}

fn add_noise(val: f64, noise_amplitude: f64, index: usize) -> f64 {
    let rand = (lcg(index as u64) >> 33) as f64 / (1u64 << 31) as f64;
    let noise = (rand - 0.5) * 2.0 * noise_amplitude;
    (val + noise).max(0.0)
}

fn generate_series(
    dataset_type: &str,
    count: usize,
    add_noise_flag: bool,
    max_val: f64,
) -> Vec<f64> {
    let noise_amp = if add_noise_flag { max_val * 0.005 } else { 0.0 };
    (0..count)
        .map(|i| {
            let base = match dataset_type {
                "Constant" => max_val * 0.5,
                "Linear Ramp" => (i as f64 / count as f64) * max_val,
                "Sine Wave" => ((i as f64 * 0.01).sin() * 0.4 + 0.5) * max_val,
                "Random Walk" => {
                    let step = ((i % 7) as f64 - 3.0) * (max_val * 0.001);
                    (max_val * 0.5 + step * (i as f64 % 50.0)).clamp(0.0, max_val)
                }
                _ => max_val * 0.5,
            };
            add_noise(base, noise_amp, i).clamp(0.0, max_val)
        })
        .collect()
}

fn scale_to_u8(vals: &[f64], max_val: f64) -> Vec<u8> {
    vals.iter()
        .map(|&v| (v / max_val * 255.0).round() as u8)
        .collect()
}

fn scale_to_u10(vals: &[f64], max_val: f64) -> Vec<U10> {
    vals.iter()
        .map(|&v| U10((v / max_val * 1023.0).round() as u16))
        .collect()
}

fn scale_to_u16(vals: &[f64], max_val: f64) -> Vec<u16> {
    vals.iter()
        .map(|&v| (v / max_val * 65535.0).round() as u16)
        .collect()
}

fn scale_to_u32(vals: &[f64], max_val: f64) -> Vec<U32> {
    vals.iter()
        .map(|&v| U32((v / max_val * 4294967295.0).round() as u32))
        .collect()
}

fn scale_to_u63(vals: &[f64], max_val: f64) -> Vec<U63> {
    vals.iter()
        .map(|&v| {
            let scaled = (v / max_val * 9223372036854775807.0).round();
            let clamped = (scaled as u64).min(9223372036854775807);
            U63(clamped)
        })
        .collect()
}

struct BenchRow {
    dataset_label: String,
    display_name: String,
    sample_count: usize,
    uncompressed_bits: usize,
    compressed_bits: usize,
    bps: f64,
    ratio: f64,
}

fn record_size_benchmark<E: Encoding>(
    name: &str,
    dataset_label: &str,
    data: &[E::Value],
    raw_bits_per_sample: usize,
    results: &mut Vec<BenchRow>,
) where
    E::Value: Copy,
{
    record_denoised_size_benchmark::<E, 0>(name, dataset_label, data, raw_bits_per_sample, results);
}

fn record_denoised_size_benchmark<E: Encoding, const DENOISE: usize>(
    name: &str,
    dataset_label: &str,
    data: &[E::Value],
    raw_bits_per_sample: usize,
    results: &mut Vec<BenchRow>,
) where
    E::Value: Copy,
{
    let mut ring = EncodedRing::<131072, E>::new();
    for &v in data {
        ring.push_denoised::<DENOISE>(v);
    }
    let uncompressed_bits = data.len() * raw_bits_per_sample;
    let compressed_bits = ring.bit_len();
    let bps = if data.is_empty() {
        0.0
    } else {
        compressed_bits as f64 / data.len() as f64
    };
    let ratio = if compressed_bits > 0 {
        uncompressed_bits as f64 / compressed_bits as f64
    } else {
        0.0
    };
    let display_name = if DENOISE > 0 {
        format!("{} [Denoise={}]", name, DENOISE)
    } else {
        name.to_string()
    };
    println!(
        "  {:<42} | BPS: {:5.2} | Ratio: {:6.2}x ({:>6} bits -> {:>6} bits)",
        display_name, bps, ratio, uncompressed_bits, compressed_bits
    );

    results.push(BenchRow {
        dataset_label: dataset_label.to_string(),
        display_name,
        sample_count: data.len(),
        uncompressed_bits,
        compressed_bits,
        bps,
        ratio,
    });
}

fn bench_speed_and_compression(c: &mut Criterion) {
    let dataset_size = 10_000;
    let linear_u16 = scale_to_u16(
        &generate_series("Linear Ramp", dataset_size, false, 1.0),
        1.0,
    );

    let mut group = c.benchmark_group("encoding_performance");
    group.bench_with_input(
        BenchmarkId::new("DiffEncoding_u16_linear", dataset_size),
        &linear_u16,
        |b, data| {
            b.iter(|| {
                let mut ring = EncodedRing::<4096, DiffEncoding<u16, U4>>::new();
                for &v in data {
                    ring.push(black_box(v));
                }
                black_box(ring)
            });
        },
    );
    group.finish();

    println!(
        "\n=== EXTENSIVE COMPRESSION SIZE BENCHMARKS WITH RESCALING AND NOISE ({} samples) ===",
        dataset_size
    );

    let mut md_table = String::new();
    md_table.push_str("# Encoding Benchmark Size Results\n\n");
    md_table.push_str("## Legend & Encoding Scheme Conventions\n\n");
    md_table.push_str("- **`DiffEncoding<T, F>`**: Difference (zero-order) delta encoding.\n");
    md_table.push_str("  - `T`: Raw sample value type (e.g. `u8`, `U10`, `u16`, `U32`, `U63`).\n");
    md_table.push_str("  - `F`: Flag/delta bit-width (e.g. `U1` = 1 bit, `U2` = 2 bits, `U4` = 4 bits). `F::KEY_FLAG` indicates a full keyframe write.\n");
    md_table.push_str(
        "- **`GradientEncoding<T, F, V>`**: First-order (velocity tracking) gradient encoding.\n",
    );
    md_table.push_str("  - `T`: Raw sample value type.\n");
    md_table.push_str("  - `F`: Residual difference flag bit-width.\n");
    md_table.push_str("  - `V`: Signed velocity bit-width stored inside keyframes.\n");
    md_table.push_str("- **`[Denoise=N]`**: Deadband filtering threshold `N` LSB counts.\n");
    md_table.push_str("  - Clamps residual step differences within `[MIN_DELTA - N, MAX_DELTA + N]` into valid `[MIN_DELTA, MAX_DELTA]` encoding steps.\n\n");
    let mut diff_rows: Vec<BenchRow> = Vec::new();
    let mut grad_rows: Vec<BenchRow> = Vec::new();

    let dataset_names = ["Constant", "Linear Ramp", "Sine Wave", "Random Walk"];

    for ds_name in dataset_names {
        for with_noise in [false, true] {
            let label = if with_noise {
                format!("{} + Noise", ds_name)
            } else {
                ds_name.to_string()
            };

            println!("\n--- Dataset: {} ---", label);
            let raw_data = generate_series(ds_name, dataset_size, with_noise, 1.0);

            let data_u8 = scale_to_u8(&raw_data, 1.0);
            let data_u10 = scale_to_u10(&raw_data, 1.0);
            let data_u16 = scale_to_u16(&raw_data, 1.0);
            let data_u32 = scale_to_u32(&raw_data, 1.0);
            let data_u63 = scale_to_u63(&raw_data, 1.0);

            // Small & Mid Range Signals (u8, U10, u16)
            record_size_benchmark::<DiffEncoding<u8, U1>>(
                "DiffEncoding<u8, U1>",
                &label,
                &data_u8,
                8,
                &mut diff_rows,
            );
            record_size_benchmark::<DiffEncoding<u8, U2>>(
                "DiffEncoding<u8, U2>",
                &label,
                &data_u8,
                8,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<u8, U2>, 2>(
                "DiffEncoding<u8, U2>",
                &label,
                &data_u8,
                8,
                &mut diff_rows,
            );
            record_size_benchmark::<DiffEncoding<u8, U4>>(
                "DiffEncoding<u8, U4>",
                &label,
                &data_u8,
                8,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<u8, U4>, 2>(
                "DiffEncoding<u8, U4>",
                &label,
                &data_u8,
                8,
                &mut diff_rows,
            );

            record_size_benchmark::<DiffEncoding<u16, U2>>(
                "DiffEncoding<u16, U2>",
                &label,
                &data_u16,
                16,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<u16, U2>, 4>(
                "DiffEncoding<u16, U2>",
                &label,
                &data_u16,
                16,
                &mut diff_rows,
            );
            record_size_benchmark::<DiffEncoding<u16, U4>>(
                "DiffEncoding<u16, U4>",
                &label,
                &data_u16,
                16,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<u16, U4>, 4>(
                "DiffEncoding<u16, U4>",
                &label,
                &data_u16,
                16,
                &mut diff_rows,
            );
            record_size_benchmark::<DiffEncoding<u16, U6>>(
                "DiffEncoding<u16, U6>",
                &label,
                &data_u16,
                16,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<u16, U6>, 32>(
                "DiffEncoding<u16, U6>",
                &label,
                &data_u16,
                16,
                &mut diff_rows,
            );

            // Large Signals (U32, U63) - Higher Range Deltas (U8, U12, U18, U20, U24, U30)
            record_size_benchmark::<DiffEncoding<U32, U8>>(
                "DiffEncoding<U32, U8>",
                &label,
                &data_u32,
                32,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<U32, U8>, 128>(
                "DiffEncoding<U32, U8>",
                &label,
                &data_u32,
                32,
                &mut diff_rows,
            );
            record_size_benchmark::<DiffEncoding<U32, U18>>(
                "DiffEncoding<U32, U18>",
                &label,
                &data_u32,
                32,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<U32, U18>, 512>(
                "DiffEncoding<U32, U18>",
                &label,
                &data_u32,
                32,
                &mut diff_rows,
            );

            record_size_benchmark::<DiffEncoding<U63, U12>>(
                "DiffEncoding<U63, U12>",
                &label,
                &data_u63,
                63,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<U63, U12>, 512>(
                "DiffEncoding<U63, U12>",
                &label,
                &data_u63,
                63,
                &mut diff_rows,
            );
            record_size_benchmark::<DiffEncoding<U63, U24>>(
                "DiffEncoding<U63, U24>",
                &label,
                &data_u63,
                63,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<U63, U24>, 2048>(
                "DiffEncoding<U63, U24>",
                &label,
                &data_u63,
                63,
                &mut diff_rows,
            );
            record_size_benchmark::<DiffEncoding<U63, U30>>(
                "DiffEncoding<U63, U30>",
                &label,
                &data_u63,
                63,
                &mut diff_rows,
            );
            record_denoised_size_benchmark::<DiffEncoding<U63, U30>, 8192>(
                "DiffEncoding<U63, U30>",
                &label,
                &data_u63,
                63,
                &mut diff_rows,
            );

            // --- GRADIENT ENCODING ---
            record_size_benchmark::<GradientEncoding<u8, U2, u8>>(
                "GradientEncoding<u8, U2, u8>",
                &label,
                &data_u8,
                8,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u8, U2, u8>, 2>(
                "GradientEncoding<u8, U2, u8>",
                &label,
                &data_u8,
                8,
                &mut grad_rows,
            );
            record_size_benchmark::<GradientEncoding<u8, U4, u8>>(
                "GradientEncoding<u8, U4, u8>",
                &label,
                &data_u8,
                8,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u8, U4, u8>, 2>(
                "GradientEncoding<u8, U4, u8>",
                &label,
                &data_u8,
                8,
                &mut grad_rows,
            );

            record_size_benchmark::<GradientEncoding<U10, U2, U6>>(
                "GradientEncoding<U10, U2, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U10, U2, U6>, 1>(
                "GradientEncoding<U10, U2, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U10, U2, U6>, 2>(
                "GradientEncoding<U10, U2, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U10, U2, U6>, 4>(
                "GradientEncoding<U10, U2, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U10, U2, U6>, 8>(
                "GradientEncoding<U10, U2, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U10, U2, U6>, 16>(
                "GradientEncoding<U10, U2, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );
            record_size_benchmark::<GradientEncoding<U10, U4, U6>>(
                "GradientEncoding<U10, U4, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U10, U4, U6>, 4>(
                "GradientEncoding<U10, U4, U6>",
                &label,
                &data_u10,
                10,
                &mut grad_rows,
            );

            record_size_benchmark::<GradientEncoding<u16, U2, u8>>(
                "GradientEncoding<u16, U2, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u16, U2, u8>, 4>(
                "GradientEncoding<u16, U2, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u16, U2, u8>, 16>(
                "GradientEncoding<u16, U2, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_size_benchmark::<GradientEncoding<u16, U4, u8>>(
                "GradientEncoding<u16, U4, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u16, U4, u8>, 2>(
                "GradientEncoding<u16, U4, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u16, U4, u8>, 8>(
                "GradientEncoding<u16, U4, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u16, U4, u8>, 16>(
                "GradientEncoding<u16, U4, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_size_benchmark::<GradientEncoding<u16, U6, u8>>(
                "GradientEncoding<u16, U6, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<u16, U6, u8>, 32>(
                "GradientEncoding<u16, U6, u8>",
                &label,
                &data_u16,
                16,
                &mut grad_rows,
            );

            // Large Signals (U32, U63) - Higher Range Deltas
            record_size_benchmark::<GradientEncoding<U32, U8, u16>>(
                "GradientEncoding<U32, U8, u16>",
                &label,
                &data_u32,
                32,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U32, U8, u16>, 128>(
                "GradientEncoding<U32, U8, u16>",
                &label,
                &data_u32,
                32,
                &mut grad_rows,
            );
            record_size_benchmark::<GradientEncoding<U32, U18, u16>>(
                "GradientEncoding<U32, U18, u16>",
                &label,
                &data_u32,
                32,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U32, U18, u16>, 512>(
                "GradientEncoding<U32, U18, u16>",
                &label,
                &data_u32,
                32,
                &mut grad_rows,
            );

            record_size_benchmark::<GradientEncoding<U63, U12, u16>>(
                "GradientEncoding<U63, U12, u16>",
                &label,
                &data_u63,
                63,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U63, U12, u16>, 512>(
                "GradientEncoding<U63, U12, u16>",
                &label,
                &data_u63,
                63,
                &mut grad_rows,
            );
            record_size_benchmark::<GradientEncoding<U63, U24, U32>>(
                "GradientEncoding<U63, U24, U32>",
                &label,
                &data_u63,
                63,
                &mut grad_rows,
            );
            record_denoised_size_benchmark::<GradientEncoding<U63, U24, U32>, 2048>(
                "GradientEncoding<U63, U24, U32>",
                &label,
                &data_u63,
                63,
                &mut grad_rows,
            );
        }
    }

    md_table.push_str("### DiffEncoding Results\n\n");
    md_table.push_str(
        "<details>\n<summary>Click to expand DiffEncoding benchmark results table</summary>\n\n",
    );
    md_table.push_str("| Dataset | Encoding Scheme | Sample Count | Uncompressed Size | Compressed Size | Bits Per Sample | Compression Ratio |\n");
    md_table.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");
    for row in &diff_rows {
        if row.ratio >= 0.80 {
            use std::fmt::Write;
            let _ = writeln!(
                md_table,
                "| {} | `{}` | {} | {} bits | {} bits | {:.2} BPS | **{:.2}x** |",
                row.dataset_label,
                row.display_name,
                row.sample_count,
                row.uncompressed_bits,
                row.compressed_bits,
                row.bps,
                row.ratio
            );
        }
    }
    md_table.push_str("\n</details>\n\n");

    md_table.push_str("### GradientEncoding Results\n\n");
    md_table.push_str("<details>\n<summary>Click to expand GradientEncoding benchmark results table</summary>\n\n");
    md_table.push_str("| Dataset | Encoding Scheme | Sample Count | Uncompressed Size | Compressed Size | Bits Per Sample | Compression Ratio |\n");
    md_table.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");
    for row in &grad_rows {
        if row.ratio >= 0.80 {
            use std::fmt::Write;
            let _ = writeln!(
                md_table,
                "| {} | `{}` | {} | {} bits | {} bits | {:.2} BPS | **{:.2}x** |",
                row.dataset_label,
                row.display_name,
                row.sample_count,
                row.uncompressed_bits,
                row.compressed_bits,
                row.bps,
                row.ratio
            );
        }
    }
    md_table.push_str("\n</details>\n");

    std::fs::write("BENCHMARK_RESULTS.md", md_table).expect("Unable to write BENCHMARK_RESULTS.md");
}

criterion_group!(benches, bench_speed_and_compression);
criterion_main!(benches);
