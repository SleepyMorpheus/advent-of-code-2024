use criterion::{criterion_group, criterion_main, Criterion};
use aoc2024lib::days::run;


fn criterion_benchmark(c: &mut Criterion) {
    for i in 1..2 {
        c.bench_function(&format!("run({}, first)", i), |b| b.iter(|| criterion::black_box(run(i, true, false))));
        c.bench_function(&format!("run({}, second)", i), |b| b.iter(|| criterion::black_box(run(i, false, false))));
    }

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);