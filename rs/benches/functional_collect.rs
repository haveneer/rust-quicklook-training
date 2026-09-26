//! Le fonctionnel gagne (1/2) : construire un `Vec` à partir d'un autre.
//!
//! `collect()` en sait plus que la boucle impérative :
//! * `iter().map(f)` a une taille exacte (`TrustedLen`) : `collect` alloue **une seule fois**,
//!   là où `push` sur un `Vec::new()` réalloue (et recopie) ~log₂ n fois ;
//! * `into_iter().map(f).collect()` consomme la source : si le type produit a la même taille et
//!   le même alignement, la bibliothèque standard **réutilise le buffer source** (« in-place
//!   collect ») — zéro allocation, même quand le type change (`u64` → `f64`), ce qu'aucune boucle
//!   en Rust *safe* ne sait faire ;
//! * ça tient encore avec `filter` : le buffer source est réutilisé puis rétréci.
//!
//! Mais l'ordre n'est pas toujours celui qu'on attend : sur de gros volumes, `iter().map().collect()`
//! (nouveau buffer) peut battre la version en place, dont la boucle lit et écrit le même buffer.
//! Et avec `filter` (taille inconnue), `iter().filter().collect()` perd contre une boucle avec
//! `with_capacity` : l'avantage du fonctionnel vient de ce que l'itérateur *sait*, pas du style.
//!
//! La source est reclonée hors mesure (`iter_batched`) pour chaque itération, et les sorties
//! sont libérées hors mesure. La libération de la source, elle, est mesurée : elle fait partie du
//! coût réel de la version qui ne la réutilise pas.
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

const SIZES: [usize; 2] = [1 << 10, 1 << 20];

fn data(n: usize) -> Vec<u64> {
    let mut x = 12_345_u64;
    (0..n)
        .map(|_| {
            x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            x >> 11
        })
        .collect()
}

#[inline]
fn f(x: u64) -> f64 {
    x as f64 * 0.5
}

#[inline]
fn keep(x: &u64) -> bool {
    x % 3 != 0
}

mod map {
    use super::f;

    pub fn imperative_push(v: Vec<u64>) -> Vec<f64> {
        let mut out = Vec::new();
        for x in v {
            out.push(f(x));
        }
        out
    }

    pub fn imperative_with_capacity(v: Vec<u64>) -> Vec<f64> {
        let mut out = Vec::with_capacity(v.len());
        for x in v {
            out.push(f(x));
        }
        out
    }

    pub fn functional_iter_collect(v: Vec<u64>) -> Vec<f64> {
        v.iter().map(|&x| f(x)).collect()
    }

    pub fn functional_into_iter_collect(v: Vec<u64>) -> Vec<f64> {
        v.into_iter().map(f).collect()
    }
}

mod filter {
    use super::{f, keep};

    pub fn imperative_push(v: Vec<u64>) -> Vec<f64> {
        let mut out = Vec::new();
        for x in v {
            if keep(&x) {
                out.push(f(x));
            }
        }
        out
    }

    pub fn imperative_with_capacity(v: Vec<u64>) -> Vec<f64> {
        let mut out = Vec::with_capacity(v.len());
        for x in v {
            if keep(&x) {
                out.push(f(x));
            }
        }
        out
    }

    pub fn functional_iter_collect(v: Vec<u64>) -> Vec<f64> {
        v.iter().filter(|x| keep(x)).map(|&x| f(x)).collect()
    }

    pub fn functional_into_iter_collect(v: Vec<u64>) -> Vec<f64> {
        v.into_iter().filter(keep).map(f).collect()
    }
}

/// Applique `$mac` à chaque variante
macro_rules! variants {
    ($mac:ident, $group:expr, $src:expr, $n:expr, $case:ident) => {
        $mac!($group, $src, $n, $case, imperative_push);
        $mac!($group, $src, $n, $case, imperative_with_capacity);
        $mac!($group, $src, $n, $case, functional_iter_collect);
        $mac!($group, $src, $n, $case, functional_into_iter_collect);
    };
}

macro_rules! bench_variant {
    ($group:expr, $src:expr, $n:expr, $case:ident, $variant:ident) => {
        $group.bench_with_input(
            BenchmarkId::new(stringify!($variant), $n),
            $src,
            |b, src| {
                b.iter_batched(
                    || src.clone(),
                    |v| $case::$variant(black_box(v)),
                    BatchSize::LargeInput,
                )
            },
        );
    };
}

macro_rules! bench_case {
    ($case:ident) => {
        fn $case(c: &mut Criterion) {
            let mut group = c.benchmark_group(concat!("functional_collect_", stringify!($case)));
            for n in SIZES {
                let src = data(n);
                variants!(bench_variant, group, &src, n, $case);
            }
            group.finish();
        }
    };
}

bench_case!(map);
bench_case!(filter);

/// Toutes les variantes calculent la même chose
fn check() {
    let v = data(1000);
    let reference = map::imperative_push(v.clone());
    assert_eq!(reference, map::imperative_with_capacity(v.clone()));
    assert_eq!(reference, map::functional_iter_collect(v.clone()));
    assert_eq!(reference, map::functional_into_iter_collect(v.clone()));

    let reference = filter::imperative_push(v.clone());
    assert_eq!(reference, filter::imperative_with_capacity(v.clone()));
    assert_eq!(reference, filter::functional_iter_collect(v.clone()));
    assert_eq!(reference, filter::functional_into_iter_collect(v.clone()));

    // « in-place collect » : le buffer de sortie est celui de la source
    let src = v.clone();
    let ptr = src.as_ptr() as usize;
    let result = map::functional_into_iter_collect(src);
    assert_eq!(ptr, result.as_ptr() as usize);

    let src = v;
    let ptr = src.as_ptr() as usize;
    let result = filter::functional_into_iter_collect(src);
    assert_eq!(ptr, result.as_ptr() as usize);
}

fn checked(c: &mut Criterion) {
    check();
    map(c);
    filter(c);
}

criterion_group!(functional_collect, checked);
criterion_main!(functional_collect);
