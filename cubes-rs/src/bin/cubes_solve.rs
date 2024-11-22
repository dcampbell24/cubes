use clap::{Parser, ValueEnum};

use std::time::Instant;
use std::{fs, io};

use cubes::{project_dir_cubes, solve, write_obj_file, write_obj_file_solution};
use cubes::{Pieces, Puzzle};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    env_logger::init();
    let cli = Cli::parse();

    if cli.list {
        list_puzzles()?;
        return Ok(());
    }

    let (puzzle, puzzle_string) = choose_puzzle(cli)?;
    write_obj_file(&puzzle, &puzzle_string)?;

    let solutions = solve(&puzzle);
    for solution in &solutions {
        println!("{solution:}");
    }
    write_obj_file_solution(&solutions[0], &puzzle_string)?;

    let elapsed_time = now.elapsed();
    log::info!(
        "Running the program took {} seconds.",
        elapsed_time.as_secs_f64(),
    );

    Ok(())
}

fn choose_puzzle(cli: Cli) -> anyhow::Result<(Pieces, String)> {
    if let Some(pieces) = cli.pieces {
        get_puzzle(&pieces).map(|data| (data, pieces))
    } else {
        let name = match cli.puzzle {
            PuzzleOption::Blue => "blue".to_owned(),
            PuzzleOption::Green => todo!(),
            PuzzleOption::Minotaur => "minotaur".into(),
            PuzzleOption::Orange => "orange".into(),
            PuzzleOption::Red => "red".into(),
            PuzzleOption::White => "white".into(),
            PuzzleOption::Yellow => "yellow".into(),
            PuzzleOption::Towo => "towo".into(),
        };

        get_puzzle(&name).map(|data| (data, name))
    }
}

fn get_puzzle(puzzle: &str) -> anyhow::Result<Pieces> {
    let proj_dirs = project_dir_cubes()?;
    let dir = proj_dirs.data_dir();
    let mut path = dir.join("puzzles").join(puzzle);
    path.set_extension("ron");

    let decoded: Puzzle = ron::from_str(&fs::read_to_string(path)?)?;
    Ok(decoded.data)
}

fn list_puzzles() -> anyhow::Result<()> {
    let proj_dirs = project_dir_cubes()?;
    let dir = proj_dirs.data_dir();
    let path = dir.join("puzzles");
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    for entry in entries {
        println!("{:?}", entry);
    }
    Ok(())
}

/// Program to display the 3x3 cube solution(s).
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List all of the puzzles.
    #[arg(short, long)]
    list: bool,

    /// What puzzle to solve as any string.
    #[arg(short, long)]
    pieces: Option<String>,

    /// What puzzle to solve.
    #[arg(value_enum, default_value_t = PuzzleOption::Minotaur)]
    puzzle: PuzzleOption,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PuzzleOption {
    /// blue
    Blue,
    /// green
    Green,
    /// minotaur
    Minotaur,
    /// orange
    Orange,
    /// red
    Red,
    /// white
    White,
    /// yellow
    Yellow,
    /// Towo
    Towo,
}
