use std::time;

#[macro_use]
extern crate criterion;
use criterion::{Benchmark, Criterion};

extern crate advent_2018;
use advent_2018::fifteen;

fn criterion_benchmark(c: &mut Criterion) {
    let benchmark = Benchmark::new("15b", |b| b.iter(|| fifteen::fifteen_b("src/inputs/15.txt")))
        .sample_size(20)
        .measurement_time(time::Duration::new(180, 0));

    c.bench("15b", benchmark);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
