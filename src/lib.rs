mod expr;
mod parser;
mod tokenizer;
mod interpreter;

pub use expr::*;
pub use parser::*;
pub use tokenizer::*;
pub use interpreter::*;

use std::str::FromStr;
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub, Div, Neg, Rem};

pub trait Num: Debug + Clone + PartialEq + FromStr + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + Neg<Output=Self> {
    fn pow(self, other: Self) -> Self;
}

// Default impls for Num
impl Num for f32 {
    fn pow(self, other: Self) -> Self {
        self.powf(other)
    }
}
impl Num for f64 {
    fn pow(self, other: Self) -> Self {
        self.powf(other)
    }
}
