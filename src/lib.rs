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
use std::ops::{Add, Mul, Sub, Div, Neg, Rem, AddAssign, MulAssign, SubAssign, DivAssign};

pub trait Num: Debug + Clone + PartialEq + PartialOrd + FromStr + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + Neg<Output=Self> + AddAssign + SubAssign + MulAssign + DivAssign {
    #[inline(always)] fn zero() -> Self;
    #[inline(always)] fn one() -> Self;
    #[inline] fn is_whole(&self) -> bool;
    #[inline] fn pow(self, other: Self) -> Self;
    fn factorial(self) -> Self {
        let one = Num::one();
        if self <= one {
            one
        } else {
            let mut result = one;
            let mut i = result.clone() + Num::one();
            while i <= self {
                result *= i.clone();
                i += Num::one();
            }
            result
        }
    }
}

// Default impls for Num
impl Num for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn is_whole(&self) -> bool {
        self.fract() == 0.0
    }
    #[inline(always)]
    fn pow(self, other: Self) -> Self {
        self.powf(other)
    }
}

impl Num for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn is_whole(&self) -> bool {
        self.fract() == 0.0
    }
    #[inline(always)]
    fn pow(self, other: Self) -> Self {
        self.powf(other)
    }
}
