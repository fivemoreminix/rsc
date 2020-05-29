//! For making notable symbols and words out of text.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    // Numerical
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
    Pipe,
    Equal, // Comparison Equal or assignment
    Exclamation, // Boolean NOT or factorial

    // Comparison
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    NotEqual,
}
use self::Operator::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    // Boolean
    True,
    False,
    And,
    Or
}
use self::Keyword::*;
use crate::computer::Num;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue<'a, T> {
    Number(Box<T>), // TODO: T may be a very large size, we might need to box it
    Operator(Operator),
    Keyword(Keyword),
    Identifier(&'a str),
}

#[derive(Debug, Clone)]
pub struct Token<'a, T> {
    pub(crate) value: TokenValue<'a, T>, // Token's value
    pub(crate) lexeme: &'a str, // Slice of source of token
    pub(crate) pos: usize, // Index of source the lexeme begins
}

impl<'a, T> Token<'a, T> {
    // pub fn new(value: TokenValue<'a, T>, lexeme: &'a str, pos: usize) -> Token<'a, T> {
    //     Token { value, lexeme, pos }
    // }

    pub fn value(&self) -> &TokenValue<'a, T> {
        &self.value
    }

    pub fn lexeme(&self) -> &'a str {
        self.lexeme
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

#[derive(Debug)]
pub struct Lexer<'a, T> {
    source: String,
    pub tokens: Vec<Token<'a, T>>
}

/// # Error Lookup Table
/// | Error ID         | Description                                                                                           |
/// |------------------|-------------------------------------------------------------------------------------------------------|
/// | InvalidCharacter | If the input contains any characters not recognized by the lexer to be numbers or characters, ex: 'ƒ' |
/// | InvalidNumber    | A number entered invalidly: '2.34.2' or '..3'                                                         |
#[derive(Debug, Clone, PartialEq)]
pub enum LexerError<'a> {
    InvalidCharacter(char),
    InvalidNumber(&'a str),
}

/// As a result of the borrow-checker in Rust, this is the best solution
/// which is not verbose nor a complete macro hack.
/// Contribution from matt1992#5582
trait AddToken<'a, T>{
    fn add(&mut self, value: TokenValue<'a, T>, lexeme: &'a str, pos: usize);
}

impl<'a, T> AddToken<'a, T> for Vec<Token<'a, T>> {
    fn add(&mut self, value: TokenValue<'a, T>, lexeme: &'a str, pos: usize) {
        self.push(Token { value, lexeme, pos });
    }
}

impl<'a, T> Lexer<'a, T> {
    pub fn new<S>(input: S) -> Lexer<'a, T>
        where S: AsRef<str>,
    {
        Lexer { source: input.as_ref().to_owned(), tokens: Vec::new() }
    }

    pub fn tokens(&self) -> &[Token<'a, T>] {
        &self.tokens
    }

    pub fn scan(&'a mut self) -> Result<&[Token<'a, T>], LexerError>
        where T: Num,
    {
        let mut chars = self.source.chars().enumerate().peekable();

        while let Some((i, c)) = chars.next() {
            match c {
                '+' => self.tokens.add(TokenValue::Operator(Plus), &self.source[i..=i], i),
                '-' => self.tokens.add(TokenValue::Operator(Minus), &self.source[i..=i], i),
                '*' => self.tokens.add(TokenValue::Operator(Star), &self.source[i..=i], i),
                '/' => self.tokens.add(TokenValue::Operator(Slash), &self.source[i..=i], i),
                '%' => self.tokens.add(TokenValue::Operator(Percent), &self.source[i..=i], i),
                '^' => self.tokens.add(TokenValue::Operator(Caret), &self.source[i..=i], i),
                '(' => self.tokens.add(TokenValue::Operator(LParen), &self.source[i..=i], i),
                ')' => self.tokens.add(TokenValue::Operator(RParen), &self.source[i..=i], i),
                '|' => self.tokens.add(TokenValue::Operator(Pipe), &self.source[i..=i], i),
                '=' => self.tokens.add(TokenValue::Operator(Equal), &self.source[i..=i], i),
                '!' => match chars.peek() {
                    Some((_, '=')) => {
                        chars.next(); // Consume =
                        self.tokens.add(TokenValue::Operator(NotEqual), &self.source[i..=i+1], i);
                    }
                    _ => self.tokens.add(TokenValue::Operator(Exclamation), &self.source[i..=i], i),
                }
                '>' => match chars.peek() {
                    Some((_, '=')) => {
                        chars.next(); // Consume =
                        self.tokens.add(TokenValue::Operator(GreaterEqual), &self.source[i..=i+1], i);
                    }
                    _ => self.tokens.add(TokenValue::Operator(Greater), &self.source[i..=i], i),
                }
                '<' => match chars.peek() {
                    Some((_, '=')) => {
                        chars.next(); // Consume =
                        self.tokens.add(TokenValue::Operator(LesserEqual), &self.source[i..=i+1], i);
                    }
                    _ => self.tokens.add(TokenValue::Operator(Lesser), &self.source[i..=i], i),
                }
                _ => {
                    if c.is_whitespace() {
                        continue;
                    } else if c.is_digit(10) || c == '.' { // Parse a number
                        let start_idx = i;
                        let mut end_idx = i;

                        while let Some((_, c)) = chars.peek() {
                            if c.is_digit(10) || c == &'.' {
                                chars.next(); // consume the character
                                end_idx += 1;
                            } else {
                                break;
                            }
                        }

                        match T::from_flt64_str(&self.source[start_idx..=end_idx]) {
                            Some(num) => self.tokens.add(TokenValue::Number(Box::new(num)), &self.source[start_idx..=end_idx], start_idx),
                            _ => return Err(LexerError::InvalidNumber(&self.source[start_idx..=end_idx])),
                        }
                    } else if c.is_alphabetic() { // Parse an identifier or keyword
                        let start_idx = i;
                        let mut end_idx = i;

                        while let Some((_, c)) = chars.peek() {
                            if c.is_alphabetic() || c == &'_' {
                                chars.next();
                                end_idx += 1;
                            } else {
                                break;
                            }
                        }

                        match &self.source[start_idx..=end_idx] {
                            "true" => self.tokens.add(TokenValue::Keyword(True), &self.source[start_idx..=end_idx], start_idx),
                            "false" => self.tokens.add(TokenValue::Keyword(False), &self.source[start_idx..=end_idx], start_idx),
                            "and" => self.tokens.add(TokenValue::Keyword(And), &self.source[start_idx..=end_idx], start_idx),
                            "or" => self.tokens.add(TokenValue::Keyword(Or), &self.source[start_idx..=end_idx], start_idx),
                            id => self.tokens.add(TokenValue::Identifier(id), &self.source[start_idx..=end_idx], start_idx),
                        }
                    } else {
                        return Err(LexerError::InvalidCharacter(c));
                    }
                }
            }
        }

        Ok(&self.tokens)
    }
}
