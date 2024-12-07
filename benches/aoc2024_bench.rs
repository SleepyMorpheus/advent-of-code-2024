use aoc2024::days::run;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    for day in 7..8 {
        c.bench_function(format!("Day {}a", day).as_str(), |b| {
            b.iter(|| (black_box(run(day, true, false))))
        });
        c.bench_function(format!("Day {}b", day).as_str(), |b| {
            b.iter(|| (black_box(run(day, false, false))))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
