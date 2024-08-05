use criterion::{criterion_group, criterion_main, Criterion};

use std::{fs, time::Duration};

use cubes::{project_dir_cubes, Pieces, Puzzle};

fn get_puzzle(puzzle: &str) -> anyhow::Result<Pieces> {
    let proj_dirs = project_dir_cubes()?;
    let dir = proj_dirs.data_dir();
    let mut path = dir.join("puzzles").join(puzzle);
    path.set_extension("ron");
    let decoded: Puzzle = ron::from_str(&fs::read_to_string(path)?)?;
    Ok(decoded.data)
}

fn minotaur() {
    let _ = cubes::solve(&get_puzzle("minotaur").unwrap());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("minotaur", |b| b.iter(minotaur));
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
);

criterion_main!(benches);
