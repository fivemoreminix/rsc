pub mod lexer;
pub mod parser;
pub mod computer;

pub fn eval(input: &str) -> Result<f64, String> {
    Ok(computer::compute(&parser::parse(&lexer::tokenize(input)?)))
}
