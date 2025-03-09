use advanced::{Block, BlockRef};

#[cfg(not(feature = "iai"))]
use criterion::{criterion_group, criterion_main, Criterion};

use iai_callgrind::library_benchmark;
use std::hint::black_box;

fn make_test_block() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&100u64.to_le_bytes());
    buf.extend_from_slice(&2u32.to_le_bytes());

    buf.extend_from_slice(&3u32.to_le_bytes());
    buf.extend_from_slice(&[10, 20, 30]);

    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&[40, 50]);

    buf.extend_from_slice(&[0u8; 32]);

    buf
}

#[cfg(not(feature = "iai"))]
fn bench_blockref_from_bytes(c: &mut Criterion) {
    let buf = make_test_block();
    c.bench_function("BlockRef::from_bytes_unsafe", |b| {
        b.iter(|| {
            let (block_ref, consumed) = BlockRef::from_bytes_unsafe(black_box(&buf)).unwrap();
            black_box((block_ref, consumed));
        });
    });
}

#[cfg(not(feature = "iai"))]
fn bench_blockref_from_bytes_old(c: &mut Criterion) {
    let buf = make_test_block();
    c.bench_function("BlockRef::from_bytes", |b| {
        b.iter(|| {
            let (block_ref, consumed) = BlockRef::from_bytes(black_box(&buf)).unwrap();
            black_box((block_ref, consumed));
        });
    });
}

#[cfg(not(feature = "iai"))]
fn bench_block_from_bytes(c: &mut Criterion) {
    let buf = make_test_block();
    c.bench_function("Block::from_bytes (owning)", |b| {
        b.iter(|| {
            let (block, consumed) = Block::from_bytes(black_box(&buf)).unwrap();
            black_box((block, consumed));
        });
    });
}

#[cfg(not(feature = "iai"))]
criterion_group!(
    benches,
    bench_blockref_from_bytes,
    bench_blockref_from_bytes_old,
    bench_block_from_bytes
);
#[cfg(not(feature = "iai"))]
criterion_main!(benches);

#[library_benchmark]
fn iai_blockref_from_bytes() {
    let buf = make_test_block();
    let (block_ref, consumed) = BlockRef::from_bytes(black_box(&buf)).unwrap();
    black_box((block_ref, consumed));
}

#[library_benchmark]
fn iai_blockref_from_bytes_unsafe() {
    let buf = make_test_block();
    let (block_ref, consumed) = BlockRef::from_bytes_unsafe(black_box(&buf)).unwrap();
    black_box((block_ref, consumed));
}

#[library_benchmark]
fn iai_block_from_bytes() {
    let buf = make_test_block();
    let (block, consumed) = Block::from_bytes(black_box(&buf)).unwrap();
    black_box((block, consumed));
}

#[cfg(feature = "iai")]
iai_callgrind::library_benchmark_group!(name = bench_parser_group; benchmarks = iai_block_from_bytes, iai_blockref_from_bytes, iai_blockref_from_bytes_unsafe);
#[cfg(feature = "iai")]
iai_callgrind::main!(library_benchmark_groups = bench_parser_group);
