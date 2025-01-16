mod error;
mod functions;
mod parser;
mod tokenizer;

use error::{Error, IoError};
use std::io;

fn main() {
    let input = get_input();

    // if invalid or no input get new input
    if input.is_none() {
        return main();
    }

    match tokenizer::tokenize(input.unwrap()).and_then(|tokens| parser::Parser::parse(tokens)) {
        Ok(result) => println!("{:?}", result),
        Err(err) => error::handle_errors(err),
    };

    return main();
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return Some(input.trim().to_string()),
        Err(err) => error::handle_errors(vec![Error::new(
            err.to_string(),
            Box::new(IoError::InvalidUTF8),
            None,
            None,
        )]),
    };
    None
}
