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
            // try tokenizing as function
            _ if is_func(&arg) => tokens.push(tokenize_as_func(&arg)),
            // try tokenizing as number
            _ if is_num_str(&arg) => tokens.push(tokenize_as_num(&arg)),
            // try tokenizing as var
            _ if is_alphabetic(&arg) => tokens.push(tokenize_as_var(&arg)),
            // try tokenizing as variable, nums will be padded with multiplications -> (6a -> 6 * a)
            _ if is_alphanumeric(&arg) => match split_num_and_str(&mut tokens, &arg) {
                Some(err) => return Err(err),
                None => (),
            },
            // try interpreting the string as a string wihtout whitespaces (e.g.: "5+6")
            // the tokens are appended within the function
            // if it does not work return an error
            _ => match interpret_string_wo_withespaces(&mut tokens, &arg) {
                Some(err) => return Err(err),
                None => (),
            },
        };
    }

    Ok(tokens)
}

fn interpret_string_wo_withespaces(tokens: &mut Vec<Tokens>, args: &str) -> Option<ErrorTypes> {
    let mut slices_idx: Vec<usize> = vec![];

    for (idx, c) in args.chars().enumerate() {
        // non-alphabetic or comma/dot char
        if !is_alphanumeric(c.to_string().as_str()) && !c.eq(&'.') && !c.eq(&'.') {
            // test char for function
            if is_func(c.to_string().as_str()) {
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

    // tokenize the slices and append them to the token list
    match tokenize(splitted_args) {
        Ok(mut token_list) => &mut tokens.append(&mut token_list),
        Err(err) => return Some(err),
    };

    None
}

fn is_alphanumeric(arg: &str) -> bool {
    for c in arg.chars() {
        if !c.is_alphanumeric() && c.ne(&',') && c.ne(&'.') {
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

fn is_num_str(arg: &str) -> bool {
    // check if number has leading sign ("+" or "-") because it will be ignored in the conversion into float
    // sign should be seen as Function
    if arg.chars().nth(0).unwrap().eq(&'+') || arg.chars().nth(0).unwrap().eq(&'-') {
        return false;
    }

    match arg.replace(",", ".").parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_num_char(c: char) -> bool {
    c.is_numeric() || c.eq(&',') || c.eq(&'.')
}

fn tokenize_as_num(arg: &str) -> Tokens {
    Tokens::Number(
        arg.replace(",", ".")
            .parse::<f64>()
            .expect("This should be parsed because it is checked before to be a number"),
    )
}

fn tokenize_as_var(arg: &str) -> Tokens {
    Tokens::Variable(Variable { name: arg.to_string(), value: None })
}

fn is_func(arg: &str) -> bool {
    match FUNCTIONS.iter().find(|(_, keyword)| (*keyword).eq(arg)) {
        Some(_) => true,
        None => false,
    }
}

fn tokenize_as_func(arg: &str) -> Tokens {
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

fn split_num_and_str(tokens: &mut Vec<Tokens>, args: &str) -> Option<ErrorTypes> {
    let mut multiplication_idx: Vec<usize> = vec![];

    for (idx, c) in args.chars().enumerate() {
        // before numbers and after numbers a multiplication operation is added
        // idx is remembered for slicing
        if is_num_char(c) {
            // add multiplication before num if there is no part of the num
            if idx != 0 && !is_num_char(args.chars().nth(idx - 1).unwrap()) {
                multiplication_idx.push(idx);
            }
            // add multiplication after num if there is no part of the num
            if idx != args.len() - 1 && !is_num_char(args.chars().nth(idx + 1).unwrap()) {
                multiplication_idx.push(idx + 1);
            }
        }
        // returns error if there are non-alpanumeric chars
        else if !c.is_alphabetic() {
            return Some(ErrorTypes::ParserError(format!(
                "Parser could not parse input: {}",
                &args
            )));
        }
    }

    let mut args_with_mult = args.to_string();

    // adding multiplication operations
    for idx in multiplication_idx.iter().rev() {
        args_with_mult.insert_str(*idx, " * ");
    }

    // splitting arg at slice indeces and tokenize the slices and append them to the token list
    match tokenize(
        args_with_mult
            .split(" ")
            .filter(|slice| (**slice).ne(""))
            .collect(),
    ) {
        Ok(mut token_list) => &mut tokens.append(&mut token_list),
        Err(err) => return Some(err),
    };

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_is_num_str() {
        assert!(is_num_str("3"));
        assert!(is_num_str("3.14"));
        assert!(is_num_str("3,14"));
        assert!(!is_num_str("3.1.4"));
        assert!(!is_num_str("3.1,4"));
        assert!(!is_num_str("3a"));
        assert!(!is_num_str("a"));
        assert!(!is_num_str("3E"));
    }

    #[test]
    fn test_is_num_char() {
        assert!(is_num_char('3'));
        assert!(is_num_char('.'));
        assert!(is_num_char(','));
        assert!(!is_num_char('a'));
        assert!(!is_num_char('$'));
    }

    #[test]
    fn test_convert_to_num() {
        assert_eq!(convert_to_num("3"), Tokens::Number(3.0));
        assert_eq!(convert_to_num("3.14"), Tokens::Number(3.14));
        assert_eq!(convert_to_num("3,14"), Tokens::Number(3.14));
    }

    #[test]
    fn test_convert_to_var() {
        assert_eq!(
            parse_input("var".to_string()),
            Ok(vec![Tokens::Variable(Variable {
                name: "var".to_string(),
                value: None
            })])
        );
        assert_eq!(
            parse_input("76var1a6,1b5.01".to_string()),
            Ok(vec![
                Tokens::Number(76.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(1.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Variable(Variable {
                    name: "a".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(6.1),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Variable(Variable {
                    name: "b".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(5.01),
            ])
        );
        assert_eq!(
            parse_input("var$".to_string()),
            Err(ErrorTypes::ParserError(
                "Parser could not parse input: var$".to_string()
            ))
        );
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
    fn test_interpret_str_wo_whitespaces() {
        let mut tokens: Vec<Tokens> = vec![];

        assert_eq!(interpret_string_wo_withespaces(&mut tokens, "5-"), None);
        assert_eq!(
            interpret_string_wo_withespaces(&mut tokens, "der7*9+6"),
            None
        );
        assert_eq!(
            interpret_string_wo_withespaces(&mut tokens, "7der+6,0*3"),
            None
        );
        assert_eq!(
            tokens,
            vec![
                Tokens::Number(5.0),
                Tokens::Function(FunctionTypes::Subtraction),
                Tokens::Function(FunctionTypes::Derivative),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(7.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(9.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0),
                Tokens::Number(7.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Function(FunctionTypes::Derivative),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(3.0),
            ]
        );
        assert_eq!(
            interpret_string_wo_withespaces(&mut tokens, "7&"),
            Some(ErrorTypes::ParserError(
                "Parser could not parse input: 7&".to_string()
            ))
        )
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
            Ok(vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(1.0)
            ])
        );
        assert_eq!(
            tokenize(vec!["var$"]),
            Err(ErrorTypes::ParserError(
                "Parser could not parse input: var$".to_string()
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
            Ok(vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(1.0)
            ])
        );
        assert_eq!(
            parse_input("var$".to_string()),
            Err(ErrorTypes::ParserError(
                "Parser could not parse input: var$".to_string()
            ))
        );
    }
}
