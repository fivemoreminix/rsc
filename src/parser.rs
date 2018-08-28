use std::slice::Iter;
use std::iter::Peekable;

use lexer::*;

#[derive(Debug)]
pub enum Expr {
    BinOp(Operator, Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Constant(f64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParserError {
    ExpectedClosingParenthesis,
    ExpectedFactor(Option<Token>), // Includes the token it found instead
}
use self::ParserError::*;

pub fn parse(tokens: &[Token]) -> Result<Expr, ParserError> {
    parse_additive_expr(&mut tokens.iter().peekable())
}

fn parse_additive_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
    let mut expr = parse_multiplicative_expr(tokens)?;
    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Plus || op == &Operator::Minus => {
                tokens.next();
                let r_expr = parse_multiplicative_expr(tokens)?;
                expr = Expr::BinOp(*op, Box::new(expr), Box::new(r_expr));
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_multiplicative_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
    let mut expr = parse_parenthetical_multiplicative_expr(tokens)?;
    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Star || op == &Operator::Slash => {
                tokens.next();
                let r_expr = parse_parenthetical_multiplicative_expr(tokens)?;
                expr = Expr::BinOp(*op, Box::new(expr), Box::new(r_expr));
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_parenthetical_multiplicative_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
    let expr = parse_power_expr(tokens)?;
    match tokens.peek() {
        Some(Token::Operator(op)) if op == &Operator::LParen => {
            tokens.next();
            let internal_expr = parse_additive_expr(tokens)?;
            match tokens.next() {
                Some(Token::Operator(op)) if op == &Operator::RParen => return Ok(Expr::BinOp(Operator::Star, Box::new(expr), Box::new(internal_expr))),
                _ => return Err(ExpectedClosingParenthesis),
            }
        }
        _ => {}
    }
    Ok(expr)
}

fn parse_power_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
    let mut expr = parse_factor(tokens)?;
    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Caret => {
                tokens.next();
                let exponent = parse_factor(tokens)?;
                expr = Expr::Pow(Box::new(expr), Box::new(exponent));
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
    match tokens.next() {
        Some(Token::Operator(Operator::LParen)) => {
            let expr = parse_additive_expr(tokens);
            match tokens.next() {
                Some(Token::Operator(Operator::RParen)) => expr,
                _ => Err(ExpectedClosingParenthesis),
            }
        }
        Some(Token::Operator(Operator::Minus)) => {
            Ok(Expr::Neg(Box::new(parse_factor(tokens)?)))
        }
        Some(Token::Number(n)) => Ok(Expr::Constant(*n)),
        t => Err(ExpectedFactor(t.cloned())),
    }
}
