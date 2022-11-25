use criterion::{criterion_group, criterion_main, Criterion};

fn minotaur() {
    cubes::solve(cubes::get_puzzle("minotaur").unwrap());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("minotaur", |b| b.iter(|| minotaur()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
