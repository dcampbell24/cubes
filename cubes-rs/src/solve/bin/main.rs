use std::time::Instant;

use cubes_rs::{choose_puzzle, solve};

fn main() {
    let now = Instant::now();

    solve(choose_puzzle());

    let elapsed_time = now.elapsed();
    println!("Running the program took {} seconds.", elapsed_time.as_micros() as f64 / 1_000_000.0);
}