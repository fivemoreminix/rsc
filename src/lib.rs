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

/// Defines the minimum operations and definitions to parse and evaluate expressions.
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
    /// Returns the additive identity value, 0, for the number.
    fn zero() -> Self;
    /// Returns the multiplicative identity value, 1, for the number.
    fn one() -> Self;
    /// Returns true if the number is a whole integer without a fractional part. E.g. 1 or 3.
    fn is_whole(&self) -> bool;
    /// Returns number to the power of `other`.
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
