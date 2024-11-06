use crate::error::Error;
use crate::types::Token;

pub fn parse(tokens: &Vec<Token>) -> Result<String, Error> {
    Ok(format!("{:?}", tokens))
}
