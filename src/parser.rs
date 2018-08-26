use std::slice::Iter;
use std::iter::Peekable;

use lexer::*;

#[derive(Debug)]
pub enum Expr {
    BinOp(Operator, Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Constant(f64),
}

pub fn parse(tokens: &[Token]) -> Expr {
    parse_additive_expr(&mut tokens.iter().peekable())
}

fn parse_additive_expr(tokens: &mut Peekable<Iter<Token>>) -> Expr {
    let mut expr = parse_multiplicative_expr(tokens);
    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Plus || op == &Operator::Minus => {
                tokens.next();
                let r_expr = parse_multiplicative_expr(tokens);
                expr = Expr::BinOp(*op, Box::new(expr), Box::new(r_expr));
            }
            _ => break,
        }
    }
    expr
}

fn parse_multiplicative_expr(tokens: &mut Peekable<Iter<Token>>) -> Expr {
    let mut expr = parse_parenthetical_multiplicative_expr(tokens);
    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Star || op == &Operator::Slash => {
                tokens.next();
                let r_expr = parse_parenthetical_multiplicative_expr(tokens);
                expr = Expr::BinOp(*op, Box::new(expr), Box::new(r_expr));
            }
            _ => break,
        }
    }
    expr
}

fn parse_parenthetical_multiplicative_expr(tokens: &mut Peekable<Iter<Token>>) -> Expr {
    let expr = parse_power_expr(tokens);
    match tokens.peek() {
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

fn parse_power_expr(tokens: &mut Peekable<Iter<Token>>) -> Expr {
    let mut expr = parse_factor(tokens);
    loop {
        match tokens.peek() {
            Some(Token::Operator(op)) if op == &Operator::Caret => {
                tokens.next();
                let exponent = parse_factor(tokens);
                expr = Expr::Pow(Box::new(expr), Box::new(exponent));
            }
            _ => break,
        }
    }
    expr
}

fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Expr {
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
