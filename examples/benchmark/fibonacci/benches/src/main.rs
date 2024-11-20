use criterion::{criterion_group, criterion_main, Criterion, measurement::WallTime};
use fibonacci::{fibonacci};

fn benchmark_fibonacci(c: &mut Criterion<WallTime>) {
    c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci(20)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(WallTime);
    targets = benchmark_fibonacci
);
criterion_main!(benches);
