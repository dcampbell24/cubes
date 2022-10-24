extern crate clap;

use clap::{Parser, ValueEnum};

pub type Piece = Vec<[i32; 3]>;
pub type Puzzle = Vec<Piece>;

/// Program to display the 3x3 cube solution(s).
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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

pub fn choose_puzzle() -> Puzzle {
    let cli = Cli::parse();

    match cli.puzzle {
        PuzzleOption::Blue => blue(),
        PuzzleOption::Green => green(),
        PuzzleOption::Minotaur => minotaur(),
        PuzzleOption::Orange => orange(),
        PuzzleOption::Red => red(),
        PuzzleOption::White => white(),
        PuzzleOption::Yellow => yellow(),
    }
}

pub fn blue() -> Puzzle {
    let mut blue = Vec::new();

    let piece_2 = vec![[1, 0, 2], [1, 1, 1], [1, 1, 2], [2, 0, 2]];
    for _ in 0..3 {
        blue.push(piece_2.clone());
    }

    let piece_3 = vec![[1, 0, 2], [1, 1, 2], [2, 0, 1], [2, 0, 2]];
    for _ in 0..3 {
        blue.push(piece_3.clone());
    }
    
    blue.push(vec![[2, 0, 2], [2, 1, 2], [2, 2, 2]]);
    blue
}

// Is wrong, has a volume of 28.
pub fn green() -> Puzzle {
    let mut green = Vec::new();
    green.push(vec![[2, 1, 3], [2, 2, 3], [2, 3, 3], [3, 1, 3]]);
    for _ in 0..2 {
        green.push(vec![[2, 2, 3], [3, 1, 2], [3, 1, 3], [3, 2, 3]]);  
    }
    green.push(vec![[1, 1, 3], [2, 1, 3], [2, 2, 2], [2, 2, 3], [3, 2, 2]]);
    green.push(vec![[2, 1, 3], [3, 1, 3], [3, 2, 1], [3, 2, 2], [3, 2, 3]]);
    green.push(vec![[2, 1, 1], [2, 1, 2], [2, 1, 3], [2, 2, 3], [2, 3, 3], [3, 1, 1]]);
    green
}

pub fn minotaur() -> Puzzle {
    let mut minotaur = Vec::new();
    minotaur.push(vec![[1, 1, 2], [1, 2, 2], [1, 2, 1], [1, 3, 1], [2, 2, 1]]);
    minotaur.push(vec![[1, 3, 1], [2, 1, 2], [2, 2, 2], [2, 2, 1], [2, 3, 1]]);
    minotaur.push(vec![[1, 1, 1], [1, 2, 1], [1, 3, 1], [2, 1, 1], [1, 2, 2]]);
    minotaur.push(vec![[1, 2, 1], [2, 2, 1], [2, 1, 1], [2, 1, 2]]);
    minotaur.push(vec![[1, 2, 1], [2, 2, 1], [2, 2, 2], [2, 1, 2]]);
    minotaur.push(vec![[1, 1, 1], [2, 1, 1], [3, 1, 1], [2, 2, 1]]);
    minotaur
}

pub fn orange() -> Puzzle {
    let mut orange = Vec::new();
    orange.push(vec![[1, 1, 3], [2, 1, 3], [3, 1, 2], [3, 1, 3], [3, 2, 3]]);
    for _ in 0..2 {
        orange.push(vec![[1, 1, 3], [2, 1, 3], [3, 1, 3], [3, 2, 2], [3, 2, 3]]);
    }
    orange.push(vec![[2, 1, 2], [2, 2, 2], [2, 2, 3], [3, 1, 2]]);
    for _ in 0..2 {
        orange.push(vec![[1, 1, 3], [2, 1, 3], [3, 1, 3], [3, 2, 3]]);
    }
    orange
}

pub fn red() -> Puzzle {
    let mut red = Vec::new();
    red.push(vec![[1, 1, 3], [1, 2, 3], [1, 3, 3], [2, 1, 3], [2, 2, 3], [3, 1, 3]]);
    red.push(vec![[2, 1, 3], [2, 2, 3], [3, 1, 3], [3, 2, 2], [3, 2, 3], [3, 3, 2]]);
    red.push(vec![[2, 3, 3], [3, 1, 3], [3, 2, 3], [3, 3, 1], [3, 3, 2], [3, 3, 3]]);
    red.push(vec![[2, 1, 3], [3, 1, 3], [3, 2, 3], [3, 3, 2], [3, 3, 3]]);
    red.push(vec![[2, 1, 3], [3, 1, 3], [3, 2, 2], [3, 2, 3]]);
    red
}

pub fn white() -> Puzzle {
    let mut white = Vec::new();
    white.push(vec![[1, 1, 2], [2, 1, 2], [2, 2, 2], [2, 2, 3], [3, 2, 3]]);
    white.push(vec![[2, 1, 3], [2, 2, 2], [2, 2, 3], [2, 3, 3], [3, 3, 3]]);
    white.push(vec![[2, 1, 3], [2, 2, 3], [2, 3, 3], [3, 3, 2], [3, 3, 3]]);
    white.push(vec![[1, 1, 3], [1, 2, 3], [2, 1, 3], [3, 1, 3]]);
    for _ in 0..2 {
        white.push(vec![[2, 1, 3], [2, 2, 3], [3, 2, 2], [3, 2, 3]]);
    }
    white
}

pub fn yellow() -> Puzzle {
    let mut yellow = Vec::new();
    yellow.push(vec![[2, 1, 3], [2, 2, 3], [3, 2, 3], [3, 3, 2], [3, 3, 3]]);
    yellow.push(vec![[2, 1, 2], [2, 2, 2], [2, 2, 3], [3, 1, 2]]);
    yellow.push(vec![[2, 1, 3], [2, 2, 2], [2, 2, 3], [3, 1, 3]]);
    for _ in 0..2 {
        yellow.push(vec![[1, 1, 3], [2, 1, 3], [3, 1, 3], [3, 2, 3]])
    } 
    for _ in 0..2 {
        yellow.push(vec![[2, 1, 3], [3, 1, 3], [3, 2, 3]]);   
    }
    yellow
}