// cargo run --example debug_eyre
// RUST_BACKTRACE=1 cargo run --example debug_eyre
use eyre::{OptionExt, Result};

fn parse_cell_count(line: &str) -> Result<usize> {
    let count = line.trim().parse::<usize>()?; // Err created here
    Ok(count)
}

fn load_mesh(header: &str) -> Result<usize> {
    let line = header.lines().nth(1).ok_or_eyre("no cell count")?;
    parse_cell_count(line) // no .wrap_err() needed to find the origin
}

fn main() -> Result<()> {
    let cells = load_mesh("mesh v2\n12O\n")?; // letter O, not zero
    println!("{cells} cells");
    Ok(())
}
