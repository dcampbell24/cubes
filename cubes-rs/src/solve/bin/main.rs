use std::time::Instant;

use cubes_rs::{choose_puzzle, solve, write_obj_file};

fn main() {
    let now = Instant::now();

    let (puzzle, puzzle_string) = choose_puzzle();
    
    write_obj_file(&puzzle, &puzzle_string).unwrap();
    
    let solutions = solve(puzzle);
    for solution in solutions {
        println!("{:}", solution);
    }

    let elapsed_time = now.elapsed();
    println!("Running the program took {} seconds.", elapsed_time.as_micros() as f64 / 1_000_000.0);
}