use sha2::{Sha256, Digest};
use std::fmt::Write as FmtWrite;
// use std::io::Write as IoWrite; // not required
use indicatif::ProgressBar;

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

    let mut results = vec![0; 32];

    let max_seed_power = 20;
    let seed_bar_step = 1 << 8;
    let bar = ProgressBar::new((1 << max_seed_power) / seed_bar_step);
    for seed in 0..1 << max_seed_power { // TODO use functional style
        let mut hasher = Sha256::new();
        let msg = std::format!("Name: me; seed: {:10}", seed);
        hasher.update(msg.as_bytes());
        let result = hasher.finalize();
        results[count_zeros(result.as_slice())] += 1;
        if seed % seed_bar_step == 0 {
            bar.inc(1);
        }
    }
    bar.finish_and_clear();

    results.iter().enumerate().for_each(|(nbz, val)| { println!("{:2} : {:5}", nbz, val) });

    // Score: 2^30 hashs en 422.66s
    //  1 zero : 4176895
    //  2 zeros:   16235
    //  3 zeros:      70
    //  4 zeros:       0
}
