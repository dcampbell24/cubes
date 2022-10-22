pub type Piece = Vec<[i32; 3]>;
pub type Puzzle = Vec<Piece>;

pub fn blue() -> Puzzle {
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
    minotaur.push(vec![[1, 2, 1], [2, 2, 1], [2, 1, 1], [2, 1, 2]]);
    minotaur.push(vec![[1, 2, 1], [2, 2, 1], [2, 2, 2], [2, 1, 2]]);
    minotaur.push(vec![[1, 1, 1], [2, 1, 1], [3, 1, 1], [2, 2, 1]]);
    minotaur.push(vec![[1, 1, 2], [1, 2, 2], [1, 2, 1], [1, 3, 1], [2, 2, 1]]);
    minotaur.push(vec![[1, 3, 1], [2, 1, 2], [2, 2, 2], [2, 2, 1], [2, 3, 1]]);
    minotaur.push(vec![[1, 1, 1], [1, 2, 1], [1, 3, 1], [2, 1, 1], [1, 2, 2]]);
    minotaur
}