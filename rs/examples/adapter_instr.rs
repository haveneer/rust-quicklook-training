//! Comptage d'instructions (callgrind) pour `benches/adapter_internal_iteration.rs`.
//! usage: adapter_instr <slice|filter|chain> <next_only|with_fold|baseline> <sum|find>
#![allow(dead_code)]
use std::hint::black_box;

const SIZE: usize = 1 << 14;
const K: u32 = 2_654_435_761;

macro_rules! declare_scale {
    ($name:ident) => {
        pub struct Scale<I> {
            pub iter: I,
            pub k: u32,
        }

        impl<I> Scale<I> {
            pub fn new(iter: I, k: u32) -> Self {
                Self { iter, k }
            }
        }

        #[inline]
        fn scale(x: u32, k: u32) -> u32 {
            x.wrapping_mul(k).rotate_left(5)
        }
    };
}

/// Seul `next()` est implémenté : `fold`, `try_fold`, `sum`, `find`… retombent sur la
/// boucle de `next()` par défaut — la source est tirée élément par élément.
mod next_only {
    declare_scale!(next_only);

    impl<I: Iterator<Item = u32>> Iterator for Scale<I> {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            self.iter.next().map(|x| scale(x, self.k))
        }
    }
}

/// `fold` est délégué à la source : les consommateurs non court-circuitants
/// (`sum`, `for_each`, `collect`…) profitent de la boucle interne de la source.
mod with_fold {
    declare_scale!(with_fold);

    impl<I: Iterator<Item = u32>> Iterator for Scale<I> {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            self.iter.next().map(|x| scale(x, self.k))
        }

        fn fold<B, F>(self, init: B, mut f: F) -> B
        where
            F: FnMut(B, u32) -> B,
        {
            let k = self.k;
            self.iter.fold(init, |acc, x| f(acc, scale(x, k)))
        }
    }
}

// Sur stable, un adapter écrit à la main ne peut **pas** surcharger `try_fold` :
// le trait `std::ops::Try` est encore instable (try_trait_v2, issue #84277). Les consommateurs
// court-circuitants (`find`, `any`, `position`, `try_for_each`…) retombent donc toujours sur la
// boucle de `next()` — c'est la moitié du protocole d'itération interne qui reste réservée à la
// `std`. Le bench `*_find-*` ci-dessous le montre.

fn data() -> Vec<u32> {
    let mut x = 12_345_u32;
    (0..SIZE)
        .map(|_| {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            x >> 8
        })
        .collect()
}

pub struct Data {
    a: Vec<u32>,
    b: Vec<u32>,
}

impl Data {
    fn new() -> Self {
        let a = data();
        let b = data().iter().map(|x| x ^ 0x5555_5555).collect();
        Self { a, b }
    }
}

/// Sources placées *avant* l'adapter : leur `fold`/`try_fold` est spécialisé par la `std`
mod src {
    use super::Data;

    /// témoin : `next()` trivial, rien à gagner
    pub fn slice(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter().copied()
    }

    /// `Filter::next` boucle jusqu'au prochain élément retenu
    pub fn filter(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter().copied().filter(|x| x % 3 == 0)
    }

    /// `Chain::next` re-teste quelle moitié est active à chaque élément
    pub fn chain(d: &Data) -> impl Iterator<Item = u32> + '_ {
        d.a.iter().chain(d.b.iter()).copied()
    }
}


/// Valeur absente : `find` parcourt donc tout le flux
const ABSENT: u32 = u32::MAX;

fn run(source: &str, variant: &str, consumer: &str, d: &Data) -> u64 {
    macro_rules! dispatch {
        ($mk:expr) => {
            match (variant, consumer) {
                ("next_only", "sum") => next_only::Scale::new($mk, K).sum::<u32>() as u64,
                ("with_fold", "sum") => with_fold::Scale::new($mk, K).sum::<u32>() as u64,
                ("next_only", "find") => {
                    next_only::Scale::new($mk, K).find(|x| *x == ABSENT).unwrap_or(0) as u64
                }
                ("with_fold", "find") => {
                    with_fold::Scale::new($mk, K).find(|x| *x == ABSENT).unwrap_or(0) as u64
                }
                ("baseline", _) => d.a.len() as u64,
                other => panic!("combinaison inconnue: {other:?}"),
            }
        };
    }
    match source {
        "slice" => dispatch!(src::slice(d)),
        "filter" => dispatch!(src::filter(d)),
        "chain" => dispatch!(src::chain(d)),
        other => panic!("source inconnue: {other}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let d = black_box(Data::new());
    let mut acc = 0u64;
    for _ in 0..black_box(20) {
        acc = acc.wrapping_add(black_box(run(&args[1], &args[2], &args[3], black_box(&d))));
    }
    println!("{acc}");
}
