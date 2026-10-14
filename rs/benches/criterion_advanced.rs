//! Criterion beyond the first bench: parametric inputs, setup excluded from
//! the measurement, group settings.
//!
//! cargo bench --bench criterion_advanced -- --save-baseline before
//! cargo bench --bench criterion_advanced -- --baseline before   (after a change)
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::time::Duration;

/// Deterministic pseudo-random values (no rand: reproducible measurements)
fn values(n: usize) -> Vec<u64> {
    let mut x: u64 = 0x2545_F491_4F6C_DD1D;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        })
        .collect()
}

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort");
    group
        .sample_size(50) // default: 100
        .measurement_time(Duration::from_secs(3)) // default: 5 s
        .noise_threshold(0.03); // changes below 3 % are reported as noise

    for n in [1_000, 10_000, 100_000] {
        let input = values(n);
        group.throughput(Throughput::Elements(n as u64)); // also reports elements/s
        group.bench_with_input(BenchmarkId::new("stable", n), &input, |b, input| {
            // The clone is the setup: excluded from the measured time
            b.iter_batched(|| input.clone(), |mut v| v.sort(), BatchSize::LargeInput)
        });
        group.bench_with_input(BenchmarkId::new("unstable", n), &input, |b, input| {
            b.iter_batched(
                || input.clone(),
                |mut v| v.sort_unstable(),
                BatchSize::LargeInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);
