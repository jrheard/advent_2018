#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate advent_2018;
use advent_2018::fifteen;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("15b", |b| b.iter(|| fifteen::fifteen_b("src/inputs/15.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
