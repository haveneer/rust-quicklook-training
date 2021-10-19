use sha2::{Sha256, Digest};
use std::fmt::Write as FmtWrite;
// use std::io::Write as IoWrite; // not required
#[allow(unused_imports)]
use indicatif::{ProgressBar, ProgressStyle, ProgressIterator, ParallelProgressIterator};
use rayon::prelude::*;

fn u8_to_string(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut s = String::with_capacity(2 * data.len());
    for byte in data {
        write!(&mut s, "{:02X}", byte)?;
    }
    return Ok(s);
}

fn count_zeros(data: &[u8]) -> usize {
    return data.iter().rev().take_while(|&&x| { x == 0 }).count();
}

fn main() {
    {
        let mut hasher = Sha256::new();
        let seed = 512;
        let msg = std::format!("Name: me; seed: {:10}", seed);
        hasher.update(msg.as_bytes());
        let result = hasher.finalize();
        println!("Msg: [{}]", msg);
        println!("Hash: {:x?}", result);
        println!("Hash: {:?}", u8_to_string(result.as_slice()).unwrap());
    }

    let max_seed_power = 20;
    let max_val = 1 << max_seed_power;
    let seed_bar_step = 1 << (max_seed_power - 8);
    let pb = ProgressBar::new(max_val);
    pb.set_draw_delta(seed_bar_step);
    pb.set_draw_rate(20);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg}"));

    let results = (0..max_val)
        .into_par_iter()
        .progress_with(pb) // progressbar kills perfs
        .fold(|| vec![0; 32], |mut acc, seed| {
            let mut hasher = Sha256::new();
            let msg = std::format!("Name: me; seed: {:10}", seed);
            hasher.update(msg.as_bytes());
            let result = hasher.finalize();
            acc[count_zeros(result.as_slice())] += 1;
            acc
        })
        .reduce(|| vec![0; 32], |mut acc, partial| {
            partial.into_iter().enumerate().for_each(|(index, val)| acc[index] += val);
            acc
        });

    results
        .iter()
        .enumerate()
        .filter(|(_, val)| **val > 0)
        .for_each(|(nbz, val)| { println!("{:2} : {:8}", nbz, val) });

    // == Without progressbar ==
    // Score: 2^30 hashs en 422.66s (séquentiel) et 80s (parallèle) 
    //  1 zero : 4176895
    //  2 zeros:   16235
    //  3 zeros:      70
    //  4 zeros:       0
}
