// C++ -> Rust: the mapping of sim::Field is *generated* by autocxx from
// cpp/field.hpp (no declaration by hand, unlike ../cxx-interface).
// Rust -> C++: see src/bridge.rs.
use autocxx::prelude::*;
use cxx::{let_cxx_string, UniquePtr};

include_cpp! {
    #include "field.hpp"
    name!(cpp)
    safety!(unsafe_ffi)
    generate!("sim::Field") // whole class: ctor, methods, UniquePtr support...
}

mod bridge;

use bridge::Stats;
use cpp::sim::Field;

// Ownership C++ -> Rust -> C++: the object is modified in place and given back.
fn smooth(mut field: UniquePtr<Field>, passes: usize) -> Result<UniquePtr<Field>, String> {
    let Some(mut f) = field.as_mut() else {
        return Err("smooth: null field".into());
    };
    let_cxx_string!(name = format!("smooth{passes}({})", f.name()));
    f.as_mut().set_name(&name);

    // std::vector<double>& -> Pin<&mut CxxVector<f64>> -> &mut [f64] (zero copy)
    let values = f.values_mut().as_mut_slice();
    let n = values.len();
    for _ in 0..passes {
        let prev = values.to_vec();
        // 3-point moving average, boundaries kept
        for (v, w) in values[1..n.saturating_sub(1)]
            .iter_mut()
            .zip(prev.windows(3))
        {
            *v = w.iter().sum::<f64>() / 3.0;
        }
    }
    Ok(field)
}

// Borrowed C++ object in, brand new C++ object out.
fn gradient(field: &Field, dx: f64) -> Result<UniquePtr<Field>, String> {
    let n = field.size();
    if n < 2 {
        return Err(format!("gradient: field of size {n} is too small"));
    }
    let src = field.values().as_slice();
    let mut grad = Field::new(n).within_unique_ptr(); // C++ constructor, called from Rust
    let_cxx_string!(name = format!("d({})", field.name()));
    grad.pin_mut().set_name(&name);
    let dst = grad.pin_mut().values_mut().as_mut_slice();
    // centered differences inside, one-sided at the boundaries
    dst[0] = (src[1] - src[0]) / dx;
    dst[n - 1] = (src[n - 1] - src[n - 2]) / dx;
    for (d, w) in dst[1..n - 1].iter_mut().zip(src.windows(3)) {
        *d = (w[2] - w[0]) / (2.0 * dx);
    }
    Ok(grad)
}

// Shared struct (defined in the bridge) returned by value.
fn stats(field: &Field) -> Result<Stats, String> {
    let values = field.values().as_slice();
    if values.is_empty() {
        return Err("stats: empty field".into());
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let var = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let (min, max) = values
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| {
            (lo.min(x), hi.max(x))
        });
    Ok(Stats {
        mean,
        stddev: var.sqrt(),
        min,
        max,
    })
}
