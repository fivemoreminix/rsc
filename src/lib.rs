//! This crate is specifically used for one thing: turning expressions inside of a string
//! into a value. This crate acts as a scientific calculator, and includes several functions.
//! 
//! If you need a portion of the calculator changed or removed, please fork it, and make your
//! changes. We encourage others to change RSC to their liking. You do not need to attribute
//! anything to us. This is MIT licensed software.

#![feature(test)]

extern crate test;

pub mod lexer;
pub mod parser;
pub mod computer;

#[derive(Debug, Clone)]
pub enum EvalError {
    ComputeError(computer::ComputeError),
    ParserError(parser::ParserError),
    LexerError(lexer::LexerError),
}

/// Turn an expression inside a string into a number.
/// If you are looking for more control, you may want to use
/// the `lexer`, `parser`, and `computer` modules individually.
/// ```
/// assert_eq!(eval("3.1 + 2.2"), Ok(5.3));
/// ```
pub fn eval(input: &str) -> Result<f64, EvalError> {
    match lexer::tokenize(input) {
        Ok(tokens) => match parser::parse(&tokens) {
            Ok(ast) => match computer::Computer::new().compute(&ast) {
                Ok(num) => Ok(num),
                Err(compute_err) => Err(EvalError::ComputeError(compute_err)),
            }
            Err(parser_err) => Err(EvalError::ParserError(parser_err)),
        }
        Err(lexer_err) => Err(EvalError::LexerError(lexer_err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

    static INPUT: &'static str = "sqrt((6.1--2.22)^2 + (-24-10.5)^2)";

    #[bench]
    fn bench_eval(b: &mut Bencher) {
        b.iter(|| eval(INPUT).unwrap());
    }

    #[bench]
    fn bench_tokenize(b: &mut Bencher) {
        b.iter(|| lexer::tokenize(INPUT).unwrap());
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let tokens = lexer::tokenize(INPUT).unwrap();
        b.iter(|| parser::parse(&tokens).unwrap());
    }

    #[bench]
    fn bench_compute(b: &mut Bencher) {
        let ast = parser::parse(&lexer::tokenize(INPUT).unwrap()).unwrap();
        let mut computer = computer::Computer::new();
        b.iter(|| computer.compute(&ast));
    }
}
