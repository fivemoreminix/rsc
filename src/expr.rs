use crate::{Num, OpVal};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'input, N: Num> {
    Eq(Box<Expr<'input, N>>, Box<Expr<'input, N>>),
    FuncOrVarMul(&'input str, Vec<Expr<'input, N>>),
    Neg(Box<Expr<'input, N>>),
    Num(&'input N),
    Op(OpVal, Box<Expr<'input, N>>, Box<Expr<'input, N>>),
    Var(&'input str),
}
