use crate::types::{ErrorTypes, Tokens, Variable, FUNCTIONS};

pub fn parse_input(input: String) -> Result<Vec<Tokens>, ErrorTypes> {
    let splitted_input: Vec<&str> = input.split(' ').filter(|slice| (**slice).ne("")).collect();

    let tokenized_input = tokenize(splitted_input);

    return match tokenized_input {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(err),
    };
}

fn tokenize(args: Vec<&str>) -> Result<Vec<Tokens>, ErrorTypes> {
    let mut tokens: Vec<Tokens> = vec![];

    for arg in &args {
        match arg {
            // try tokenizing as keyword
            _ if is_function(&arg) => &mut tokens.push(get_function(&arg)),
            // try tokenizing as number
            _ if is_num(&arg) => &mut tokens.push(convert_to_num(&arg)),
            // try tokenizing as variable
            _ if is_alphanumeric(&arg) => &mut tokens.push(convert_to_var(&arg)),
            _ => {
                return Err(ErrorTypes::ParserError(format!(
                    "Parser could not parse input {:?}",
                    &args
                )))
            }
        };
    }

    Ok(tokens)
}

fn is_alphanumeric(arg: &str) -> bool {
    for c in arg.chars() {
        if !c.is_alphanumeric() {
            return false;
        }
    }
    true
}

fn is_num(arg: &str) -> bool {
    match arg.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn convert_to_num(arg: &str) -> Tokens {
    Tokens::Number(
        arg.parse::<f64>()
            .expect("This should be parsed because it is checked beforeto be a number"),
    )
}

fn is_function(arg: &str) -> bool {
    match FUNCTIONS.iter().find(|(_, keyword)| (*keyword).eq(arg)) {
        Some(_) => true,
        None => false,
    }
}

fn get_function(arg: &str) -> Tokens {
    //search keyword in function list
    let function_idx = FUNCTIONS.iter().position(|(_, keyword)| (*keyword).eq(arg));

    Tokens::Function(
        FUNCTIONS
            .get(
                function_idx.expect(
                    "This should be a valid index because it is checked before to be valid",
                ),
            )
            .unwrap()
            .0
            .clone(),
    )
}

fn convert_to_var(arg: &str) -> Tokens {
    Tokens::Variable(Variable {
        name: arg.to_string(),
        value: None,
    })
}
