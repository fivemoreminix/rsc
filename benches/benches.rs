#![feature(test)]

extern crate test;

use rsc::{parse, tokenize, Interpreter, Variant};
use test::{black_box, Bencher};

const SHORT_STR: &str = "5.324 * 54(pad)";
const LONG_STR: &str = "0.999998543 / sqrt(54 ^ (x(3))) % applesauce + bees";
const FUNCTIONS_VARS: &str = "abs(5) + x(3) + abs(x(2)) + sqrt(4)";

macro_rules! tokenizer_bench {
    ($name:ident, $input:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| tokenize::<f64>(black_box($input)));
        }
    };
}

macro_rules! parser_bench {
    ($name:ident, $input:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let tokens = tokenize::<f64>($input).unwrap();
            b.iter(|| parse(black_box(&tokens)))
        }
    };
}

macro_rules! eval_bench {
    ($name:ident, $input:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let tokens = tokenize($input).unwrap();
            let expr = parse(&tokens).unwrap();
            let mut i = Interpreter::default();
            i.set_var(String::from("pad"), Variant::Num(5.0));
            i.set_var(String::from("x"), Variant::Num(2.0));
            i.set_var(String::from("applesauce"), Variant::Num(1.0));
            i.set_var(String::from("bees"), Variant::Num(1.0));
            b.iter(|| {
                i.eval(black_box(&expr)).unwrap();
            })
        }
    };
}

tokenizer_bench!(tokenizer_short_expr, SHORT_STR);
tokenizer_bench!(tokenizer_long_expr, LONG_STR);
tokenizer_bench!(tokenizer_function_vars, FUNCTIONS_VARS);

parser_bench!(parser_short_expr, SHORT_STR);
parser_bench!(parser_long_expr, LONG_STR);
parser_bench!(parser_function_vars, FUNCTIONS_VARS);

eval_bench!(eval_short_expr, SHORT_STR);
eval_bench!(eval_long_expr, LONG_STR);
eval_bench!(eval_function_vars, FUNCTIONS_VARS);
