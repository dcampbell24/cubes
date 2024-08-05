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

fn blue() {
    let _ = cubes::solve(&get_puzzle("blue").unwrap());
}

fn blue_benchmark(c: &mut Criterion) {
    c.bench_function("blue", |b| b.iter(blue));
}

fn minotaur() {
    let _ = cubes::solve(&get_puzzle("minotaur").unwrap());
}

fn minotaur_benchmark(c: &mut Criterion) {
    c.bench_function("minotaur", |b| b.iter(minotaur));
}

fn orange() {
    let _ = cubes::solve(&get_puzzle("orange").unwrap());
}

fn orange_benchmark(c: &mut Criterion) {
    c.bench_function("orange", |b| b.iter(orange));
}

fn red() {
    let _ = cubes::solve(&get_puzzle("red").unwrap());
}

fn red_benchmark(c: &mut Criterion) {
    c.bench_function("red", |b| b.iter(red));
}

fn white() {
    let _ = cubes::solve(&get_puzzle("white").unwrap());
}

fn white_benchmark(c: &mut Criterion) {
    c.bench_function("white", |b| b.iter(white));
}

fn yellow() {
    let _ = cubes::solve(&get_puzzle("yellow").unwrap());
}

fn yellow_benchmark(c: &mut Criterion) {
    c.bench_function("yellow", |b| b.iter(yellow));
}

fn towo() {
    let _ = cubes::solve(&get_puzzle("towo").unwrap());
}

fn towo_benchmark(c: &mut Criterion) {
    c.bench_function("towo", |b| b.iter(towo));
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = blue_benchmark, minotaur_benchmark, orange_benchmark, red_benchmark, white_benchmark, yellow_benchmark, towo_benchmark
);

criterion_main!(benches);
