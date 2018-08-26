#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}
use self::Operator::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::<Token>::new();

    let chars: Vec<char> = input.chars().collect();

    let mut i = 0usize;
    while i < chars.len() {
        match chars[i] {
            '+' => tokens.push(Token::Operator(Plus)),
            '-' => tokens.push(Token::Operator(Minus)),
            '*' => tokens.push(Token::Operator(Star)),
            '/' => tokens.push(Token::Operator(Slash)),
            '^' => tokens.push(Token::Operator(Caret)),
            '(' => tokens.push(Token::Operator(LParen)),
            ')' => tokens.push(Token::Operator(RParen)),
            c => {
                if c.is_whitespace() {
                    break;
                }

                if is_digit(c) || c == '.' {
                    let mut number_string = c.to_string();
                    
                    i += 1;
                    while i < chars.len() && (is_digit(chars[i]) || chars[i] == '.') {
                        number_string.push(chars[i]);
                        i += 1;
                    }

                    let number;
                    match number_string.parse::<f64>() {
                        Ok(num) => number = num,
                        _ => return Err(format!("Invalid number entered: {:?}", number_string)),
                    }

                    tokens.push(Token::Number(number));

                    continue; // we i += 1 at end of while
                } else {
                    println!("I don't understand. Please type only math equations. It's all I know.");
                }
            }
        }
        i += 1;
    }
    
    Ok(tokens)
}

fn is_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}
