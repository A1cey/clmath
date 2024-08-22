mod error;
mod parser;
mod evaluator;
mod types;

use std::io;

fn main() {
    let input = get_input();

    // if invalid or no input get new input
    if input.is_none() {
        return main();
    }

    assert!(
        input.is_some(),
        "input should be available when this is run"
    );

    let parsed_input = parser::parse_input(input.unwrap());

    if parsed_input.is_err() {
        error::handle_error(parsed_input.unwrap_err());
        return main();
    }

    assert!(
        parsed_input.is_ok(),
        "parsed input should be ok when this is run"
    );
    println!("{:?}", parsed_input);

    let result = evaluator::evaluate(&parsed_input.unwrap());

    if result.is_err() {
        error::handle_error(result.unwrap_err());
        return main();
    }

    assert!(result.is_ok(), "result should be ok when this is run");
    println!("{}", result.unwrap());
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return Some(input.trim().to_string()),
        Err(err) => error::handle_error(types::ErrorTypes::IoError(err.to_string())),
    };
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    fn test_main() {
        assert!(false, "test needs to be written")
    }
}
