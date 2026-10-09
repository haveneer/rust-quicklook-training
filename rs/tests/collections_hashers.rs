use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, RandomState};

/// Same table, the hash function is only a type parameter (BuildHasher)
fn build<S: BuildHasher + Default>(n: u64) -> HashMap<u64, u64, S> {
    (0..n).map(|i| (i, i * i)).collect()
}

fn main() {
    let sip: HashMap<u64, u64, RandomState> = build(1000); // std default: SipHash 1-3
    let fx: rustc_hash::FxHashMap<u64, u64> = build(1000); // rustc's FxHash
    let fold: foldhash::HashMap<u64, u64> = build(1000); // hashbrown's default
    let a: HashMap<u64, u64, ahash::RandomState> = build(1000);
    let fnv: HashMap<u64, u64, fnv::FnvBuildHasher> = build(1000);
    let xxh3: HashMap<u64, u64, BuildHasherDefault<twox_hash::XxHash3_64>> = build(1000);
    let rapid: rapidhash::RapidHashMap<u64, u64> = build(1000);
    let id: nohash_hasher::IntMap<u64, u64> = build(1000); // hash(k) = k
    for map in [sip.get(&42), fx.get(&42), fold.get(&42), a.get(&42)] {
        assert_eq!(map, Some(&1764));
    }
    for map in [fnv.get(&42), xxh3.get(&42), rapid.get(&42), id.get(&42)] {
        assert_eq!(map, Some(&1764));
    }

    // Seeded hashers (SipHash, foldhash, ahash, rapidhash) draw a random key per
    // table: an attacker cannot precompute colliding keys. FxHash, FNV are fixed:
    let siphash = |s: RandomState| s.hash_one("mesh");
    let (k1, k2) = (siphash(RandomState::new()), siphash(RandomState::new()));
    println!("SipHash, 2 tables: {k1:x} {k2:x}");
    println!(
        "FxHash, always   : {:x}",
        rustc_hash::FxBuildHasher.hash_one("mesh")
    );
}

#[test]
fn test() {
    main()
}
