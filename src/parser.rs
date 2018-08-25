use lexer::*;
use std::slice::Iter;
use peek_nth::{IteratorExt, PeekableNth};

#[derive(Debug)]
pub enum Expr {
    BinOp(Operator, Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Constant(f64),
}

pub fn parse(tokens: &[Token]) -> Expr {
    let mut iterator = tokens.iter().peekable_nth();
    parse_additive_expr(&mut iterator)
}

fn parse_additive_expr(tokens: &mut PeekableNth<Iter<Token>>) -> Expr {
    let expr = parse_multiplicative_expr(tokens);
    loop {
        match tokens.peek_nth(0) {
            Some(Token::Operator(op)) if op == &Operator::Plus || op == &Operator::Minus => {
                tokens.next();
                let r_expr = parse_additive_expr(tokens);
                return Expr::BinOp(*op, Box::new(expr), Box::new(r_expr));
            }
            _ => break,
        }
    }
    expr
}

fn parse_multiplicative_expr(tokens: &mut PeekableNth<Iter<Token>>) -> Expr {
    let expr = parse_parenthetical_multiplicative_expr(tokens);
    loop {
        match tokens.peek_nth(0) {
            Some(Token::Operator(op)) if op == &Operator::Star || op == &Operator::Slash => {
                tokens.next();
                let r_expr = parse_additive_expr(tokens);
                return Expr::BinOp(*op, Box::new(expr), Box::new(r_expr));
            }
            _ => break,
        }
    }
    expr
}

fn parse_parenthetical_multiplicative_expr(tokens: &mut PeekableNth<Iter<Token>>) -> Expr {
    let expr = parse_power_expr(tokens);
    match tokens.peek_nth(0) {
        Some(Token::Operator(op)) if op == &Operator::LParen => {
            tokens.next();
            let internal_expr = parse_additive_expr(tokens);
            match tokens.next() {
                Some(Token::Operator(op)) if op == &Operator::RParen => return Expr::BinOp(Operator::Star, Box::new(expr), Box::new(internal_expr)),
                _ => panic!("Expected closing parenthesis"),
            }
        }
        _ => {}
    }
    expr
}

fn parse_power_expr(tokens: &mut PeekableNth<Iter<Token>>) -> Expr {
    let expr = parse_factor(tokens);
    match tokens.peek_nth(0) {
        Some(Token::Operator(op)) if op == &Operator::Caret => {
            tokens.next();
            let r_expr = parse_factor(tokens);
            return Expr::Pow(Box::new(expr), Box::new(r_expr));
        }
        _ => {}
    }
    expr
}

fn parse_factor(tokens: &mut PeekableNth<Iter<Token>>) -> Expr {
    match tokens.next() {
        Some(Token::Operator(Operator::LParen)) => {
            let expr = parse_additive_expr(tokens);
            match tokens.next() {
                Some(Token::Operator(Operator::RParen)) => expr,
                _ => panic!("Expected closing parenthesis"),
            }
        }
        Some(Token::Number(n)) => Expr::Constant(*n),
        t => panic!("Expected factor, found {:?}", t),
    }
}
