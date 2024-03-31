use crate::{Expr, Num, OpVal, SymbolVal, Token, TokenValue};
use peekmore::{PeekMore, PeekMoreIterator};
use std::ops::Range;
use std::slice::Iter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParseErrorCode<'t, N: Num> {
    ExpectedValue,
    ExpectedClosingParen,
    UnexpectedToken(&'t Token<'t, N>),
    UnexpectedEOF,
}
use ParseErrorCode::*;

#[derive(Debug, Clone)]
pub struct ParseError<'t, N: Num> {
    pub code: ParseErrorCode<'t, N>,
    pub span: Range<usize>,
}

pub type ParseResult<'input, N> = Result<Expr<'input, N>, ParseError<'input, N>>;

macro_rules! error {
    ($code:expr, $span:expr) => {
        ParseError {
            code: $code,
            span: $span,
        }
    };
}

type TokenIter<'t, N> = PeekMoreIterator<Iter<'t, Token<'t, N>>>;

pub fn parse<'input, N: Num>(tokens: &'input [Token<'input, N>]) -> ParseResult<'input, N> {
    let mut iter = tokens.iter().peekmore();
    let result = parse_expr(&mut iter);
    match result {
        Ok(_) => {
            if let Some(tok) = iter.next() {
                Err(error!(UnexpectedToken(tok), tok.span.clone()))
            } else {
                result
            }
        }
        Err(_) => result,
    }
}

#[inline]
fn parse_expr<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
    parse_eq(tokens)
}

fn parse_eq<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
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

fn parse_add<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
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

fn parse_mul<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
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

fn parse_pow<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
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

fn parse_parentheses_mul<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
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
fn parse_func_or_var_mul<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> Option<ParseResult<'t, N>> {
    match tokens.peek() {
        Some(Token {
            value: TokenValue::Id(id),
            ..
        }) => {
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
                    return Some(Ok(Expr::FuncOrVarMul(id, Vec::new())));
                }
            }

            // Collecting function parameters
            let mut params = Vec::with_capacity(3);
            while let Ok(expr) = parse_expr(tokens) {
                params.push(expr);
                match tokens.next() {
                    Some(Token {
                        value: TokenValue::Symbol(SymbolVal::Comma),
                        ..
                    }) => {
                        continue;
                    }
                    Some(Token {
                        value: TokenValue::Symbol(SymbolVal::RP),
                        ..
                    }) => {
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

fn parse_factorial<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
    let mut result = parse_factor(tokens)?;
    while let Some(peek_tok) = tokens.peek() {
        if peek_tok.value == TokenValue::Op(OpVal::Exclaim) {
            tokens.next(); // Consume '!'
            result = Expr::FuncOrVarMul("factorial", vec![result]);
        } else {
            break;
        }
    }
    Ok(result)
}

fn parse_factor<'t, N: Num>(tokens: &mut TokenIter<'t, N>) -> ParseResult<'t, N> {
    match tokens.next() {
        Some(tok) => match &tok.value {
            TokenValue::Num(num) => Ok(Expr::Num(num)),
            TokenValue::Id(id) => Ok(Expr::Var(id)),
            TokenValue::Op(op) => match op {
                OpVal::Sub => Ok(Expr::Neg(Box::new(parse_expr(tokens)?))),
                _ => Err(error!(UnexpectedToken(tok), tok.span.clone())),
            },
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
            },
        },
        None => Err(error!(UnexpectedEOF, 0..0)),
    }
}
