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

macro_rules! impl_num_for_integer {
    ($itype:ty) => {
        impl Num for $itype {
            #[inline(always)]
            fn zero() -> Self {
                0
            }
            #[inline(always)]
            fn one() -> Self {
                1
            }
            #[inline(always)]
            fn is_whole(&self) -> bool {
                true
            }
            #[inline(always)]
            fn pow(self, other: Self) -> Self {
                self.wrapping_pow(other as u32) // Wraps on overflow...
            }
        }
    };
}
impl_num_for_integer!(i8);
impl_num_for_integer!(i16);
impl_num_for_integer!(i32);
impl_num_for_integer!(i64);
impl_num_for_integer!(i128);
impl_num_for_integer!(isize);

macro_rules! impl_num_for_float {
    ($ftype:ty) => {
        impl Num for $ftype {
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
                self.powf(other) // inf or -inf if overflowed...
            }
        }
    };
}
impl_num_for_float!(f32);
impl_num_for_float!(f64);
