//! This crate is specifically used for one thing: turning expressions inside of a string
//! into a value. This crate acts as a scientific calculator, and includes several functions.
//! 
//! If you need a portion of the calculator changed or removed, please fork it, and make your
//! changes. We encourage others to change RSC to their liking. You do not need to attribute
//! anything to us. This is MIT licensed software.
//! 
//! Anyone can easily create a [Calculator](computer/struct.Computer.html) and begin working with expressions. Calculators
//! also remember variables using a HashMap. You can create and begin using the Calculator like so:
//! ```
//! use rsc::computer::Computer;
//!
//! fn main() {
//!     let mut c = Computer::new(std::f64::consts::PI, std::f64::consts::E);
//!
//!     assert!(c.eval("x = 5").unwrap() == 5.0);
//!     assert!(c.eval("x^2").unwrap() == 25.0);
//! }
//! ```
//! 
//! In most cases a simple `eval` should be all you need, but just as many times you may need
//! to directly access the tokens and AST. Some reasons may include:
//! * For performance or caching; lexing and parsing an expression only once, to calculate it later hundreds
//! of times in a loop.
//! * Better error messages or visual information for what is happening.
//! ```
//! use rsc::{
//!     lexer::tokenize,
//!     parser::{parse, Expr},
//!     computer::Computer,
//! };
//! 
//! fn main() {
//!     let tokens = tokenize("x^2", true).unwrap();
//!     let ast = parse(&tokens).unwrap();
//!     let mut computer = Computer::new(std::f64::consts::PI, std::f64::consts::E);
//!     
//!     for x in 2..=5 {
//!         let mut ast = ast.clone();
//!         ast.replace(&Expr::Identifier("x".to_owned()), &Expr::Constant(x as f64), false);
//!         println!("{}", computer.compute(&ast).unwrap());
//!     }
//! }
//! 
//! // Output:
//! // 4
//! // 9
//! // 16
//! // 25
//! ```
pub mod lexer;
pub mod parser;
pub mod computer;

use crate::computer::Num;
use std::ops::*;

impl Num for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn is_integer(&self) -> bool {
        !(self.fract() > 0.0)
    }
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
    fn sin(&self) -> Self {
        f64::sin(*self)
    }
    fn cos(&self) -> Self {
        f64::cos(*self)
    }
    fn tan(&self) -> Self {
        f64::tan(*self)
    }
    fn log(&self) -> Self {
        self.log10()
    }
    fn abs(&self) -> Self {
        f64::abs(*self)
    }
    fn pow(&self, other: &Self) -> Self {
        self.powf(*other)
    }
}

#[derive(Debug, Clone)]
pub enum EvalError<T> {
    ComputeError(computer::ComputeError),
    ParserError(parser::ParserError<T>),
    LexerError(lexer::LexerError),
}

/// Turn an expression inside a string into a number.
/// If you are looking for more control, you may want to use
/// the `lexer`, `parser`, and `computer` modules individually.
/// ```
/// use rsc::eval;
/// eval("3.1 + 2.2"); // Ok(5.3)
/// ```
pub fn eval<T>(input: &str, pi_val: T, e_val: T) -> Result<T, EvalError<T>> where T: Num + std::str::FromStr + Clone + PartialOrd + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>{
    match lexer::tokenize(input, false) {
        Ok(tokens) => match parser::parse(&tokens) {
            Ok(ast) => match computer::Computer::new(pi_val, e_val).compute(&ast) {
                Ok(num) => Ok(num),
                Err(compute_err) => Err(EvalError::ComputeError(compute_err)),
            }
            Err(parser_err) => Err(EvalError::ParserError(parser_err)),
        }
        Err(lexer_err) => Err(EvalError::LexerError(lexer_err)),
    }
}
