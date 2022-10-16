type Puzzle = Vec<Vec<[i32; 3]>>;
type PuzzleDense = [[[i32; 3]; 3]; 3];

fn main() {

    let puzzle = blue();
    for piece in &puzzle {
        println!("{:?}", piece);
    }
    println!("");

    let pieces = push_to_zero(puzzle);
    for piece in pieces {
        for part in piece {
            println!("{:?}", part);
        }
        println!("");
    }
}

fn zeros() -> [[[i32; 3]; 3]; 3] {
    let zeros = [
        [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ],
        [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ],
        [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ],
    ];
    zeros
}

fn push_to_zero(puzzle: Puzzle) -> Vec<PuzzleDense> {
    let mut pieces = Vec::new();
    for _ in 0..puzzle.len() {
        pieces.push(zeros());
    }

    for i in 0..puzzle.len() {
        let mut min_x = 99;
        let mut min_y = 99;
        let mut min_z = 99;
        for [x, y, z] in &puzzle[i] {
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
        for [x, y, z] in &puzzle[i] {
            pieces[i][(x - min_x) as usize][(y - min_y) as usize][(z - min_z) as usize] = (i + 1) as i32;
        }

    }

    pieces
}


fn blue() -> Puzzle {
    let mut blue = Vec::new();
    
    blue.push(vec!(
        [2, 0, 2],
        [2, 1, 2],
        [2, 2, 2],
    ));

    let piece_2 = vec!(
        [1, 0, 2],
        [1, 1, 1],
        [1, 1, 2],
        [2, 0, 2],
    );
    for _ in 0..3 {
        blue.push(piece_2.clone());
    }

    let piece_3 = vec!(
        [1, 0, 2],
        [1, 1, 2],
        [2, 0, 1],
        [2, 0, 2],
    );
    for _ in 0..3 {
        blue.push(piece_3.clone());
    }

    blue
}
