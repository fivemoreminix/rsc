use criterion::{criterion_group, criterion_main, Criterion, Benchmark};

use rsc::{
    eval,
    lexer::tokenize,
    parser::parse,
    computer::Computer,
};

const INPUT: &'static str = "sqrt((6.1--2.22)^2 + (-24-10.5)^2)";

fn criterion_benchmark(c: &mut Criterion) {
    let tokens = tokenize(INPUT, true).unwrap();
    let ast = parse(&tokens).unwrap();
    let mut computer = Computer::new(std::f64::consts::PI, std::f64::consts::E);

    // Tokenization
    c.bench_function("lex", |b| b.iter(|| tokenize::<f64>(INPUT, true)));

    // Parsing
    c.bench_function("parse", move |b| b.iter(|| parse::<f64>(&tokens)));

    // Computing
    c.bench_function("compute", move |b| b.iter(|| computer.compute(&ast)));

    // All at once
    c.bench(
        "eval",
        Benchmark::new("indirectly", |b| b.iter(|| eval(INPUT, std::f64::consts::PI, std::f64::consts::E)))
        .with_function("directly", |b| b.iter(|| Computer::new(std::f64::consts::PI, std::f64::consts::E).eval(INPUT)))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
