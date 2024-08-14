use std::collections::HashMap;

use crate::{error::ErrorTypes, FunctionTypes, KeyWords, Variable, FUNCTIONS};

pub fn parse_input(input: String) -> Result<HashMap<KeyWords, Option<isize>>, ErrorTypes> {
    let mut map: HashMap<KeyWords, Option<isize>> = HashMap::new();

    let splitted_input: Vec<&str> = input.split(' ').collect();

    for arg in splitted_input {
        match arg {
            str if is_only_chars(&arg) => match_for_keyword(arg.clone()),

            _ => {
                return Err(ErrorTypes::ParserError(format!(
                    "Parser could not parse input {}",
                    input
                )))
            }
        };
    }

    Ok(map)
}

fn is_only_chars(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    true
}

fn match_for_keyword<'a>(str: &'a str) -> KeyWords {
    let keyword_idx = FUNCTIONS.iter().position(|&keyword| keyword.eq(str));
    if keyword_idx.is_none() {
        return KeyWords::Variable(Variable {
            name: str,
            value: None,
        });
    };

    assert!(keyword_idx.is_some());

    match keyword_idx.unwrap() {
        0 => KeyWords::Function(FunctionTypes::Derivative),
        _ => panic!(
            "This should not happen. A valid function should exist for this keyword index {}",
            keyword_idx.unwrap()
        ),
    }
}
