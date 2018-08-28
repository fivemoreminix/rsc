use lexer::*;
use parser::*;

pub fn compute(expr: &Expr) -> f64 {
    match expr {
        Expr::Constant(num) => *num,
        Expr::Neg(expr) => -compute(expr),
        Expr::BinOp(op, lexpr, rexpr) => {
            let lnum = compute(&lexpr);
            let rnum = compute(&rexpr);

            match op {
                Operator::Plus => lnum + rnum,
                Operator::Minus => lnum - rnum,
                Operator::Star => lnum * rnum,
                Operator::Slash => lnum / rnum,
                _ => unimplemented!(),
            }
        }
        Expr::Pow(lexpr, rexpr) => {
            compute(&lexpr).powf(compute(&rexpr))
        }
    }
}
