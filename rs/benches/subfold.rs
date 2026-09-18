//! L'adaptateur `subfold`: replier une source par blocs de `n` éléments.
//!
//! Ce fichier porte à la fois le matériel du cours (un type itérable, les écritures de
//! l'adaptateur, un trait d'extension, une démonstration) et sa mesure.
//!
//! Trois écritures du même adaptateur, toutes consommées par `sum()` (donc par `fold()`):
//! * `pull_next` : itération **externe** de bout en bout — `next()` tire la source
//!   élément par élément
//! * `pull_fold` : `next()` délègue le sous-bloc à `fold()` — itération **interne dans le bloc**
//! * `push_fold` : `pull_fold` + surcharge de `Iterator::fold` — itération interne de bout en bout
//!
//! Quatre sources, du plus simple au plus composite:
//! * `slice`   : `next()` trivial, tout s'inline — témoin, les 3 écritures doivent se valoir
//! * `filter`  : `Filter::next` teste le prédicat via `Option`, `Filter::fold` est spécialisé
//! * `chain4`  : `Chain::next` re-teste son état à chaque élément (imbriqué 4 fois)
//! * `flatmap` : `FlatMap::next` doit reprendre l'itérateur interne à chaque appel
//!
//! `cargo bench --bench subfold` mesure les temps; `cargo bench --bench subfold -- --test`
//! (ou `cargo test --benches`) exécute la démonstration et chaque mesure une fois.
//! Les écarts de temps sont petits et très sensibles au codegen: pour un chiffre reproductible,
//! compter les instructions avec `examples/subfold_instr.rs` (cf `benches/README.md`).
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

// ─────────────────────────────── le matériel du cours ───────────────────────────────

/// Un nouveau type itérable: une suite infinie d'entiers tirés au hasard.
struct RandomGenerator {
    rng: ThreadRng,
}

impl RandomGenerator {
    fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomGenerator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.r#gen::<i32>())
    }
}

/// Écriture « pull » naïve: itération externe, élément par élément.
mod pull_next {
    pub struct SubFold<I, B, F> {
        iter: I,
        n: usize,
        init: B,
        f: F,
    }

    impl<I, B, F> SubFold<I, B, F> {
        pub fn new(iter: I, n: usize, init: B, f: F) -> SubFold<I, B, F> {
            SubFold { iter, n, init, f }
        }
    }

    impl<I, B, F> Iterator for SubFold<I, B, F>
    where
        I: Iterator,
        B: Clone,
        F: Fn(B, I::Item) -> B,
    {
        type Item = B;

        fn next(&mut self) -> Option<Self::Item> {
            let mut acc = None;
            for _ in 0..self.n {
                if let Some(val) = self.iter.next() {
                    acc = acc.or(Some(self.init.clone())).map(|a| (self.f)(a, val));
                } else {
                    break;
                }
            }
            acc
        }
    }
}

/// Écriture de référence: `next()` replie le sous-bloc avec `fold` (itération interne
/// dans le bloc), et le trait d'extension ci-dessous ajoute la méthode `.subfold(…)`
/// à *tout* itérateur.
mod pull_fold {
    pub struct SubFold<I, B, F> {
        iter: I,
        n: usize,
        init: B,
        f: F,
    }

    impl<I, B, F> SubFold<I, B, F> {
        pub fn new(iter: I, n: usize, init: B, f: F) -> SubFold<I, B, F> {
            SubFold { iter, n, init, f }
        }
    }

    impl<I, B, F> Iterator for SubFold<I, B, F>
    where
        I: Iterator,
        B: Clone,
        F: Fn(B, I::Item) -> B,
    {
        type Item = B;

        fn next(&mut self) -> Option<Self::Item> {
            let mut subiter = self.iter.by_ref().take(self.n).peekable();
            if subiter.peek().is_some() {
                Some(subiter.fold(self.init.clone(), &self.f))
            } else {
                None
            }
        }
    }
}

/// Ajoute l'opération `subfold` à tout ce qui est `Iterator`
trait SubFoldable: Iterator {
    fn subfold<B, F>(self, n: usize, init: B, f: F) -> pull_fold::SubFold<Self, B, F>
    where
        Self: Sized,
        F: Fn(B, Self::Item) -> B,
        Self::Item: Clone,
    {
        pull_fold::SubFold::new(self, n, init, f)
    }
}

impl<T> SubFoldable for T where T: Iterator {}

/// Écriture « push »: `pull_fold` + surcharge de `fold()` (itération interne de bout en bout).
mod push_fold {
    pub struct SubFold<I, B, F> {
        iter: I,
        n: usize,
        init: B,
        f: F,
    }

    impl<I, B, F> SubFold<I, B, F> {
        pub fn new(iter: I, n: usize, init: B, f: F) -> SubFold<I, B, F> {
            SubFold { iter, n, init, f }
        }
    }

    impl<I, B, F> Iterator for SubFold<I, B, F>
    where
        I: Iterator,
        B: Clone,
        F: Fn(B, I::Item) -> B,
    {
        type Item = B;

        fn next(&mut self) -> Option<Self::Item> {
            // Extraction directe du 1er élément : arrêt immédiat si vide
            let first = self.iter.next()?;
            let acc = (self.f)(self.init.clone(), first);
            // On consomme les (n - 1) éléments restants avec fold
            Some(self.iter.by_ref().take(self.n - 1).fold(acc, &self.f))
        }

        // Surcharge critique pour annuler le surcoût de l'itération externe
        fn fold<Acc, FoldFn>(mut self, init: Acc, mut fold_fn: FoldFn) -> Acc
        where
            FoldFn: FnMut(Acc, Self::Item) -> Acc,
        {
            let mut accum = init;
            // Boucle directe sans passer par la machinerie de next()
            while let Some(first) = self.iter.next() {
                let chunk_acc = (self.f)(self.init.clone(), first);
                let chunk = self.iter.by_ref().take(self.n - 1).fold(chunk_acc, &self.f);
                accum = fold_fn(accum, chunk);
            }
            accum
        }
    }
}

/// Démonstration: l'équivalent de cette commande Unix, en une seule chaîne d'itérateurs
fn unix_pipeline() -> String {
    // Implementation of this Unix command
    // </dev/urandom          |
    //     tr -dc 'a-fA-F0-9' |
    //     head -c15          |
    //     fold -w 3          |
    //     paste -sd-
    RandomGenerator::new()
        .map(|x| (x % 256) as u8 as char)
        // .inspect(|x| println!("char: {x}")) // pour voir les caractères défiler
        .filter(|x| x.is_ascii_hexdigit())
        .take(15)
        .subfold(3, String::new(), |acc, x| acc + &x.to_string())
        .fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s
            } else {
                acc + "-" + &s
            }
        })
    // .collect::<Vec<String>>().join("-") // stable, mais avec une allocation intermédiaire
    // .intersperse("-".into()).collect::<String>() // nightly
}

// Enregistrée dans le `criterion_group!` mais ne mesure rien: elle est exécutée une fois par
// `cargo bench` et par `cargo test --benches` (un bench `harness = false` n'exécute aucun `#[test]`).
fn unix_pipeline_demo(_c: &mut Criterion) {
    let result = unix_pipeline();
    println!(">> {result}");

    let groups: Vec<&str> = result.split('-').collect();
    assert_eq!(groups.len(), 5, "15 caractères repliés par blocs de 3");
    assert!(groups
        .iter()
        .all(|g| g.len() == 3 && g.chars().all(|c| c.is_ascii_hexdigit())));
}

// ───────────────────────────────────── la mesure ─────────────────────────────────────

const SIZE: usize = 1 << 14;
/// Tailles de sous-bloc repliées par `subfold`
const CHUNK_SIZES: [usize; 3] = [2, 8, 64];

/// Données déterministes (pas de RNG dans la boucle de mesure)
fn data() -> Vec<u64> {
    let mut x = 12_345_u64;
    (0..SIZE)
        .map(|_| {
            x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            x >> 33
        })
        .collect()
}

/// Mélange non associatif : le résultat dépend du découpage en sous-blocs,
/// ce qui interdit à LLVM de fusionner les sous-folds en une seule réduction.
#[inline]
fn combine(acc: u64, x: u64) -> u64 {
    acc.rotate_left(7).wrapping_mul(31).wrapping_add(x)
}

/// Les 4 sources mesurées (fonctions plutôt que closures : elles renvoient un itérateur emprunté)
mod src {
    use super::SIZE;

    pub fn slice(v: &[u64]) -> impl Iterator<Item = u64> + '_ {
        v.iter().copied()
    }

    pub fn filter(v: &[u64]) -> impl Iterator<Item = u64> + '_ {
        v.iter().copied().filter(|x| x & 1 == 0)
    }

    pub fn chain4(v: &[u64]) -> impl Iterator<Item = u64> + '_ {
        let (a, b) = v.split_at(SIZE / 2);
        let (a1, a2) = a.split_at(SIZE / 4);
        let (b1, b2) = b.split_at(SIZE / 4);
        a1.iter().chain(a2).chain(b1).chain(b2).copied()
    }

    pub fn flatmap(v: &[u64]) -> impl Iterator<Item = u64> + '_ {
        v.chunks(64).flat_map(|c| c.iter()).copied()
    }
}

/// Applique `$mac` aux 4 sources
macro_rules! for_each_source {
    ($mac:ident) => {
        $mac!(slice);
        $mac!(filter);
        $mac!(chain4);
        $mac!(flatmap);
    };
}

macro_rules! bench_source {
    ($src:ident) => {
        fn $src(c: &mut Criterion) {
            let v = data();

            for n in CHUNK_SIZES {
                let case = format!(concat!("subfold_", stringify!($src), "_n{}"), n);

                c.bench_function(&format!("{case}-pull_next"), |b| {
                    b.iter(|| {
                        pull_next::SubFold::new(src::$src(black_box(&v)), n, 0u64, combine)
                            .sum::<u64>()
                    })
                });
                c.bench_function(&format!("{case}-pull_fold"), |b| {
                    b.iter(|| {
                        pull_fold::SubFold::new(src::$src(black_box(&v)), n, 0u64, combine)
                            .sum::<u64>()
                    })
                });
                c.bench_function(&format!("{case}-push_fold"), |b| {
                    b.iter(|| {
                        push_fold::SubFold::new(src::$src(black_box(&v)), n, 0u64, combine)
                            .sum::<u64>()
                    })
                });
                // Référence : découpage explicite par tranches (pas d'adaptateur générique)
                c.bench_function(&format!("{case}-chunks_ref"), |b| {
                    b.iter(|| {
                        black_box(&v)
                            .chunks(n)
                            .map(|chunk| chunk.iter().copied().fold(0u64, combine))
                            .sum::<u64>()
                    })
                });
            }
        }
    };
}

for_each_source!(bench_source);

criterion_group!(subfold, unix_pipeline_demo, slice, filter, chain4, flatmap);
criterion_main!(subfold);
