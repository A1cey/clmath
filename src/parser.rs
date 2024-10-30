use crate::types::{ErrorType, Token};

pub fn parse(tokens: &Vec<Token>) -> Result<String, ErrorType> {
    Ok(format!("{:?}", tokens))
}
