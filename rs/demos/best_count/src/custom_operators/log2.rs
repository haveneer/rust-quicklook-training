


const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub const MAXDIGITNUMBER: u64 =  (std::mem::size_of::<u64>() * 8) as u64 / 2;

pub fn log2(x: u64) -> u64 {
    num_bits::<u64>() as u64 - x.leading_zeros() as u64
}