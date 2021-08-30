use crate::{Expr, OpVal, Num};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone)]
pub enum Variant<N: Num> {
    Num(N),
    Function(fn(&str, &[N]) -> Result<N, InterpretError>),
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
pub struct Interpreter<N: Num> {
    pub vars: HashMap<String, Variant<N>>,
}

impl<N: Num> Interpreter<N> {
    #[inline(always)]
    pub fn new() -> Interpreter<N> {
        Interpreter { vars: HashMap::new() }
    }

    #[inline(always)]
    pub fn set_var(&mut self, name: String, value: Variant<N>) {
        self.vars.insert(name, value);
    }

    #[inline(always)]
    pub fn delete_var(&mut self, name: &str) -> Option<Variant<N>> {
        self.vars.remove(name)
    }

    pub fn eval(&mut self, expr: &Expr<N>) -> Result<N, InterpretError> {
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
                            let arg = args.remove(0);
                            Ok(n.clone().mul(arg))
                        } else {
                            Err(InterpretError::VarIsNotFunction(id.to_string()))
                        },
                        Variant::Function(func) => func(id, &args),
                    }
                } else {
                    Err(InterpretError::VarDoesNotExist(id.to_string()))
                }
            },
            Expr::Neg(expr) => Ok(-self.eval(expr)?),
            Expr::Num(n) => Ok(n.deref().clone()),
            Expr::Op(op, lhs, rhs) => {
                let lhs = self.eval(lhs)?;
                let rhs = self.eval(rhs)?;
                Ok(match op {
                    OpVal::Add => lhs + rhs,
                    OpVal::Sub => lhs - rhs,
                    OpVal::Mul => lhs * rhs,
                    OpVal::Div => lhs / rhs,
                    OpVal::Mod => lhs % rhs,
                    OpVal::Pow => lhs.pow(rhs),
                    _ => unreachable!(),
                })
            }
            Expr::Var(id) => if let Some(var) = self.vars.get(*id) {
                match var {
                    Variant::Num(n) => Ok(n.clone()),
                    Variant::Function(_) => Err(InterpretError::FunctionNameUsedLikeVar(id.to_string())),
                }
            } else {
                Err(InterpretError::VarDoesNotExist(id.to_string()))
            },
        }
    }
}

#[inline]
pub fn ensure_arg_count(min: usize, max: usize, args_len: usize, func_id: &str) -> Result<(), InterpretError> {
    if args_len < min {
        Err(InterpretError::TooFewArgs(func_id.to_string(), min))
    } else if args_len > max {
        Err(InterpretError::TooManyArgs(func_id.to_string(), max))
    } else {
        Ok(())
    }
}

impl Default for Interpreter<f64> {
    fn default() -> Self {
        let mut vars = HashMap::new();
        vars.insert(String::from("pi"), Variant::Num(std::f64::consts::PI));
        vars.insert(String::from("e"), Variant::Num(std::f64::consts::E));
        vars.insert(String::from("tau"), Variant::Num(std::f64::consts::TAU));
        vars.insert(String::from("abs"), Variant::Function(|id, args| {
            match ensure_arg_count(1, 1, args.len(), id) {
                Ok(()) => Ok(args[0].abs()),
                Err(e) => Err(e),
            }
        }));
        vars.insert(String::from("sqrt"), Variant::Function(|id, args| {
            match ensure_arg_count(1, 1, args.len(), id) {
                Ok(()) => Ok(args[0].sqrt()),
                Err(e) => Err(e),
            }
        }));
        Interpreter { vars }
    }
}
