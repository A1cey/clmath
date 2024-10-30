mod error;
mod parser;
mod tokenizer;
mod types;

use std::io;

fn main() {
    let input = get_input();

    // if invalid or no input get new input
    if input.is_none() {
        return main();
    }

    match tokenizer::tokenize_input(input.unwrap())
        .and_then(|tokenized_input| parser::parse(&tokenized_input))
    {
        Ok(result) => println!("{}", result),
        Err(err) => error::handle_error(err),
    };

    return main();
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return Some(input.trim().to_string()),
        Err(err) => error::handle_error(types::ErrorType::IoError(err.to_string())),
    };
    None
}
