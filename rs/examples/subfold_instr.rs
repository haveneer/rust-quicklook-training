//! Comptage d'instructions (callgrind) pour les variantes de `subfold` du bench
//! `benches/subfold.rs` : déterministe, contrairement au temps mesuré par criterion.
//! Procédure complète (Docker + valgrind) : voir `benches/README.md`.
//!
//! Les 3 écritures ci-dessous sont une **copie** de celles de `benches/subfold.rs`
//! (un bench criterion ne peut pas être importé) : les garder synchronisées.
//!
//! usage: subfold_instr <slice|filter|chain4|flatmap> <pull_next|pull_fold|push_fold|baseline> <n>
#![allow(dead_code)]
use std::hint::black_box;

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


fn run(source: &str, variant: &str, v: &[u64], n: usize) -> u64 {
    macro_rules! dispatch_variant {
        ($mk:expr) => {
            match variant {
                "pull_next" => pull_next::SubFold::new($mk, n, 0u64, combine).sum::<u64>(),
                "pull_fold" => pull_fold::SubFold::new($mk, n, 0u64, combine).sum::<u64>(),
                "push_fold" => push_fold::SubFold::new($mk, n, 0u64, combine).sum::<u64>(),
                "baseline" => v.len() as u64,
                other => panic!("variante inconnue: {other}"),
            }
        };
    }
    match source {
        "slice" => dispatch_variant!(src::slice(v)),
        "filter" => dispatch_variant!(src::filter(v)),
        "chain4" => dispatch_variant!(src::chain4(v)),
        "flatmap" => dispatch_variant!(src::flatmap(v)),
        other => panic!("source inconnue: {other}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(3).map_or(8, |a| a.parse().unwrap());
    let v = black_box(data());
    let mut acc = 0u64;
    for _ in 0..black_box(20) {
        acc = acc.wrapping_add(black_box(run(&args[1], &args[2], black_box(&v), black_box(n))));
    }
    println!("{acc}");
}
