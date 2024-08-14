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
            // _ if is_alphabetic(&arg) => &mut tokens.push(convert_to_var(&arg)),
            // try interpreting the string as a string wihtout whitespaces (e.g.: "5+6")
            // the tokens are appended within the function
            // if it does not work return an error
            _ => &mut match interpret_string_wo_withespaces(&mut tokens, &arg) {
                Some(err) => return Err(err),
                None => (),
            },
        };
    }

    Ok(tokens)
}

fn interpret_string_wo_withespaces(tokens: &mut Vec<Tokens>, args: &str) -> Option<ErrorTypes> {
    let mut slices_idx: Vec<usize> = vec![];
    // let mut multiplication_idx: Vec<usize> = vec![];

    for (idx, c) in args.chars().enumerate() {
        // // before numbers and after numbers a multiplication operation is added
        // // idx is remembered for slicing
        // if c.is_numeric() {

        //     multiplication_idx.push(idx);
        //     multiplication_idx.push(idx + 1);
            
        //     // slices_idx.push(idx)
        // }
        // non-alphabetic or comma/dot char
        if !is_alphabetic(c.to_string().as_str())
            && !c.to_string().as_str().eq(",")
            && !c.to_string().as_str().eq(".")
        {
            // test char for function
            if is_function(c.to_string().as_str()) {
                // if char is a function the idx is remembered for slicing
                slices_idx.push(idx)
            }
            // if char is not a function a error is returned
            else {
                return Some(ErrorTypes::ParserError(format!(
                    "Parser could not parse input: {}",
                    &args
                )));
            }
        }
    }

    // splitting arg at slice indeces
    let mut splitted_args: Vec<&str> = vec![];

    let mut last_idx: usize = 0;

    for curr_idx in slices_idx {
        if curr_idx != last_idx {
            splitted_args.push(&args[last_idx..curr_idx]);
        }
        splitted_args.push(&args[curr_idx..curr_idx + 1]);
        last_idx = curr_idx + 1;
    }

    if last_idx < args.len() {
        splitted_args.push(&args[last_idx..]);
    }

    // // adding multiplication operations (too many multiplications will later be removed when interpreting the whole formula)
    // for idx in multiplication_idx.iter().rev() {
    //     splitted_args.insert(*idx, "*");
    // }

    // tokenize the slices and append them to the token list
    match tokenize(splitted_args) {
        Ok(mut token_list) => &mut tokens.append(&mut token_list),
        Err(err) => return Some(err),
    };

    None
}

fn is_alphanumeric(arg: &str) -> bool {
    
    for c in arg.chars() {
        if !c.is_alphanumeric() {
            return false;
        }
    }
    true
}

fn is_alphabetic(arg: &str) -> bool {

    for c in arg.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    true
}

fn is_num(arg: &str) -> bool {
    // check if number has leading sign ("+" or "-") because it will be ignored in the conversion into float
    // sign should be seen as Function
    if arg.chars().nth(0).eq(&"+".to_string().chars().nth(0))
        || arg.chars().nth(0).eq(&"-".to_string().chars().nth(0))
    {
        return false;
    }

    match arg.replace(",", ".").parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn convert_to_num(arg: &str) -> Tokens {
    Tokens::Number(
        arg.replace(",", ".")
            .parse::<f64>()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_is_num() {
        assert!(is_num("3"));
        assert!(is_num("3.14"));
        assert!(is_num("3,14"));
        assert!(!is_num("3.1.4"));
        assert!(!is_num("3.1,4"));
        assert!(!is_num("3a"));
        assert!(!is_num("a"));
        assert!(!is_num("3E"));
    }

    #[test]
    fn test_convert_to_num() {
        assert_eq!(convert_to_num("3"), Tokens::Number(3.0));
        assert_eq!(convert_to_num("3.14"), Tokens::Number(3.14));
        assert_eq!(convert_to_num("3,14"), Tokens::Number(3.14));
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(is_alphanumeric("kdsjakl"));
        assert!(is_alphanumeric("djha534"));
        assert!(is_alphanumeric("a1"));
        assert!(!is_alphanumeric("5%"));
    }

    #[test]
    fn test_is_function() {
        for func in FUNCTIONS {
            assert!(is_function(func.1));
        }
    }

    #[test]
    fn test_get_function() {
        for func in FUNCTIONS {
            assert_eq!(get_function(func.1), Tokens::Function(func.0.clone()));
        }
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(vec!["5", "+", "6"]),
            Ok(vec![
                Tokens::Number(5.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0)
            ])
        );
        assert_eq!(
            tokenize(vec!["der", "7", "*", "9", "+", "6"]),
            Ok(vec![
                Tokens::Function(FunctionTypes::Derivative),
                Tokens::Number(7.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(9.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0)
            ])
        );

        assert_eq!(
            tokenize(vec!["/"]),
            Ok(vec![Tokens::Function(FunctionTypes::Division),])
        );
        assert_eq!(
            tokenize(vec!["var"]),
            Ok(vec![Tokens::Variable(Variable {
                name: "var".to_string(),
                value: None
            })])
        );
        assert_eq!(
            tokenize(vec!["var1"]),
            Ok(vec![Tokens::Variable(Variable {
                name: "var1".to_string(),
                value: None
            })])
        );
        assert_eq!(
            tokenize(vec!["var%"]),
            Err(ErrorTypes::ParserError(
                "Parser could not parse input: var%".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("5 +    6".to_string()),
            Ok(vec![
                Tokens::Number(5.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0)
            ])
        );
        assert_eq!(
            parse_input("5 + 6".to_string()),
            Ok(vec![
                Tokens::Number(5.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0)
            ])
        );
        assert_eq!(
            parse_input("der 7 * 9 + 6".to_string()),
            Ok(vec![
                Tokens::Function(FunctionTypes::Derivative),
                Tokens::Number(7.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(9.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0)
            ])
        );

        assert_eq!(
            parse_input("/".to_string()),
            Ok(vec![Tokens::Function(FunctionTypes::Division),])
        );
        assert_eq!(
            parse_input("var".to_string()),
            Ok(vec![Tokens::Variable(Variable {
                name: "var".to_string(),
                value: None
            })])
        );
        assert_eq!(
            parse_input("var1".to_string()),
            Ok(vec![Tokens::Variable(Variable {
                name: "var1".to_string(),
                value: None
            })])
        );
        assert_eq!(
            parse_input("var%".to_string()),
            Err(ErrorTypes::ParserError(
                "Parser could not parse input: var%".to_string()
            ))
        );
    }
}
