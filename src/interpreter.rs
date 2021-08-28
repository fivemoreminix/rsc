use crate::{Expr, OpVal};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Variant {
    Num(f64),
    Function(fn(&[f64]) -> Result<f64, InterpretError>),
}

#[derive(Debug, Clone)]
pub enum InterpretError {
    TooFewArgs(String, usize), // Id of function, min args
    TooManyArgs(String, usize), // Id of function, max args
    VarDoesNotExist(String),
    VarIsNotFunction(String),
    FunctionNameUsedLikeVar(String),
}

#[derive(Clone)]
pub struct Interpreter {
    pub vars: HashMap<String, Variant>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { vars: HashMap::new() }
    }

    pub fn set_var(&mut self, name: String, value: Variant) {
        self.vars.insert(name, value);
    }

    pub fn delete_var(&mut self, name: &str) -> Option<Variant> {
        self.vars.remove(name)
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<f64, InterpretError> {
        // simple, naive recursive tree walk
        match expr {
            Expr::Eq(lhs, rhs) => {
                unimplemented!()
            }
            Expr::Factorial(expr) => unimplemented!(),
            Expr::FuncOrVarMul(id, exprs) => {
                let mut args = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    args.push(self.eval(expr)?);
                }

                if let Some(var) = self.vars.get(*id) {
                    match var {
                        Variant::Num(n) => if args.len() == 1 {
                            Ok(n * args[0])
                        } else {
                            Err(InterpretError::VarIsNotFunction(id.to_string()))
                        },
                        Variant::Function(func) => func(&args),
                    }
                } else {
                    Err(InterpretError::VarDoesNotExist(id.to_string()))
                }
            },
            Expr::Neg(expr) => Ok(-self.eval(expr)?),
            Expr::Num(n) => Ok(*n),
            Expr::Op(op, lhs, rhs) => {
                let lhs = self.eval(lhs)?;
                let rhs = self.eval(rhs)?;
                Ok(match op {
                    OpVal::Add => lhs + rhs,
                    OpVal::Sub => lhs - rhs,
                    OpVal::Mul => lhs * rhs,
                    OpVal::Div => lhs / rhs,
                    OpVal::Mod => lhs % rhs,
                    OpVal::Pow => lhs.powf(rhs),
                    _ => unreachable!(),
                })
            }
            Expr::Var(id) => if let Some(var) = self.vars.get(*id) {
                match var {
                    Variant::Num(n) => Ok(*n),
                    Variant::Function(_) => Err(InterpretError::FunctionNameUsedLikeVar(id.to_string())),
                }
            } else {
                Err(InterpretError::VarDoesNotExist(id.to_string()))
            },
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut vars = HashMap::new();
        vars.insert(String::from("pi"), Variant::Num(std::f64::consts::PI));
        vars.insert(String::from("abs"), Variant::Function(|args| {
            if args.len() > 1 {
                Err(InterpretError::TooManyArgs(String::from("abs"), 1))
            } else if args.len() < 1 {
                Err(InterpretError::TooFewArgs(String::from("abs"), 1))
            } else {
                Ok(args[0].abs())
            }
        }));
        Interpreter { vars }
    }
}
