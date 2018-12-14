//! For taking the product of the parser and calculating it into a 
//! a final form. In this case, the final form is an f64.

use crate::lexer::*;
use crate::parser::*;
use crate::EvalError;

use std::collections::HashMap;

// If you come bearing big changes, you may have to rewrite
// this to suit your needs.

#[derive(Debug, Clone, PartialEq)]
pub enum ComputeError {
    UnrecognizedIdentifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Computer {
    variables: HashMap<String, f64>,
}

impl Computer {
    pub fn new() -> Computer {
        Computer { variables: HashMap::new() }
    }

    pub fn eval(&mut self, expr: &str) -> Result<f64, EvalError> {
        match tokenize(expr) {
            Ok(tokens) => match parse(&tokens) {
                Ok(ast) => match self.compute(&ast) {
                    Ok(num) => Ok(num),
                    Err(compute_err) => Err(EvalError::ComputeError(compute_err)),
                }
                Err(parser_err) => Err(EvalError::ParserError(parser_err)),
            }
            Err(lexer_err) => Err(EvalError::LexerError(lexer_err)),
        }
    }

    pub fn compute(&mut self, expr: &Expr) -> Result<f64, ComputeError> {
        match expr {
            Expr::Constant(num) => Ok(*num),
            Expr::Identifier(id) => {
                match self.variables.get(id) {
                    Some(&value) => Ok(value),
                    None => Err(ComputeError::UnrecognizedIdentifier(id.clone())),
                }
            }
            Expr::Neg(expr) => Ok(-self.compute(expr)?),
            Expr::BinOp(op, lexpr, rexpr) => {
                let lnum = self.compute(&lexpr)?;
                let rnum = self.compute(&rexpr)?;

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
                let num = self.compute(&expr)?;
                Ok(match function {
                    Function::Sqrt => num.sqrt(),
                    Function::Sin => num.sin(),
                    Function::Cos => num.cos(),
                    Function::Tan => num.tan(),
                    Function::Log => num.log10(),
                    Function::Abs => num.abs(),
                })
            }
            Expr::Assignment(id, expr) => {
                let value = self.compute(&expr)?;
                self.variables.insert(id.clone(), value);
                Ok(value)
            }
            Expr::Pow(lexpr, rexpr) => {
                Ok(self.compute(&lexpr)?.powf(self.compute(&rexpr)?))
            }
        }
    }

    pub fn get(&self, identifier: &str) -> Option<&f64> {
        self.variables.get(identifier)
    }
}
