use criterion::{criterion_group, criterion_main, Criterion};

use std::fs;

use cubes::{Error, Pieces, Puzzle, project_dir_cubes};

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

fn minotaur() {
    cubes::solve(get_puzzle("minotaur").unwrap());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("minotaur", |b| b.iter(|| minotaur()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);