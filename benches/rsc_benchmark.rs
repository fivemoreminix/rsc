use criterion::{criterion_group, criterion_main, Criterion};

use rsc::eval;

const INPUT: &'static str = "sqrt((6.1--2.22)^2 + (-24-10.5)^2)";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("eval", |b| b.iter(|| eval(INPUT, std::f64::consts::PI, std::f64::consts::E)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
