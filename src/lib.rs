//! This crate is specifically used for one thing: turning expressions inside of a string
//! into a value. This crate acts as a scientific calculator, and includes several functions.
//!
//! If you need a portion of the calculator changed or removed, please fork it, and make your
//! changes. We encourage others to change RSC to their liking. You do not need to attribute
//! anything to us. This is MIT licensed software.
//!
//! Anyone can easily create a [Computer](computer/struct.Computer.html) and begin working with expressions. Computers
//! also remember variables using a HashMap. You can create and begin using the preconfigured Computer like so:
//! ```
//! use rsc::computer::Computer;
//!
//! fn main() {
//!     let mut c = Computer::<f64>::default();
//!     // or
//!     // let mut c: Computer<f64> = Default::default();
//!
//!     assert!(c.eval("x = 5").unwrap() == 5.0);
//!     assert!(c.eval("x^2").unwrap() == 25.0);
//! }
//! ```
//!
//! # 
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
//!     let mut computer = Computer::<f64>::default();
//!     
//!     for x in 2..=5 {
//!         let mut ast = ast.clone();
//! 
//!         // Replace instances of variable reference 'x' with f64 value x from loop
//!         ast.replace(&Expr::Identifier("x".to_owned()), &Expr::Constant(x as f64), false);
//! 
//!         // or this could be done like:
//!         // computer.variables.
//! 
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
//! 
//! # Creating an *Empty* Computer
//! Constructing a Computer with `Computer::<f64>::default()` will initialize it with default
//! variables: PI and E, and default functions: `sin`, `cos`, `tan`, `log`, and `sqrt`.
//! To get a Computer with absolutely no default values, please construct one like so:
//! ```
//! let mut c = Computer::new();
//! /// ... using the computer
//! ```
pub mod computer;
pub mod lexer;
pub mod parser;

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
        self.fract() <= 0.0
    }
    fn abs(&self) -> Self {
        f64::abs(*self)
    }
    fn pow(&self, other: &Self) -> Self {
        self.powf(*other)
    }
}

/// An error that groups together all three error types: [computer::ComputeError](computer/enum.ComputeError.html),
/// [parser::ParserError](parser/enum.ParserError.html), and [lexer::LexerError](lexer/enum.LexerError.html). Produced when using `eval` helper functions.
#[derive(Debug, Clone)]
pub enum EvalError<'a, T: Clone + std::fmt::Debug> {
    ComputeError(computer::ComputeError<'a>),
    ParserError(parser::ParserError<'a, T>),
    LexerError(lexer::LexerError<'a>),
}

/// Turn an expression inside a string into a number.
/// If you are looking for more control, you may want to use
/// the [lexer](lexer/index.html), [parser](parser/index.html), and [computer](computer/index.html) modules individually.
/// 
/// This creates the computer using `Computer::new()`. If you are looking
/// to do simple `eval` on a preconfigured Computer, please call [`eval`
/// on the computer](computer/struct.Computer.html#method.eval). For example:
/// ```
/// let mut computer = Computer::<f64>::default();
/// assert!(computer.eval("3.1 + 2.2").unwrap() == 5.3);
/// ```
/// 
/// # Example
/// ```
/// use rsc::eval;
/// assert!(eval("3.1 + 2.2").unwrap() == 5.3);
/// ```
pub fn eval<'a, T: std::fmt::Debug>(input: &'a str) -> Result<T, EvalError<T>> where T: Num + std::str::FromStr + Clone + PartialOrd + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>{
    match lexer::tokenize(input, false) {
        Ok(tokens) => match parser::parse(&tokens) {
            Ok(ast) => match computer::Computer::new().compute(&ast) {
                Ok(num) => Ok(num),
                Err(compute_err) => Err(EvalError::ComputeError(compute_err)),
            },
            Err(parser_err) => Err(EvalError::ParserError(parser_err)),
        },
        Err(lexer_err) => Err(EvalError::LexerError(lexer_err)),
    }
}
