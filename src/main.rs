mod error;
mod parser;
mod types;

use std::io;

fn main() {
    let input = get_input();

    // if invalid or no input get new input
    if input.is_none() {
        main();
    }

    assert!(input.is_some());

    let parsed_input = parser::parse_input(input.unwrap());

    println!("{:?}", parsed_input);
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return Some(input.trim().to_string()),
        Err(err) => error::handle_error(types::ErrorTypes::IoError(err)),
    };
    None
}
