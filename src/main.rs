pub mod error;
pub mod parser;

use std::io;

fn main() {
    let input = get_input();

    // if invalid or no input get new input
    if input.is_none() {
        main();
    }

    assert!(input.is_some());

    let parsed_input = parser::parse_input(input.unwrap());
}

struct Variable {
    name: &str,
    value: Option<isize>,
}

enum KeyWords {
    Variable(Variable),
    Function(FunctionTypes),
}

enum FunctionTypes {
    Derivative,
}

const FUNCTIONS: [&str; 1] = ["der"];

fn get_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return Some(input),
        Err(err) => error::handle_error(error::ErrorTypes::IoError(err)),
    };
    None
}
