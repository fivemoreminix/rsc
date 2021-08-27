use crate::{Token, Expr, TokenValue, OpVal, SymbolVal};
use peekmore::{PeekMoreIterator, PeekMore};
use std::slice::Iter;
use std::iter::Peekable;
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub enum ParseErrorCode<'t> {
    ExpectedValue,
    ExpectedClosingParen,
    UnexpectedToken(&'t Token<'t>),
    UnexpectedEOF,
}
use ParseErrorCode::*;

#[derive(Debug, Clone)]
pub struct ParseError<'t> {
    pub code: ParseErrorCode<'t>,
    pub span: Range<usize>,
}

pub type ParseResult<'input> = Result<Expr<'input>, ParseError<'input>>;

macro_rules! error {
    ($code:expr, $span:expr) => {
        ParseError { code: $code, span: $span }
    };
}

type TokenIter<'t> = PeekMoreIterator<Iter<'t, Token<'t>>>;

#[inline(always)]
pub fn parse<'input>(tokens: &'input [Token<'input>]) -> ParseResult<'input> {
    let mut iter = tokens.iter().peekmore();
    parse_expr(&mut iter)
}

#[inline(always)]
fn parse_expr<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    parse_eq(tokens)
}

fn parse_eq<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    let mut result = parse_add(tokens)?;
    while let Some(peek_tok) = tokens.peek() {
        if peek_tok.value == TokenValue::Op(OpVal::Eq) {
            tokens.next(); // Consume '='
            let rhs = parse_add(tokens)?;
            result = Expr::Eq(Box::new(result), Box::new(rhs));
        } else {
            break;
        }
    }
    Ok(result)
}

fn parse_add<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    let mut result = parse_mul(tokens)?;
    while let Some(peek_tok) = tokens.peek() {
        match peek_tok.value {
            TokenValue::Op(op) if op == OpVal::Add || op == OpVal::Sub => {
                tokens.next(); // Consume '+' or '-'
                let rhs = parse_mul(tokens)?;
                result = Expr::Op(op, Box::new(result), Box::new(rhs));
            }
            _ => break,
        }
    }
    Ok(result)
}

fn parse_mul<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    let mut result = parse_pow(tokens)?;
    while let Some(peek_tok) = tokens.peek() {
        match peek_tok.value {
            TokenValue::Op(op) if op == OpVal::Mul || op == OpVal::Div || op == OpVal::Mod => {
                tokens.next(); // Consume '*' or '/' or '%'
                let rhs = parse_pow(tokens)?;
                result = Expr::Op(op, Box::new(result), Box::new(rhs));
            }
            _ => break,
        }
    }
    Ok(result)
}

fn parse_pow<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    let mut result = parse_parentheses_mul(tokens)?;
    while let Some(peek_tok) = tokens.peek() {
        if peek_tok.value == TokenValue::Op(OpVal::Pow) {
            tokens.next(); // Consume '^'
            let rhs = parse_factor(tokens)?;
            result = Expr::Op(OpVal::Pow, Box::new(result), Box::new(rhs));
        } else {
            break;
        }
    }
    Ok(result)
}

fn parse_parentheses_mul<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    if let Some(func_or_var_mul) = parse_func_or_var_mul(tokens) {
        Ok(func_or_var_mul?)
    } else {
        let mut result = parse_factorial(tokens)?;
        while let Some(peek_tok) = tokens.peek() {
            if peek_tok.value == TokenValue::Symbol(SymbolVal::LP) {
                tokens.next(); // Consume '('
                let rhs = parse_expr(tokens)?;
                if let Some(tok) = tokens.next() {
                    if tok.value == TokenValue::Symbol(SymbolVal::RP) {
                        result = Expr::Op(OpVal::Mul, Box::new(result), Box::new(rhs));
                    } else {
                        return Err(error!(ExpectedClosingParen, tok.span.clone()));
                    }
                } else {
                    return Err(error!(UnexpectedEOF, 0..0));
                }
            } else {
                break;
            }
        }
        Ok(result)
    }
}

// This function returns Option to the result, because it doesn't *have* to parse a value.
// And because it should only be used by parse_parentheses_mul.
#[inline]
fn parse_func_or_var_mul<'t, 's>(tokens: &'s mut TokenIter<'t>) -> Option<ParseResult<'t>> {
    match tokens.peek() {
        Some(Token { value: TokenValue::Id(id), .. }) => {
            // Check for opening parentheses
            if let Some(tok) = tokens.peek_nth(1) {
                if tok.value != TokenValue::Symbol(SymbolVal::LP) {
                    return None;
                }
            } else {
                return None;
            }

            // Consume previous tokens
            tokens.next(); // id
            tokens.next(); // '('

            // At this point, the function should always return a Some(...)

            // Shortcut: function has no parameters
            if let Some(tok) = tokens.peek() {
                if tok.value == TokenValue::Symbol(SymbolVal::RP) {
                    tokens.next(); // Consume ')'
                    return Some(Ok(Expr::FuncOrVarMul(*id, Vec::new())));
                }
            }

            // Collecting function parameters
            let mut params = Vec::with_capacity(2);
            while let Ok(expr) = parse_expr(tokens) {
                params.push(expr);
                match tokens.next() {
                    Some(Token { value: TokenValue::Symbol(SymbolVal::Comma), ..}) => {
                        tokens.next(); // Consume ','
                    }
                    Some(Token { value: TokenValue::Symbol(SymbolVal::RP), ..}) => {
                        tokens.next(); // Consume ')'
                        break;
                    }
                    Some(tok) => return Some(Err(error!(UnexpectedToken(tok), tok.span.clone()))),
                    None => return Some(Err(error!(UnexpectedEOF, 0..0))),
                }
            }
            Some(Ok(Expr::FuncOrVarMul(id, params)))
        }
        _ => None,
    }
}

fn parse_factorial<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    let mut result = parse_factor(tokens)?;
    while let Some(peek_tok) = tokens.peek() {
        if peek_tok.value == TokenValue::Op(OpVal::Exclaim) {
            tokens.next(); // Consume '!'
            result = Expr::Factorial(Box::new(result));
        } else {
            break;
        }
    }
    Ok(result)
}

fn parse_factor<'t, 's>(tokens: &'s mut TokenIter<'t>) -> ParseResult<'t> {
    match tokens.next() {
        Some(tok) => match tok.value {
            TokenValue::Num(num) => Ok(Expr::Num(num)),
            TokenValue::Id(id) => Ok(Expr::Var(id)),
            TokenValue::Op(op) => match op {
                OpVal::Sub => Ok(Expr::Neg(Box::new(parse_factor(tokens)?))),
                _ => Err(error!(UnexpectedToken(tok), tok.span.clone()))
            }
            TokenValue::Symbol(sym) => match sym {
                SymbolVal::LP => {
                    let expr = parse_expr(tokens)?;
                    // Expect a closing parentheses
                    if let Some(tok) = tokens.next() {
                        if tok.value == TokenValue::Symbol(SymbolVal::RP) {
                            Ok(expr)
                        } else {
                            Err(error!(UnexpectedToken(tok), tok.span.clone()))
                        }
                    } else {
                        Err(error!(UnexpectedEOF, 0..0))
                    }
                }
                SymbolVal::Pipe => {
                    let expr = parse_expr(tokens)?;
                    // Expect a closing pipe
                    if let Some(tok) = tokens.next() {
                        if tok.value == TokenValue::Symbol(SymbolVal::Pipe) {
                            Ok(Expr::FuncOrVarMul("abs", vec![expr]))
                        } else {
                            Err(error!(UnexpectedToken(tok), tok.span.clone()))
                        }
                    } else {
                        Err(error!(UnexpectedEOF, 0..0))
                    }
                }
                _ => Err(error!(UnexpectedToken(tok), tok.span.clone())),
            }
        }
        None => {
            Err(error!(UnexpectedEOF, 0..0))
        }
    }
}
