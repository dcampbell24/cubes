use clap::{Parser, ValueEnum};

use std::time::Instant;
use std::{fs, io};

use cubes::{Error, Pieces, Puzzle};
use cubes::{project_dir_cubes, solve, write_obj_file, write_obj_file_solution};

fn main() -> Result<(), cubes::Error> {
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
        println!("{:}", solution);
    }
    write_obj_file_solution(&solutions[0], &puzzle_string)?;

    let elapsed_time = now.elapsed();
    log::info!(
        "Running the program took {} seconds.",
        elapsed_time.as_secs_f64(),
    );

    Ok(())
}

fn choose_puzzle(cli: Cli) -> Result<(Pieces, String), Error> {
    if let Some(pieces) = cli.pieces {
        match get_puzzle(&pieces) {
            Ok(data) => Ok((data, pieces)),
            Err(_) => Err(Error::DirectoryError), 
        }
    } else {
        let name = match cli.puzzle {
            PuzzleOption::Blue => "blue".to_owned(),
            PuzzleOption::Green => todo!(),
            PuzzleOption::Minotaur => "minotaur".into(),
            PuzzleOption::Orange => "orange".into(),
            PuzzleOption::Red => "red".into(),
            PuzzleOption::White => "white".into(),
            PuzzleOption::Yellow => "yellow".into(),
        };

        match get_puzzle(&name) {
            Ok(data) => Ok((data, name)),
            Err(_) => Err(Error::DirectoryError),        
        }
    }
}

fn get_puzzle(puzzle: &str) -> Result<Pieces, Error> {
    if let Some(proj_dirs) = project_dir_cubes() {
        let dir = proj_dirs.data_dir();
        let path = dir.join("puzzles");
        let decoded: Puzzle = bincode::deserialize(&fs::read(path.join(&puzzle))?)?;
        Ok(decoded.data)
    } else {
        Err(Error::DirectoryError)
    }
}

fn list_puzzles() -> Result<(), Error>{
    if let Some(proj_dirs) = project_dir_cubes() {
        let dir = proj_dirs.data_dir();
        let path = dir.join("puzzles");
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        
        entries.sort();
        for entry in entries {
            println!("{:?}", entry.file_name().expect("there is a file"));
        }
        Ok(())
    } else {
        Err(Error::DirectoryError)
    }
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
}