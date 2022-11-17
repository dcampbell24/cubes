use std::time::Instant;

use cubes_rs::{choose_puzzle, solve, write_obj_file, write_obj_file_solution};

fn main() -> Result<(), cubes_rs::Error> {
    let now = Instant::now();
    env_logger::init();

    let (puzzle, puzzle_string) = choose_puzzle()?;
    write_obj_file(&puzzle, &puzzle_string)?;

    let solutions = solve(puzzle);
    for solution in &solutions {
        println!("{:}", solution);
    }
    write_obj_file_solution(&solutions[0], &puzzle_string)?;

    let elapsed_time = now.elapsed();
    log::info!(
        "Running the program took {} seconds.",
        elapsed_time.as_micros() as f64 / 1_000_000.0
    );

    Ok(())
}
