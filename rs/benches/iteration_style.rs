//! Itération **interne** (`fold`/`sum`, style « push ») contre itération **externe**
//! (boucle explicite de `next()`, style « pull ») sur des pipelines std identiques.
//!
//! C'est la question posée par Veedrac (« Rust's iterators are inefficient, and here's what we
//! can do about it », 2016) : `next()` force chaque adaptateur à reconstruire son état à chaque
//! élément, alors que `fold`/`try_fold` laissent l'adaptateur dérouler sa propre boucle.
//!
//! Les deux consommations calculent exactement la même chose ; seule la façon de tirer les
//! éléments change :
//!
//! ```ignore
//! // externe (pull)                       // interne (push)
//! let mut acc = 0;                        pipeline().fold(0, wrapping_add)
//! for x in pipeline() { acc += x; }
//! ```
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SIZE: usize = 1 << 14;
const INNER: usize = 16;

fn data() -> Vec<u32> {
    let mut x = 12_345_u32;
    (0..SIZE)
        .map(|_| {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            x >> 8
        })
        .collect()
}

#[inline]
fn add(acc: u32, x: u32) -> u32 {
    acc.wrapping_add(x)
}

/// Les pipelines mesurés (fonctions et non closures : elles renvoient un itérateur emprunté)
mod pipe {
    use super::Data;

    /// témoin : aucun adaptateur porteur d'état
    pub fn map(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter().map(|x| x.wrapping_mul(3))
    }

    /// `Filter::next` boucle jusqu'au prochain élément retenu
    pub fn filter(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter().copied().filter(|x| x % 3 == 0)
    }

    /// `Chain::next` re-teste quelle moitié est active à chaque élément
    pub fn chain(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter().chain(d.b.iter()).copied()
    }

    /// `Zip::next` doit retester les deux bornes
    pub fn zip(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter().zip(d.b.iter()).map(|(x, y)| x.wrapping_mul(*y))
    }

    /// `Zip` de `Chain` : le cas dur de l'article (état imbriqué)
    pub fn zip_chain(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter()
            .chain(d.b.iter())
            .zip(d.c.iter().chain(d.a.iter()))
            .map(|(x, y)| x.wrapping_add(*y))
    }

    /// `FlatMap::next` doit reprendre l'itérateur interne à chaque appel
    pub fn flatmap(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.nested.iter().flat_map(|inner| inner.iter()).copied()
    }
}

/// Applique `$mac` à chaque pipeline
macro_rules! pipelines {
    ($mac:ident) => {
        $mac!(map);
        $mac!(filter);
        $mac!(chain);
        $mac!(zip);
        $mac!(zip_chain);
        $mac!(flatmap);
    };
}

pub struct Data {
    a: Vec<u32>,
    b: Vec<u32>,
    c: Vec<u32>,
    nested: Vec<Vec<u32>>,
}

impl Data {
    fn new() -> Self {
        let a = data();
        let b = data().iter().map(|x| x ^ 0x5555_5555).collect();
        let c = data().iter().map(|x| x.wrapping_add(7)).collect();
        let nested = a.chunks(INNER).map(|c| c.to_vec()).collect();
        Self { a, b, c, nested }
    }
}

/// Itération externe : le consommateur pilote. Une boucle `for` est exactement cela —
/// elle se désucre en `loop { match it.next() { ... } }`.
#[inline(always)]
pub fn consume_external<I: Iterator<Item = u32>>(it: I) -> u32 {
    let mut acc = 0u32;
    for x in it {
        acc = add(acc, x);
    }
    acc
}

/// Itération interne : l'itérateur déroule sa propre boucle
#[inline(always)]
pub fn consume_internal<I: Iterator<Item = u32>>(it: I) -> u32 {
    it.fold(0u32, add)
}

macro_rules! bench_pipeline {
    ($name:ident) => {
        fn $name(c: &mut Criterion) {
            let d = Data::new();
            let case = concat!("iteration_", stringify!($name));

            c.bench_function(&format!("{case}-external_next"), |b| {
                b.iter(|| consume_external(pipe::$name(black_box(&d))))
            });
            c.bench_function(&format!("{case}-internal_fold"), |b| {
                b.iter(|| consume_internal(pipe::$name(black_box(&d))))
            });
        }
    };
}

pipelines!(bench_pipeline);

criterion_group!(
    iteration_style,
    map,
    filter,
    chain,
    zip,
    zip_chain,
    flatmap
);
criterion_main!(iteration_style);
