pub mod lexer;
pub mod parser;
pub mod computer;

pub enum EvalError {
    ParserError(parser::ParserError),
    LexerError(lexer::LexerError),
}

pub fn eval(input: &str) -> Result<f64, EvalError> {
    match lexer::tokenize(input) {
        Ok(tokens) => match parser::parse(&tokens) {
            Ok(ast) => Ok(computer::compute(&ast)),
            Err(parser_err) => Err(EvalError::ParserError(parser_err)),
        }
        Err(lexer_err) => Err(EvalError::LexerError(lexer_err)),
    }
}
