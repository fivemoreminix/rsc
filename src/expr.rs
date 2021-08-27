use crate::OpVal;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'input> {
    Eq(Box<Expr<'input>>, Box<Expr<'input>>),
    Factorial(Box<Expr<'input>>),
    FuncOrVarMul(&'input str, Vec<Expr<'input>>),
    Neg(Box<Expr<'input>>),
    Num(f64), // TODO: replace with num crate type
    Op(OpVal, Box<Expr<'input>>, Box<Expr<'input>>),
    Var(&'input str),
}
