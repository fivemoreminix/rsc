use crate::{Expr, Num, OpVal};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone)]
pub enum Variant<N: Num> {
    Num(N),
    Function(for<'expr> fn(&'expr str, &[N]) -> Result<N, InterpretError<'expr>>),
}

#[derive(Debug, Clone)]
pub enum InterpretError<'expr> {
    TooFewArgs(&'expr str, usize),  // Id of function, min args
    TooManyArgs(&'expr str, usize), // Id of function, max args
    VarDoesNotExist(&'expr str),
    VarIsNotFunction(&'expr str),
    FunctionNameUsedLikeVar(&'expr str),
}

#[derive(Clone)]
pub struct Interpreter<N: Num> {
    pub vars: HashMap<String, Variant<N>>,
}

impl<N: Num> Interpreter<N> {
    #[inline(always)]
    pub fn new() -> Interpreter<N> {
        Interpreter {
            vars: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn set_var(&mut self, name: String, value: Variant<N>) {
        self.vars.insert(name, value);
    }

    #[inline(always)]
    pub fn delete_var(&mut self, name: &str) -> Option<Variant<N>> {
        self.vars.remove(name)
    }

    pub fn eval<'expr>(&mut self, expr: &'expr Expr<N>) -> Result<N, InterpretError<'expr>> {
        // simple, naive recursive tree walk
        match expr {
            Expr::Eq(lhs, rhs) => match lhs.deref() {
                Expr::Var(id) => {
                    let result = self.eval(rhs)?;
                    if let Some(val) = self.vars.get_mut(*id) {
                        *val = Variant::Num(result.clone());
                    } else {
                        self.vars
                            .insert(id.to_string(), Variant::Num(result.clone()));
                    }
                    Ok(result)
                }
                _ => todo!("implement algebra solving"),
            },
            Expr::FuncOrVarMul(id, exprs) => {
                let mut args = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    args.push(self.eval(expr)?);
                }

                if let Some(var) = self.vars.get(*id) {
                    match var {
                        Variant::Num(n) => {
                            if args.len() == 1 {
                                let arg = args.remove(0);
                                Ok(n.clone().mul(arg))
                            } else {
                                Err(InterpretError::VarIsNotFunction(id))
                            }
                        }
                        Variant::Function(func) => func(id, &args),
                    }
                } else {
                    Err(InterpretError::VarDoesNotExist(id))
                }
            }
            Expr::Neg(expr) => Ok(-self.eval(expr)?),
            Expr::Num(n) => Ok((**n).clone()),
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
            Expr::Var(id) => {
                if let Some(var) = self.vars.get(*id) {
                    match var {
                        Variant::Num(n) => Ok(n.clone()),
                        Variant::Function(_) => Err(InterpretError::FunctionNameUsedLikeVar(id)),
                    }
                } else {
                    Err(InterpretError::VarDoesNotExist(id))
                }
            }
        }
    }
}

#[inline]
pub fn ensure_arg_count(
    min: usize,
    max: usize,
    args_len: usize,
    func_id: &str,
) -> Result<(), InterpretError> {
    if args_len < min {
        Err(InterpretError::TooFewArgs(func_id, min))
    } else if args_len > max {
        Err(InterpretError::TooManyArgs(func_id, max))
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
        vars.insert(
            String::from("abs"),
            Variant::Function(|id, args| {
                ensure_arg_count(1, 1, args.len(), id)?;
                Ok(args[0].abs())
            }),
        );
        vars.insert(
            String::from("sqrt"),
            Variant::Function(|id, args| {
                ensure_arg_count(1, 1, args.len(), id)?;
                Ok(args[0].sqrt())
            }),
        );
        vars.insert(
            String::from("factorial"),
            Variant::Function(|id, args| {
                ensure_arg_count(1, 1, args.len(), id)?;
                let n = args[0];
                if n <= 1.0 {
                    Ok(1.0)
                } else {
                    Ok((1..=n as u64).product::<u64>() as f64)
                }
            }),
        );
        Interpreter { vars }
    }
}
