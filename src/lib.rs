mod expr;
mod interpreter;
mod parser;
mod tokenizer;

pub use expr::*;
pub use interpreter::*;
pub use parser::*;
pub use tokenizer::*;

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use std::str::FromStr;

pub trait Num:
    Debug
    + Clone
    + PartialEq
    + PartialOrd
    + FromStr
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_whole(&self) -> bool;
    fn pow(self, other: Self) -> Self;
}

// Default impls for Num
impl Num for f32 {
    #[inline(always)]
    fn zero() -> Self {
        0.0
    }
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn is_whole(&self) -> bool {
        self.fract() == 0.0
    }
    #[inline(always)]
    fn pow(self, other: Self) -> Self {
        self.powf(other)
    }
}

impl Num for f64 {
    #[inline(always)]
    fn zero() -> Self {
        0.0
    }
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn is_whole(&self) -> bool {
        self.fract() == 0.0
    }
    #[inline(always)]
    fn pow(self, other: Self) -> Self {
        self.powf(other)
    }
}
