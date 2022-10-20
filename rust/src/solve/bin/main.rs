extern crate clap;

use clap::Parser;

use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

type Piece = Vec<[i32; 3]>;
type Puzzle = Vec<Piece>;

const SIN: [i32; 4] = [0, 1, 0, -1];
const COS: [i32; 4] = [1, 0, -1, 0];

/// Program to display the 3x3 cube solution(s).
/// Solves mintaur by default.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Solve the blue problem.
    #[arg(short, long)]
    blue: bool,
}

#[derive(Clone, Debug)]
struct PuzzleDense {
    data: [[[i32; 3]; 3]; 3],
}

impl fmt::Display for PuzzleDense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.data[0])?;
        writeln!(f, "{:?}", self.data[1])?;
        writeln!(f, "{:?}", self.data[2])
    }
}

fn main() {
    let now = Instant::now();
    
    let puzzle;
    let args = Args::parse();
    if args.blue {
        puzzle = blue();
    } else {
        puzzle = minotaur();
    }

    let pieces = push_to_zero(puzzle);
    for piece in &pieces {
        for part in piece {
            println!("{:?}", part);
        }
        println!();
    }

    let zero = vec![PuzzleDense { data: zeros() }];

    let mut solutions = Vec::new();
    for (i, piece) in pieces.iter().enumerate() {
        if i == 0 {
            solutions = all_puts(zero.clone(), (i + 1) as i32, piece);
        } else {
            solutions = all_rotations_and_puts(solutions.clone(), (i + 1) as i32, piece);
        }
    }

    for solution in solutions {
        println!("{:}", solution);
    }

    let elapsed_time = now.elapsed();
    println!("Running the program took {} seconds.", elapsed_time.as_millis() as f64 / 1000.0);
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

fn all_rotations(piece: &Piece) -> Vec<Vec<[i32; 3]>> {
    let mut all_rots = Vec::new();
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
        all_rots.push(rotations);
    }

    let mut all_rotss = all_rots.clone();
    for piece in &all_rots {
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
            all_rotss.push(rotations);
        }
    }

    let mut all_rotsss = all_rotss.clone();
    for piece in &all_rotss {
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
            all_rotsss.push(rotations)
        }
    }

    let mut unique_solutions = HashSet::new();
    let rots = push_to_zero(all_rotsss);
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

fn push_to_zero(puzzle: Puzzle) -> Puzzle {
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

fn blue() -> Puzzle {
    let mut blue = Vec::new();

    blue.push(vec![[2, 0, 2], [2, 1, 2], [2, 2, 2]]);

    let piece_2 = vec![[1, 0, 2], [1, 1, 1], [1, 1, 2], [2, 0, 2]];
    for _ in 0..3 {
        blue.push(piece_2.clone());
    }

    let piece_3 = vec![[1, 0, 2], [1, 1, 2], [2, 0, 1], [2, 0, 2]];
    for _ in 0..3 {
        blue.push(piece_3.clone());
    }

    blue
}

fn minotaur() -> Puzzle {
    let mut minotaur = Vec::new();
    minotaur.push(vec![[1, 2, 1], [2, 2, 1], [2, 1, 1], [2, 1, 2]]);
    minotaur.push(vec![[1, 2, 1], [2, 2, 1], [2, 2, 2], [2, 1, 2]]);
    minotaur.push(vec![[1, 1, 1], [2, 1, 1], [3, 1, 1], [2, 2, 1]]);
    minotaur.push(vec![[1, 1, 2], [1, 2, 2], [1, 2, 1], [1, 3, 1], [2, 2, 1]]);
    minotaur.push(vec![[1, 3, 1], [2, 1, 2], [2, 2, 2], [2, 2, 1], [2, 3, 1]]);
    minotaur.push(vec![[1, 1, 1], [1, 2, 1], [1, 3, 1], [2, 1, 1], [1, 2, 2]]);
    minotaur
}