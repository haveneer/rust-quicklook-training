// `transmute` only compiles when the source and destination have the SAME size.
// Here `u32` (4 bytes) and `u64` (8 bytes) differ, so the compiler rejects it
// (E0512) — this is the one safety check `transmute` performs for you.
fn main() {
    let x: u32 = 1;
    let _y: u64 = unsafe { std::mem::transmute(x) }; //~ ERROR cannot transmute between types of different sizes
}
