use std::{
    any::{Any, TypeId},
    fmt::format,
    vec,
};

use crate::types::{
    CharOrStr, ErrorTypes, FunctionTypes, SymbolTypes, Tokens, Variable, FUNCTIONS, SYMBOLS,
};

pub fn parse_input(input: String) -> Result<Vec<Tokens>, ErrorTypes> {
    let splitted_input: Vec<&str> = input.split(' ').filter(|slice| (**slice).ne("")).collect();

    let tokenized_input = tokenize(splitted_input);

    if tokenized_input.is_err() {
        return tokenized_input;
    }

    let token_list = add_multiplications(&mut tokenized_input.unwrap());

    return Ok(token_list);
}

fn tokenize(args: Vec<&str>) -> Result<Vec<Tokens>, ErrorTypes> {
    let mut tokens: Vec<Tokens> = vec![];

    for arg in args {
        match arg {
            // try tokenizing as function
            _ if is_func(arg) => tokens.push(tokenize_as_func(arg)),
            // try tokenizing as number
            _ if is_num(arg) => tokens.push(tokenize_as_num(arg)),
            // try tokenizing as var
            _ if is_alphabetic(arg) => tokens.push(tokenize_as_var(arg)),
            // try tokenizing as symbol, multiplication will be added before opening bracket and after closing bracket
            _ if is_symbol(arg) => tokens.append(&mut tokenize_as_symbol(arg)),
            // try tokenizing as variable, nums will be padded with multiplications -> (6a -> 6 * a)
            _ if is_alphanumeric(arg) => tokens.append(&mut split_num_and_str(arg)),
            // try interpreting the string as a string wihtout whitespaces (e.g.: "5+6")
            // the tokens are appended within the function
            // if it does not work return an error
            _ => match interpret_string_wo_withespaces(arg) {
                Ok(mut token_list) => tokens.append(&mut token_list),
                Err(err) => return Err(err),
            },
        };
    }

    Ok(tokens)
}

/// Adds multiplications where nessecary:
/// 5 a -> 5 * a
/// 5 der -> 5 * der
/// 5 (5 + 5) 5 -> 5 * (5 + 5) * 5
/// () * () -> () * ()
fn add_multiplications(tokens: &mut Vec<Tokens>) -> Vec<Tokens> {
    let mut multiplication_idx: Vec<usize> = vec![];

    for (idx, token) in tokens.iter().enumerate() {
        if idx + 1 != tokens.len() {
            match token {
                Tokens::Number(_)
                | Tokens::Symbol(SymbolTypes::ClosingBracket)
                | Tokens::Variable(_) => {
                    match tokens.get(idx + 1).unwrap() {
                        Tokens::Function(FunctionTypes::Derivative)
                        | Tokens::Number(_)
                        | Tokens::Symbol(SymbolTypes::OpeningBracket)
                        | Tokens::Variable(_) => multiplication_idx.push(idx + 1),
                        _ => (),
                    };
                }
                _ => (),
            };
        }
    }

    for idx in multiplication_idx.iter().rev() {
        tokens.insert(*idx, Tokens::Function(FunctionTypes::Multiplication));
    }

    tokens.clone()
}

fn interpret_string_wo_withespaces(args: &str) -> Result<Vec<Tokens>, ErrorTypes> {
    let mut tokens: Vec<Tokens> = vec![];
    let mut slices_idx: Vec<usize> = vec![];

    for (idx, c) in args.chars().enumerate() {
        // non-alphabetic or comma/dot char
        if !is_alphanumeric(c.to_string().as_str()) && !c.eq(&'.') && !c.eq(&'.') {
            // test char for function
            if is_func(c.to_string().as_str()) || is_symbol(c.to_string().as_str()) {
                // if char is a function or symbol the idx is remembered for slicing
                slices_idx.push(idx)
            }
            // if char is not a function a error is returned
            else {
                return Err(ErrorTypes::ParserError(format!(
                    "Parser could not parse input: {}",
                    args
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
        Ok(mut token_list) => {
            tokens.append(&mut token_list);
            Ok(tokens)
        }
        Err(err) => Err(err),
    }
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

fn is_symbol(arg: &str) -> bool {
    match SYMBOLS.iter().find(|(_, keyword)| (*keyword).eq(arg)) {
        Some(_) => true,
        None => false,
    }
}

fn is_num<'a, T: Into<CharOrStr<'a>>>(arg: T) -> bool {
    match arg.into() {
        CharOrStr::Char(c) => c.is_numeric() || c.eq(&',') || c.eq(&'.'),
        CharOrStr::Str(str) => {
            // Checks if number has leading sign ("+" or "-") because it will be ignored in the conversion into float
            // sign should be seen as function
            if str.chars().nth(0).unwrap().eq(&'+') || str.chars().nth(0).unwrap().eq(&'-') {
                return false;
            }

            match str.replace(",", ".").parse::<f64>() {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }
}

fn is_func(arg: &str) -> bool {
    match FUNCTIONS.iter().find(|(_, keyword)| (*keyword).eq(arg)) {
        Some(_) => true,
        None => false,
    }
}

fn tokenize_as_num(arg: &str) -> Tokens {
    Tokens::Number(
        arg.replace(",", ".")
            .parse::<f64>()
            .expect("This should be parsed because it is checked before to be a number"),
    )
}

fn tokenize_as_var(arg: &str) -> Tokens {
    Tokens::Variable(Variable {
        name: arg.to_string(),
        value: None,
    })
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

fn tokenize_as_symbol(arg: &str) -> Vec<Tokens> {
    //search keyword in function list
    let symbol_idx = SYMBOLS.iter().position(|(_, keyword)| (*keyword).eq(arg));

    let symbol = SYMBOLS
        .get(
            symbol_idx
                .expect("This should be a valid index because it is checked before to be valid"),
        )
        .unwrap()
        .0
        .clone();

    match symbol {
        SymbolTypes::OpeningBracket => vec![
            Tokens::Function(FunctionTypes::Multiplication),
            Tokens::Symbol(SymbolTypes::OpeningBracket),
        ],
        SymbolTypes::ClosingBracket => vec![
            Tokens::Symbol(SymbolTypes::ClosingBracket),
            Tokens::Function(FunctionTypes::Multiplication),
        ],
    }
}

fn split_num_and_str(arg: &str) -> Vec<Tokens> {
    let mut multiplication_idx: Vec<usize> = vec![];

    for (idx, c) in arg.chars().enumerate() {
        // before numbers and after numbers a multiplication operation is added
        // idx is remembered for slicing
        if is_num(c) {
            // add multiplication before num if there is no part of the num
            if idx != 0 && !is_num(arg.chars().nth(idx - 1).unwrap()) {
                multiplication_idx.push(idx);
            }
            // add multiplication after num if there is no part of the num
            if idx != arg.len() - 1 && !is_num(arg.chars().nth(idx + 1).unwrap()) {
                multiplication_idx.push(idx + 1);
            }
        }
    }

    let mut arg_with_mult = arg.to_string();

    // adding multiplication operations
    for idx in multiplication_idx.iter().rev() {
        arg_with_mult.insert_str(*idx, " * ");
    }

    // splitting arg at slice indeces and tokenize the slices and append them to the token list
    match tokenize(
        arg_with_mult
            .split(" ")
            .filter(|slice| (**slice).ne(""))
            .collect(),
    ) {
        Ok(token_list) => token_list,
        Err(err) => panic!("{}", format!("This should never happen: {:?}", err)),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
        assert!(is_num('3'));
        assert!(is_num('.'));
        assert!(is_num(','));
        assert!(!is_num('a'));
        assert!(!is_num('$'));
    }

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol(")"));
        assert!(is_symbol("("));
    }

    #[test]
    fn test_tokenize_as_num() {
        assert_eq!(tokenize_as_num("3"), Tokens::Number(3.0));
        assert_eq!(tokenize_as_num("3.14"), Tokens::Number(3.14));
        assert_eq!(tokenize_as_num("3,14"), Tokens::Number(3.14));
    }

    #[test]
    fn test_tokenize_as_symbol() {
        assert_eq!(
            tokenize_as_symbol("("),
            vec![
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Symbol(SymbolTypes::OpeningBracket)
            ]
        );
        assert_eq!(
            tokenize_as_symbol(")"),
            vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Multiplication)
            ]
        );
    }

    #[test]
    fn test_tokenize_as_var() {
        assert_eq!(
            tokenize_as_var("var"),
            Tokens::Variable(Variable {
                name: "var".to_string(),
                value: None
            })
        );
    }

    #[test]
    fn test_split_num_and_str() {
        assert_eq!(
            split_num_and_str("var"),
            vec![Tokens::Variable(Variable {
                name: "var".to_string(),
                value: None
            })]
        );
        assert_eq!(
            split_num_and_str("76var1a6,1b5.01"),
            vec![
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
            ]
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
            assert!(is_func(func.1));
        }
    }

    #[test]
    fn test_tokenize_as_func() {
        for func in FUNCTIONS {
            assert_eq!(tokenize_as_func(func.1), Tokens::Function(func.0.clone()));
        }
    }

    #[test]
    fn test_interpret_str_wo_whitespaces() {
        assert_eq!(
            interpret_string_wo_withespaces("5-"),
            Ok(vec![
                Tokens::Number(5.0),
                Tokens::Function(FunctionTypes::Subtraction),
            ])
        );
        assert_eq!(
            interpret_string_wo_withespaces("der7*9+6"),
            Ok(vec![
                Tokens::Function(FunctionTypes::Derivative),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(7.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(9.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0),
            ])
        );
        assert_eq!(
            interpret_string_wo_withespaces("7der+6,0*4(3+2)der"),
            Ok(vec![
                Tokens::Number(7.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Function(FunctionTypes::Derivative),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(6.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(4.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Symbol(SymbolTypes::OpeningBracket),
                Tokens::Number(3.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(2.0),
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Function(FunctionTypes::Derivative),
            ])
        );
        assert_eq!(
            interpret_string_wo_withespaces("7&"),
            Err(ErrorTypes::ParserError(
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
    fn test_add_multiplications() {
        assert_eq!(
            add_multiplications(&mut vec![Tokens::Number(6.0), Tokens::Number(6.0),]),
            vec![
                Tokens::Number(6.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(6.0),
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Number(6.0),
                Tokens::Function(FunctionTypes::Derivative)
            ]),
            vec![
                Tokens::Number(6.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Function(FunctionTypes::Derivative)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Number(6.0),
                Tokens::Symbol(SymbolTypes::OpeningBracket)
            ]),
            vec![
                Tokens::Number(6.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Symbol(SymbolTypes::OpeningBracket)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Number(6.0),
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]),
            vec![
                Tokens::Number(6.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Number(6.0),
            ]),
            vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(6.0),
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Derivative)
            ]),
            vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Function(FunctionTypes::Derivative)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Symbol(SymbolTypes::OpeningBracket)
            ]),
            vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Symbol(SymbolTypes::OpeningBracket)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]),
            vec![
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Number(6.0),
            ]),
            vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(6.0),
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Derivative)
            ]),
            vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Function(FunctionTypes::Derivative)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Symbol(SymbolTypes::OpeningBracket)
            ]),
            vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Symbol(SymbolTypes::OpeningBracket)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]),
            vec![
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]
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
            parse_input("5(3+4a)5".to_string()),
            Ok(vec![
                Tokens::Number(5.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Symbol(SymbolTypes::OpeningBracket),
                Tokens::Number(3.0),
                Tokens::Function(FunctionTypes::Addition),
                Tokens::Number(4.0),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Variable(Variable {
                    name: "a".to_string(),
                    value: None
                }),
                Tokens::Symbol(SymbolTypes::ClosingBracket),
                Tokens::Function(FunctionTypes::Multiplication),
                Tokens::Number(5.0)
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
