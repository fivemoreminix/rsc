#![feature(test)]

extern crate test;

use test::{Bencher, black_box};
use rsc::tokenize;

#[bench]
fn bench_tokenizer_short_exprs(b: &mut Bencher) {
    b.iter(|| {
        tokenize(black_box("5.324 * 54(pad)"))
    });
}

#[bench]
fn bench_tokenizer_long_exprs(b: &mut Bencher) {
    b.iter(|| {
        tokenize(black_box("0.999998543 / sqrt(54 ^ (x(3)) % applesauce + bees"))
    });
}
