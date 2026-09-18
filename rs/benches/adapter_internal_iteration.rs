//! Écrire un adapter : ce que coûte de n'implémenter que `next()`.
//!
//! La bibliothèque standard a appliqué la leçon de Veedrac (« Rust's iterators are inefficient,
//! and here's what we can do about it », 2016) : ses adaptateurs surchargent `fold`/`try_fold`
//! pour dérouler leur propre boucle. **Mais un adapter écrit à la main casse cette chaîne** :
//! son `fold` par défaut est une boucle de `next()`, donc la source est de nouveau tirée
//! élément par élément et ses spécialisations ne servent plus à rien.
//!
//! Le même adapter (`Scale`, un `map` spécialisé) est écrit de deux façons :
//! * `next_only`     : seul `next()` est implémenté
//! * `with_fold`     : `next()` + `fold` délégué à la source
//! * `with_fold_find`: + `find` délégué (ce qu'on peut faire sur stable faute de `try_fold`)
//!
//! La `std` surcharge aussi `try_fold` (ce qui accélère `find`, `any`, `position`…), mais ce
//! trait est instable : un adapter écrit à la main ne peut pas le faire sur stable.
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

/// `fold` **et** `find` délégués : sur stable, `try_fold` n'est pas surchargeable, mais
/// `find`, `find_map`, `any`, `all`, `position`… ont des signatures sans `Try` et peuvent
/// être redirigées une par une vers la version spécialisée de la source.
mod with_fold_find {
    declare_scale!(with_fold_find);

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

        fn find<P>(&mut self, mut predicate: P) -> Option<u32>
        where
            P: FnMut(&u32) -> bool,
        {
            let k = self.k;
            // `find_map` de la source passe par son `try_fold` spécialisé
            self.iter.find_map(|x| {
                let y = scale(x, k);
                predicate(&y).then_some(y)
            })
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

/// Valeur absente : `find` parcourt donc tout le flux (pas de sortie anticipée fortuite)
const ABSENT: u32 = u32::MAX;

macro_rules! bench_source {
    ($src:ident) => {
        fn $src(c: &mut Criterion) {
            let d = Data::new();
            let case = concat!("adapter_", stringify!($src));

            // consommateur non court-circuitant : `sum()` passe par `fold`
            c.bench_function(&format!("{case}_sum-next_only"), |b| {
                b.iter(|| next_only::Scale::new(src::$src(black_box(&d)), K).sum::<u32>())
            });
            c.bench_function(&format!("{case}_sum-with_fold"), |b| {
                b.iter(|| with_fold::Scale::new(src::$src(black_box(&d)), K).sum::<u32>())
            });
            c.bench_function(&format!("{case}_sum-with_fold_find"), |b| {
                b.iter(|| with_fold_find::Scale::new(src::$src(black_box(&d)), K).sum::<u32>())
            });

            // référence : le `map` de la `std`, qui surcharge `fold` *et* `try_fold`
            c.bench_function(&format!("{case}_sum-std_map"), |b| {
                b.iter(|| {
                    src::$src(black_box(&d))
                        .map(|x| x.wrapping_mul(K).rotate_left(5))
                        .sum::<u32>()
                })
            });

            // consommateur court-circuitant : `find` passe par `try_fold`
            c.bench_function(&format!("{case}_find-next_only"), |b| {
                b.iter(|| next_only::Scale::new(src::$src(black_box(&d)), K).find(|x| *x == ABSENT))
            });
            c.bench_function(&format!("{case}_find-with_fold"), |b| {
                b.iter(|| with_fold::Scale::new(src::$src(black_box(&d)), K).find(|x| *x == ABSENT))
            });
            c.bench_function(&format!("{case}_find-with_fold_find"), |b| {
                b.iter(|| {
                    with_fold_find::Scale::new(src::$src(black_box(&d)), K).find(|x| *x == ABSENT)
                })
            });
            c.bench_function(&format!("{case}_find-std_map"), |b| {
                b.iter(|| {
                    src::$src(black_box(&d))
                        .map(|x| x.wrapping_mul(K).rotate_left(5))
                        .find(|x| *x == ABSENT)
                })
            });
        }
    };
}

bench_source!(slice);
bench_source!(filter);
bench_source!(chain);

criterion_group!(adapter_internal_iteration, slice, filter, chain);
criterion_main!(adapter_internal_iteration);
