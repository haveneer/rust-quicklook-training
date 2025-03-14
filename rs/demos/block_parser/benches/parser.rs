#[cfg(not(feature = "iai"))]
use criterion::{criterion_group, criterion_main, Criterion};

use block_parser::block_iter::BlockViewIterator;
use block_parser::{
    make_test_block, Block, FromBytes, OwnedBlock, RefBlock, ToBytes, Transaction, Transactions,
};
use iai_callgrind::library_benchmark;
use std::hint::black_box;

const N: usize = 10;

fn make_block_bytes() -> Vec<u8> {
    let mut v = Vec::new();
    let one_block_bytes = make_test_block().to_bytes();
    for _ in 0..N {
        v.extend_from_slice(&one_block_bytes);
    }
    v
}

// trait Process {
//     type Output;
//     fn process(&self, data: &[u8]) -> Self::Output;
// }
//
// struct CheckSum;
//
// impl Process for CheckSum {
//     type Output = u64;
//
//     fn process(&self, transaction_data: &[u8]) -> Self::Output {
//         transaction_data.iter().fold(0, |acc, x| acc + *x as u64)
//     }
// }
//
// fn static_process_block(data: &[u8], process: impl Process<Output = u64>) {
//     let (block, _offset) = BlockRef::from_bytes(data).unwrap();
//     block.transactions().iter().for_each(|t| {
//         process.process(t.data());
//     });
// }
//
// fn dynamic_process_block(data: &[u8], process: &dyn Process<Output = u64>) {
//     let (block, _offset) = BlockRef::from_bytes(data).unwrap();
//     block.transactions().iter().for_each(|t| {
//         process.process(t.data());
//     });
// }

fn check_data(data: &[u8]) -> u64 {
    data.iter().fold(0, |acc, x| acc + *x as u64)
}

#[cfg(not(feature = "iai"))]
fn bench_ref_block_from_bytes(c: &mut Criterion) {
    c.bench_function("RefBlock::from_bytes", |b| {
        let buf = make_block_bytes();
        b.iter(|| {
            let mut offset = 0;
            for _ in 0..N {
                let (block_ref, consumed) =
                    RefBlock::from_bytes(black_box(&buf[offset..])).unwrap();
                offset += consumed;
                let mut acc: u64 = 0;
                for t in block_ref.transactions().iter() {
                    acc += check_data(black_box(t.data()));
                }
                black_box((block_ref, consumed, acc));
            }
        });
    });
}

#[cfg(not(feature = "iai"))]
fn bench_owned_block_from_bytes(c: &mut Criterion) {
    c.bench_function("OwnedBlock::from_bytes (owning)", |b| {
        let buf = make_block_bytes();
        b.iter(|| {
            let mut offset = 0;
            for _ in 0..N {
                let (block, consumed) = OwnedBlock::from_bytes(black_box(&buf[offset..])).unwrap();
                offset += consumed;
                let mut acc: u64 = 0;
                for t in block.transactions().iter() {
                    acc += check_data(black_box(t.data()));
                }
                black_box((block, consumed, acc));
            }
        });
    });
}

#[cfg(not(feature = "iai"))]
fn bench_block_iter_from_bytes(c: &mut Criterion) {
    c.bench_function("BlockIterator", |b| {
        let buf = make_block_bytes();
        b.iter(|| {
            let mut iter = BlockViewIterator::new(black_box(&buf));
            for _ in 0..N {
                let block = iter.next().unwrap().unwrap();
                let mut acc: u64 = 0;
                for t in block.transactions() {
                    acc += check_data(black_box(t.data()));
                }
                black_box((block, acc));
            }
        });
    });
}

#[cfg(not(feature = "iai"))]
criterion_group!(
    benches,
    bench_ref_block_from_bytes,
    bench_owned_block_from_bytes,
    bench_block_iter_from_bytes,
);
#[cfg(not(feature = "iai"))]
criterion_main!(benches);

// setup/teardown: https://iai-callgrind.github.io/iai-callgrind/latest/html/benchmarks/library_benchmarks/setup_and_teardown.html

#[library_benchmark]
#[bench::one_block(setup = make_block_bytes)]
fn iai_ref_block_from_bytes(buf: Vec<u8>) {
    let (ref_block, consumed) = RefBlock::from_bytes(black_box(&buf)).unwrap();
    let mut acc: u64 = 0;
    for t in ref_block.transactions().iter() {
        acc += check_data(black_box(t.data()));
    }
    black_box((ref_block, consumed, acc));
}

#[library_benchmark]
#[bench::one_block(setup = make_block_bytes)]
fn iai_owned_block_from_bytes(buf: Vec<u8>) {
    let (owned_block, consumed) = OwnedBlock::from_bytes(black_box(&buf)).unwrap();
    let mut acc: u64 = 0;
    for t in owned_block.transactions().iter() {
        acc += check_data(black_box(t.data()));
    }
    black_box((owned_block, consumed, acc));
}

#[library_benchmark]
#[bench::one_block(setup = make_block_bytes)]
fn iai_block_iter_from_bytes(buf: Vec<u8>) {
    let mut iter = BlockViewIterator::new(black_box(&buf));
    let block = iter.next().unwrap().unwrap();
    let mut acc: u64 = 0;
    for t in block.transactions() {
        acc += check_data(black_box(t.data()));
    }
    black_box((iter, block, acc));
}

#[cfg(feature = "iai")]
iai_callgrind::library_benchmark_group!(name = bench_parser_group; benchmarks = iai_owned_block_from_bytes, iai_ref_block_from_bytes, iai_block_iter_from_bytes);
#[cfg(feature = "iai")]
iai_callgrind::main!(library_benchmark_groups = bench_parser_group);
