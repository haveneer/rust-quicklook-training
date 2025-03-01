#![allow(unused_imports)]
#![allow(dead_code)]

use std::hint::black_box; // Use black_box from std (re-exported by Criterion and Iai as well)

use criterion::{criterion_group, criterion_main, Criterion};
use iai_callgrind::{library_benchmark, library_benchmark_group, main};

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
    fn diameter(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> &'static str {
        "Square"
    }

    fn diameter(&self) -> f64 {
        self.side * std::f64::consts::SQRT_2
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    fn name(&self) -> &'static str {
        "Circle"
    }

    fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}

#[cfg(not(feature = "iai"))]
fn static_no_box_dispatch(bencher: &mut Criterion) {
    let square: Square = Square { side: 1.0 };
    let circle: Circle = Circle { radius: 1.0 };
    bencher.bench_function("static_no_box_dispatch", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(&square).diameter());
                black_box(black_box(&circle).diameter());
            }
        })
    });
}

#[cfg(not(feature = "iai"))]
fn static_in_box_dispatch(bencher: &mut Criterion) {
    let square: Box<Square> = Box::new(Square { side: 1.0 }); // both in Box
    let circle: Box<Circle> = Box::new(Circle { radius: 1.0 }); // to make fair comparisons
    bencher.bench_function("static_in_box_dispatch", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(&square).diameter());
                black_box(black_box(&circle).diameter());
            }
        })
    });
}

#[cfg(not(feature = "iai"))]
fn dynamic_no_box_dispatch(bencher: &mut Criterion) {
    let square: &dyn Shape = &Square { side: 1.0 };
    let circle: &dyn Shape = &Circle { radius: 1.0 };
    bencher.bench_function("dynamic_no_box_dispatch", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(&square).diameter());
                black_box(black_box(&circle).diameter());
            }
        })
    });
}

#[cfg(not(feature = "iai"))]
fn dynamic_in_box_dispatch(bencher: &mut Criterion) {
    let square: Box<dyn Shape> = Box::new(Square { side: 1.0 }); // both in Box
    let circle: Box<dyn Shape> = Box::new(Circle { radius: 1.0 }); // to make fair comparisons
    bencher.bench_function("dynamic_in_box_dispatch", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(&square).diameter());
                black_box(black_box(&circle).diameter());
            }
        })
    });
}

#[cfg(not(feature = "iai"))]
criterion_group!(
    benches,
    static_no_box_dispatch,
    static_in_box_dispatch,
    dynamic_no_box_dispatch,
    dynamic_in_box_dispatch
);
#[cfg(not(feature = "iai"))]
criterion_main!(benches);

#[library_benchmark]
fn iai_static_no_box_dispatch() {
    let square: Square = Square { side: 1.0 };
    let circle: Circle = Circle { radius: 1.0 };
    for _ in 0..1000 {
        black_box(black_box(&square).diameter());
        black_box(black_box(&circle).diameter());
    }
}

#[library_benchmark]
fn iai_static_in_box_dispatch() {
    let square: Box<Square> = Box::new(Square { side: 1.0 }); // both in Box
    let circle: Box<Circle> = Box::new(Circle { radius: 1.0 }); // to make fair comparisons
    for _ in 0..1000 {
        black_box(black_box(&square).diameter());
        black_box(black_box(&circle).diameter());
    }
}

#[library_benchmark]
fn iai_dynamic_no_box_dispatch() {
    let square: &dyn Shape = &Square { side: 1.0 };
    let circle: &dyn Shape = &Circle { radius: 1.0 };
    for _ in 0..1000 {
        black_box(black_box(&square).diameter());
        black_box(black_box(&circle).diameter());
    }
}

#[library_benchmark]
fn iai_dynamic_in_box_dispatch() {
    let square: Box<dyn Shape> = Box::new(Square { side: 1.0 }); // both in Box
    let circle: Box<dyn Shape> = Box::new(Circle { radius: 1.0 }); // to make fair comparisons
    for _ in 0..1000 {
        black_box(black_box(&square).diameter());
        black_box(black_box(&circle).diameter());
    }
}

#[cfg(feature = "iai")]
library_benchmark_group!(name = bench_dispatch_group; benchmarks = iai_static_no_box_dispatch, iai_static_in_box_dispatch, iai_dynamic_no_box_dispatch, iai_dynamic_in_box_dispatch);
#[cfg(feature = "iai")]
main!(library_benchmark_groups = bench_dispatch_group);
