//! For taking the product of the parser and calculating it into a 
//! a final form. In this case, the final form is an f64.

use crate::lexer::*;
use crate::parser::*;

// If you come bearing big changes, you may have to rewrite
// this to suit your needs.

#[derive(Debug, Clone, PartialEq)]
pub enum ComputeError {
    UnrecognizedIdentifier(String),
}

/// Turn an AST / Expr into an f64.
pub fn compute(expr: &Expr) -> Result<f64, ComputeError> {
    match expr {
        Expr::Constant(num) => Ok(*num),
        Expr::Identifier(id) => Err(ComputeError::UnrecognizedIdentifier(id.clone())),
        Expr::Neg(expr) => Ok(-compute(expr)?),
        Expr::BinOp(op, lexpr, rexpr) => {
            let lnum = compute(&lexpr)?;
            let rnum = compute(&rexpr)?;

            match op {
                Operator::Plus => Ok(lnum + rnum),
                Operator::Minus => Ok(lnum - rnum),
                Operator::Star => Ok(lnum * rnum),
                Operator::Slash => Ok(lnum / rnum),
                Operator::Percent => Ok(lnum % rnum),
                _ => unimplemented!(),
            }
        }
        Expr::Function(function, expr) => {
            let num = compute(&expr)?;
            Ok(match function {
                Function::Sqrt => num.sqrt(),
                Function::Sin => num.sin(),
                Function::Cos => num.cos(),
                Function::Tan => num.tan(),
                Function::Log => num.log10(),
                Function::Abs => num.abs(),
            })
        }
        Expr::Pow(lexpr, rexpr) => {
            Ok(compute(&lexpr)?.powf(compute(&rexpr)?))
        }
    }
}
