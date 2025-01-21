mod error;
mod executor;
mod functions;
mod parser;
mod tokenizer;

use error::{CLMathError, IoError};
use functions::FunctionReturnType;
use std::io;

fn main() {
    loop {
        let input = get_input();
    
        // if invalid or no input get new input
        if input.is_none() {
            continue;
        }
        
        if input.as_ref().unwrap() == "exit" {
            break;
        }
    
        match tokenizer::tokenize(input.unwrap()) {
            Ok(tokens) => {
                match parser::Parser::parse(tokens) {
                    Ok(expression) => match executor::execute(expression) {
                        Ok(result) => print_result(result),
                        Err(err) => error::handle_error(err),
                    },
                    Err(err) => error::handle_error(err),
                }
            }
            Err(err) => error::handle_errors(err),
        };
    }
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return Some(input.trim().to_string()),
        Err(err) => error::handle_error(CLMathError::Io(IoError::InvalidUTF8(err.to_string()))),
    };
    None
}

fn print_result(result: Option<FunctionReturnType>) {
    if let Some(val) = result {
        println!("{val}")
    } else {
        println!();
    }
}
