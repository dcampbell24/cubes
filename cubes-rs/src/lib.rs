use std::fmt::Write;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write as _;

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use std::collections::{hash_map, HashMap, HashSet};
use std::{fmt, fs};

const SIN: [i32; 4] = [0, 1, 0, -1];
const COS: [i32; 4] = [1, 0, -1, 0];

#[derive(Debug)]
pub enum Error {
    BincodeError(bincode::Error),
    FmtError(std::fmt::Error),
    IoError(std::io::Error),
}

impl From<bincode::Error> for Error {
    fn from(other: bincode::Error) -> Error {
        Error::BincodeError(other)
    }
}
impl From<std::fmt::Error> for Error {
    fn from(other: std::fmt::Error) -> Error {
        Error::FmtError(other)
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Error {
        Error::IoError(other)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PuzzleDense {
    data: [[[i32; 3]; 3]; 3],
}

impl fmt::Display for PuzzleDense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.data[0])?;
        writeln!(f, "{:?}", self.data[1])?;
        writeln!(f, "{:?}", self.data[2])
    }
}

pub type Piece = Vec<[i32; 3]>;
pub type Pieces = Vec<Piece>;
#[derive(Debug, Deserialize, Serialize)]
pub struct Puzzle {
    pub data: Pieces,
}

pub fn solve(puzzle: Pieces) -> Vec<PuzzleDense> {
    let pieces = push_to_zero(puzzle);
    let zero = vec![PuzzleDense { data: zeros() }];

    let mut solutions = Vec::new();
    for (i, piece) in pieces.iter().enumerate() {
        if i == 0 {
            solutions = all_puts(zero.clone(), (i + 1) as i32, piece);
        } else {
            solutions = all_rotations_and_puts(solutions.clone(), (i + 1) as i32, piece);
        }
    }

    unique_pieces(solutions)
}

fn unique_pieces(puzzles: Vec<PuzzleDense>) -> Vec<PuzzleDense> {
    let mut puzzles_unique = HashSet::new();
    for mut puzzle in puzzles {
        let mut puzzle_unique: HashMap<i32, i32> = HashMap::new();
        let mut piece_count = 0;
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    let value = puzzle.data[x][y][z];
                    if let hash_map::Entry::Vacant(e) = puzzle_unique.entry(value) {
                        piece_count += 1;
                        e.insert(piece_count);
                        puzzle.data[x][y][z] = puzzle_unique[&value]
                    } else {
                        puzzle.data[x][y][z] = puzzle_unique[&value]
                    }
                }
            }
        }
        puzzles_unique.insert(puzzle);
    }

    let mut all_puzzles = Vec::new();
    for puzzle in puzzles_unique {
        all_puzzles.push(puzzle);
    }
    all_puzzles
}

fn all_rotations_and_puts(
    already_placed: Vec<PuzzleDense>,
    piece_count: i32,
    piece: &Piece,
) -> Vec<PuzzleDense> {
    let mut solutions = Vec::new();
    for rotation in all_rotations(piece) {
        for s in all_puts(already_placed.clone(), piece_count, &rotation) {
            solutions.push(s);
        }
    }
    solutions
}

fn rotate_z(all_rotations: &mut Pieces, piece: &Piece) {
    for theta in 0..4 {
        let mut rotations = Vec::new();
        for [x, y, z] in piece {
            let rotated_z = [
                x * COS[theta] - y * SIN[theta],
                x * SIN[theta] + y * COS[theta],
                *z,
            ];
            rotations.push(rotated_z);
        }
        all_rotations.push(rotations);
    }
}

fn rotate_y(all_rotations: &mut Pieces, piece: &Piece) {
    for theta in 0..4 {
        let mut rotations = Vec::new();
        for [x, y, z] in piece {
            let rotated_yz = [
                x * COS[theta] + z * SIN[theta],
                *y,
                -x * SIN[theta] + z * COS[theta],
            ];
            rotations.push(rotated_yz);
        }
        all_rotations.push(rotations);
    }
}

fn rotate_x(all_rotations: &mut Pieces, piece: &Piece) {
    for theta in 0..4 {
        let mut rotations = Vec::new();
        for [x, y, z] in piece {
            let rotated_xyz = [
                *x,
                y * COS[theta] - z * SIN[theta],
                y * SIN[theta] + z * COS[theta],
            ];
            rotations.push(rotated_xyz);
        }
        all_rotations.push(rotations)
    }
}

fn all_rotations(piece: &Piece) -> Pieces {
    let all_rots = &mut Vec::new();
    rotate_z(all_rots, piece);

    for piece in all_rots.clone() {
        rotate_y(all_rots, &piece)
    }

    for piece in all_rots.clone() {
        rotate_x(all_rots, &piece)
    }

    let mut unique_solutions = HashSet::new();
    let rots = push_to_zero(all_rots.to_vec());
    for mut solution in rots {
        solution.sort();
        unique_solutions.insert(solution);
    }

    let mut rotations = Vec::new();
    for solution in unique_solutions {
        rotations.push(solution);
    }
    rotations
}

fn max_xyz(piece: &Piece) -> (i32, i32, i32) {
    let mut max_x = -1;
    let mut max_y = -1;
    let mut max_z = -1;

    for [x, y, z] in piece {
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
        if z > &max_z {
            max_z = *z;
        }
    }
    (max_x, max_y, max_z)
}

fn all_puts(already_placed: Vec<PuzzleDense>, piece_count: i32, piece: &Piece) -> Vec<PuzzleDense> {
    let mut all_solutions = Vec::new();
    let (max_x, max_y, max_z) = max_xyz(piece);

    for x_step in 0..3 - max_x {
        for y_step in 0..3 - max_y {
            for z_step in 0..3 - max_z {
                let already_placed = already_placed.clone();
                'next_piece: for mut puzzle in already_placed {
                    for [x, y, z] in piece {
                        if puzzle.data[(x + x_step) as usize][(y + y_step) as usize]
                            [(z + z_step) as usize]
                            > 0
                        {
                            continue 'next_piece;
                        }
                    }

                    for [x, y, z] in piece {
                        puzzle.data[(x + x_step) as usize][(y + y_step) as usize]
                            [(z + z_step) as usize] = piece_count;
                    }

                    all_solutions.push(puzzle);
                }
            }
        }
    }

    all_solutions
}

fn zeros() -> [[[i32; 3]; 3]; 3] {
    [
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
    ]
}

fn push_to_zero(puzzle: Pieces) -> Pieces {
    let mut pieces = Vec::new();

    for part in &puzzle {
        let mut piece = Vec::new();
        let mut min_x = 99;
        let mut min_y = 99;
        let mut min_z = 99;

        for [x, y, z] in part {
            if x < &min_x {
                min_x = *x;
            }
            if y < &min_y {
                min_y = *y;
            }
            if z < &min_z {
                min_z = *z;
            }
        }
        for [x, y, z] in part {
            piece.push([x - min_x, y - min_y, z - min_z]);
        }

        pieces.push(piece);
    }

    pieces
}

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

pub fn choose_puzzle() -> Result<(Pieces, String), Error> {
    let cli = Cli::parse();

    let name = match cli.puzzle {
        PuzzleOption::Blue => "blue".to_string(),
        PuzzleOption::Green => todo!(),
        PuzzleOption::Minotaur => "minotaur".to_string(),
        PuzzleOption::Orange => "orange".to_string(),
        PuzzleOption::Red => "red".to_string(),
        PuzzleOption::White => "white".to_string(),
        PuzzleOption::Yellow => "yellow".to_string(),
    };

    let decoded: Puzzle =
        bincode::deserialize(&fs::read(format!("puzzles/{}", &name))?)?;
    Ok((decoded.data, name))
}

pub fn get_puzzle(puzzle: &str) -> Result<Pieces, Error> {
    let decoded: Puzzle =
        bincode::deserialize(&fs::read(format!("puzzles/{}", puzzle))?)?;
    Ok(decoded.data)
}

pub fn write_obj_file_solution(puzzle: &PuzzleDense, puzzle_string: &str) -> Result<(), Error> {
    let mut buffer = File::create(format!("target/{}/solution.mtl", puzzle_string))?;
    let mut string = String::new();

    writeln!(string, "# Rust generated MTL file")?;

    writeln!(string, "newmtl 1")?;
    writeln!(string, "Kd 1.0 0.0 0.0")?;

    writeln!(string, "newmtl 2")?;
    writeln!(string, "Kd 0.0 1.0 0.0")?;

    writeln!(string, "newmtl 3")?;
    writeln!(string, "Kd 0.0 0.0 1.0")?;

    writeln!(string, "newmtl 4")?;
    writeln!(string, "Kd 1.0 1.0 0.0")?;

    writeln!(string, "newmtl 5")?;
    writeln!(string, "Kd 0.0 1.0 1.0")?;

    writeln!(string, "newmtl 6")?;
    writeln!(string, "Kd 1.0 0.5 0.0")?;

    writeln!(string, "newmtl 7")?;
    writeln!(string, "Kd 0.6 0.6 0.6")?;

    writeln!(string, "newmtl 8")?;
    writeln!(string, "Kd 0.3 0.3 0.3")?;

    buffer.write_all(&string.into_bytes())?;

    let mut buffer = File::create(format!("target/{}/solution.obj", puzzle_string))?;
    let mut string = String::new();

    writeln!(string, "# Rust generated OBJ file.")?;
    writeln!(string, "mtllib solution.mtl")?;

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let color = puzzle.data[x as usize][y as usize][z as usize];
                writeln!(string, "usemtl {}", color)?;
                write_box_points(&mut string, &x, &y, &z)?;
                write_box_faces(&mut string, (x * 9 + y * 3 + z) as usize)?;
            }
        }
    }

    buffer.write_all(&string.into_bytes())?;
    Ok(())
}

pub fn write_obj_file(puzzle: &Pieces, puzzle_string: &str) -> Result<(), Error> {
    match fs::create_dir(format!("target/{}", puzzle_string)) {
        Err(e) if e.kind() == AlreadyExists => {}
        Err(e) => return Err(Error::from(e)),
        Ok(_) => {}
    }

    write_mtl_file(puzzle_string)?;

    for (i, piece) in puzzle.iter().enumerate() {
        let mut buffer = File::create(format!("target/{}/piece_{}.obj", puzzle_string, i))?;
        let mut string = String::new();

        writeln!(string, "# Rust generated OBJ file.")?;
        writeln!(string, "mtllib piece.mtl")?;
        writeln!(string, "usemtl {}", 0)?;

        for [x, y, z] in piece {
            write_box_points(&mut string, x, y, z)?;
        }

        for (i, _) in piece.iter().enumerate() {
            write_box_faces(&mut string, i)?;
        }

        buffer.write_all(&string.into_bytes())?;
    }
    Ok(())
}

fn write_box_points(s: &mut String, x: &i32, y: &i32, z: &i32) -> Result<(), std::fmt::Error> {
    writeln!(s, "v {} {} {}", x - 1, y - 1, z - 1)?;
    writeln!(s, "v {} {} {}", x - 1, y, z - 1)?;
    writeln!(s, "v {} {} {}", x, y - 1, z - 1)?;
    writeln!(s, "v {} {} {}", x, y, z - 1)?;
    writeln!(s, "v {} {} {}", x - 1, y - 1, z)?;
    writeln!(s, "v {} {} {}", x - 1, y, z)?;
    writeln!(s, "v {} {} {}", x, y - 1, z)?;
    writeln!(s, "v {} {} {}", x, y, z)
}

#[rustfmt::skip]
fn write_box_faces(s: &mut String, i: usize) -> Result<(), std::fmt::Error> {
    writeln!(s, "f {} {} {} {}", i * 8 + 1, i * 8 + 2, i * 8 + 4, i * 8 + 3)?;
    writeln!(s, "f {} {} {} {}", i * 8 + 5, i * 8 + 6, i * 8 + 8, i * 8 + 7)?;
    writeln!(s, "f {} {} {} {}", i * 8 + 1, i * 8 + 2, i * 8 + 6, i * 8 + 5)?;
    writeln!(s, "f {} {} {} {}", i * 8 + 4, i * 8 + 3, i * 8 + 7, i * 8 + 8)?;
    writeln!(s, "f {} {} {} {}", i * 8 + 1, i * 8 + 5, i * 8 + 7, i * 8 + 3)?;
    writeln!(s, "f {} {} {} {}", i * 8 + 2, i * 8 + 6, i * 8 + 8, i * 8 + 4)
}

fn write_mtl_file(path: &str) -> Result<(), Error> {
    let mut buffer = File::create(format!("target/{}/piece.mtl", path))?;
    let mut string = String::new();

    writeln!(string, "# Rust generated MTL file")?;
    writeln!(string, "newmtl 0")?;

    match path {
        "blue" => writeln!(string, "Kd 0.0 0.0 1.0")?,
        "green" => todo!(),
        "minotaur" => writeln!(string, "Kd 0.3 0.3 0.3")?,
        "orange" => writeln!(string, "Kd 1.0 0.3 0.0")?,
        "red" => writeln!(string, "Kd 1.0 0.0 0.0")?,
        "white" => writeln!(string, "Kd 0.6 0.6 0.6")?,
        "yellow" => writeln!(string, "Kd 1.0 1.0 0.0")?,
        _ => unreachable!(),
    }

    buffer.write_all(&string.into_bytes())?;
    Ok(())
}
