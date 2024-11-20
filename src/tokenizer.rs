use core::fmt::Display;
use phf_macros::phf_map;
use std::vec;

use crate::error::*;

use crate::functions::{
    ElementaryFunc, Func, ELEMENTARY_FUNC_KEYWORDS, HIGHER_ORDER_FUNC_KEYWORDS,
};

#[derive(Debug, PartialEq, Clone)]
struct Variable {
    name: String,
    value: Option<isize>,
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Some(value) => write!(f, "{} = {}", self.name, value),
            None => write!(f, "{} = undefined", self.name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Func(Func),
    Number(f64),
    Variable(Variable),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq, Clone)]
enum Symbol {
    OpeningBracket,
    ClosingBracket,
}

const SYMBOLS: phf::Map<&'static str, Symbol> = phf_map! {
    "(" => Symbol::OpeningBracket,
    ")" => Symbol::ClosingBracket
};

enum CharOrStr<'a> {
    Char(char),
    Str(&'a str),
}

impl<'a> From<char> for CharOrStr<'a> {
    fn from(c: char) -> Self {
        CharOrStr::Char(c)
    }
}

impl<'a> From<&'a str> for CharOrStr<'a> {
    fn from(s: &'a str) -> Self {
        CharOrStr::Str(s)
    }
}

pub fn tokenize_input(input: String) -> Result<Vec<Token>, Error> {
    let splitted_input: Vec<&str> = input.split(' ').filter(|slice| (**slice).ne("")).collect();

    let tokenized_input = tokenize(splitted_input);

    if tokenized_input.is_err() {
        return tokenized_input;
    }

    let token_list = add_multiplications(&mut tokenized_input.unwrap());

    return Ok(token_list);
}

fn tokenize(args: Vec<&str>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = vec![];

    for arg in args {
        match arg {
            // try tokenizing as function
            _ if is_func(arg) => tokens.push(tokenize_as_func(arg)),
            // try tokenizing as number
            _ if is_num(arg) => tokens.push(tokenize_as_num(arg)),
            // try tokenizing as var
            _ if is_alphabetic(arg) => tokens.push(tokenize_as_var(arg)),
            // try tokenizing as symbol
            _ if is_symbol(arg) => tokens.push(tokenize_as_symbol(arg)),
            // split numbers and chars
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
/// 5 Der -> 5 * Der
/// 5 (5 + 5) 5 -> 5 * (5 + 5) * 5
/// () * () -> () * ()
fn add_multiplications(tokens: &mut Vec<Token>) -> Vec<Token> {
    let mut multiplication_idx: Vec<usize> = vec![];

    for (idx, token) in tokens.iter().enumerate() {
        if idx + 1 != tokens.len() {
            match token {
                Token::Number(_) | Token::Symbol(Symbol::ClosingBracket) | Token::Variable(_) => {
                    match tokens.get(idx + 1).unwrap() {
                        Token::Func(Func::HigherOrder(_))
                        | Token::Number(_)
                        | Token::Symbol(Symbol::OpeningBracket)
                        | Token::Variable(_) => multiplication_idx.push(idx + 1),
                        _ => (),
                    };
                }
                _ => (),
            };
        }
    }

    for idx in multiplication_idx.iter().rev() {
        tokens.insert(
            *idx,
            Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
        );
    }

    tokens.clone()
}

fn interpret_string_wo_withespaces(args: &str) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = vec![];
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
                return Err(Error::new(
                    format!("Tokenizer could not tokenize input: {}", args),
                    ErrorType::Tokenizer(TokenizerError::InvalidInput),
                    None,
                    None,
                ));
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
    SYMBOLS.contains_key(arg)
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
    match ELEMENTARY_FUNC_KEYWORDS.get(&arg.chars().nth(0).unwrap()) {
        Some(_) => true,
        None => match HIGHER_ORDER_FUNC_KEYWORDS.get(arg) {
            Some(_) => true,
            None => false,
        },
    }
}

fn tokenize_as_num(arg: &str) -> Token {
    Token::Number(
        arg.replace(",", ".")
            .parse::<f64>()
            .expect("This should be tokenized because it is checked before to be a number."),
    )
}

fn tokenize_as_var(arg: &str) -> Token {
    Token::Variable(Variable {
        name: arg.to_string(),
        value: None,
    })
}

fn tokenize_as_func(arg: &str) -> Token {
    // search keyword in function list
    match ELEMENTARY_FUNC_KEYWORDS.get(&arg.chars().nth(0).unwrap()) {
        Some(func) => Token::Func(Func::Elementary(func.clone())),
        None => match HIGHER_ORDER_FUNC_KEYWORDS.get(arg) {
            Some(func) => Token::Func(Func::HigherOrder(func.clone())),
            None => panic!(
                "This should not happen, because the keyword was checked to be a function before."
            ),
        },
    }
}

fn tokenize_as_symbol(arg: &str) -> Token {
    match SYMBOLS.get(arg).expect(
        format!("This ({arg}) should be a valid symbol because it is checked before to be valid.")
            .as_str(),
    ) {
        Symbol::OpeningBracket => Token::Symbol(Symbol::OpeningBracket),
        Symbol::ClosingBracket => Token::Symbol(Symbol::ClosingBracket),
    }
}

fn split_num_and_str(arg: &str) -> Vec<Token> {
    let mut split_idx: Vec<usize> = vec![];

    for (idx, c) in arg.chars().enumerate() {
        // idx is remembered for slicing
        if is_num(c) {
            // add whitespace before num if there is no part of the num
            if idx != 0 && !is_num(arg.chars().nth(idx - 1).unwrap()) {
                split_idx.push(idx);
            }
            // add whitespace after num if there is no part of the num
            if idx != arg.len() - 1 && !is_num(arg.chars().nth(idx + 1).unwrap()) {
                split_idx.push(idx + 1);
            }
        }
    }

    let mut arg_with_mult = arg.to_string();

    // split string at determined indeces
    for idx in split_idx.iter().rev() {
        arg_with_mult.insert_str(*idx, " ");
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
    use crate::functions::*;

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
        assert_eq!(tokenize_as_num("3"), Token::Number(3.0));
        assert_eq!(tokenize_as_num("3.14"), Token::Number(3.14));
        assert_eq!(tokenize_as_num("3,14"), Token::Number(3.14));
    }

    #[test]
    fn test_tokenize_as_symbol() {
        assert_eq!(
            tokenize_as_symbol("("),
            Token::Symbol(Symbol::OpeningBracket)
        );
        assert_eq!(
            tokenize_as_symbol(")"),
            Token::Symbol(Symbol::ClosingBracket),
        );
    }

    #[test]
    fn test_tokenize_as_var() {
        assert_eq!(
            tokenize_as_var("var"),
            Token::Variable(Variable {
                name: "var".to_string(),
                value: None
            })
        );
    }

    #[test]
    fn test_split_num_and_str() {
        assert_eq!(
            split_num_and_str("var"),
            vec![Token::Variable(Variable {
                name: "var".to_string(),
                value: None
            })]
        );
        assert_eq!(
            split_num_and_str("76var1a6,1b5.01"),
            vec![
                Token::Number(76.0),
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Number(1.0),
                Token::Variable(Variable {
                    name: "a".to_string(),
                    value: None
                }),
                Token::Number(6.1),
                Token::Variable(Variable {
                    name: "b".to_string(),
                    value: None
                }),
                Token::Number(5.01),
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
        for (keyword, _) in ELEMENTARY_FUNC_KEYWORDS.into_iter() {
            assert!(is_func(keyword.to_string().as_str()));
        }
        for (keyword, _) in HIGHER_ORDER_FUNC_KEYWORDS.into_iter() {
            assert!(is_func(keyword));
        }
    }

    #[test]
    fn test_tokenize_as_func() {
        for (keyword, func) in ELEMENTARY_FUNC_KEYWORDS.into_iter() {
            assert_eq!(
                tokenize_as_func(keyword.to_string().as_str()),
                Token::Func(Func::Elementary(func.clone()))
            );
        }
        for (keyword, func) in HIGHER_ORDER_FUNC_KEYWORDS.into_iter() {
            assert_eq!(
                tokenize_as_func(keyword),
                Token::Func(Func::HigherOrder(func.clone()))
            );
        }
    }

    #[test]
    fn test_interpret_str_wo_whitespaces() {
        assert_eq!(
            interpret_string_wo_withespaces("5-"),
            Ok(vec![
                Token::Number(5.0),
                Token::Func(Func::Elementary(ElementaryFunc::Subtraction)),
            ])
        );
        assert_eq!(
            interpret_string_wo_withespaces("Der7*9+6"),
            Ok(vec![
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative)),
                Token::Number(7.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(9.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(6.0),
            ])
        );
        assert_eq!(
            interpret_string_wo_withespaces("7Der+6,0*4(3+2)Der"),
            Ok(vec![
                Token::Number(7.0),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative)),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(6.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(4.0),
                Token::Symbol(Symbol::OpeningBracket),
                Token::Number(3.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(2.0),
                Token::Symbol(Symbol::ClosingBracket),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative)),
            ])
        );
        assert_eq!(
            interpret_string_wo_withespaces("7&"),
            Err(Error::new(
                "Tokenizer could not tokenize input: 7&".to_string(),
                ErrorType::Tokenizer(TokenizerError::InvalidInput),
                None,
                None
            ))
        )
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(vec!["5", "+", "6"]),
            Ok(vec![
                Token::Number(5.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(6.0)
            ])
        );
        assert_eq!(
            tokenize(vec!["Der", "7", "*", "9", "+", "6"]),
            Ok(vec![
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative)),
                Token::Number(7.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(9.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(6.0)
            ])
        );

        assert_eq!(
            tokenize(vec!["/"]),
            Ok(vec![Token::Func(Func::Elementary(
                ElementaryFunc::Division
            )),])
        );
        assert_eq!(
            tokenize(vec!["var"]),
            Ok(vec![Token::Variable(Variable {
                name: "var".to_string(),
                value: None
            })])
        );
        assert_eq!(
            tokenize(vec!["var1"]),
            Ok(vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Number(1.0)
            ])
        );
        assert_eq!(
            tokenize(vec!["var$"]),
            Err(Error::new(
                "Tokenizer could not tokenize input: var$".to_string(),
                ErrorType::Tokenizer(TokenizerError::InvalidInput),
                None,
                None
            ))
        );
    }

    #[test]
    fn test_add_multiplications() {
        assert_eq!(
            add_multiplications(&mut vec![Token::Number(6.0), Token::Number(6.0),]),
            vec![
                Token::Number(6.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(6.0),
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Number(6.0),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative))
            ]),
            vec![
                Token::Number(6.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative))
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Number(6.0),
                Token::Symbol(Symbol::OpeningBracket)
            ]),
            vec![
                Token::Number(6.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Symbol(Symbol::OpeningBracket)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Number(6.0),
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]),
            vec![
                Token::Number(6.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Number(6.0),
            ]),
            vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(6.0),
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative))
            ]),
            vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative))
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Symbol(Symbol::OpeningBracket)
            ]),
            vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Symbol(Symbol::OpeningBracket)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]),
            vec![
                Token::Symbol(Symbol::ClosingBracket),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Number(6.0),
            ]),
            vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(6.0),
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative))
            ]),
            vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative))
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Symbol(Symbol::OpeningBracket)
            ]),
            vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Symbol(Symbol::OpeningBracket)
            ]
        );
        assert_eq!(
            add_multiplications(&mut vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]),
            vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                })
            ]
        );
    }

    #[test]
    fn test_tokenize_input() {
        assert_eq!(
            tokenize_input("5 +    6".to_string()),
            Ok(vec![
                Token::Number(5.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(6.0)
            ])
        );
        assert_eq!(
            tokenize_input("5 + 6".to_string()),
            Ok(vec![
                Token::Number(5.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(6.0)
            ])
        );
        assert_eq!(
            tokenize_input("Der 7 * 9 + 6".to_string()),
            Ok(vec![
                Token::Func(Func::HigherOrder(HigherOrderFunc::Derivative)),
                Token::Number(7.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(9.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(6.0)
            ])
        );

        assert_eq!(
            tokenize_input("/".to_string()),
            Ok(vec![Token::Func(Func::Elementary(
                ElementaryFunc::Division
            )),])
        );
        assert_eq!(
            tokenize_input("var".to_string()),
            Ok(vec![Token::Variable(Variable {
                name: "var".to_string(),
                value: None
            })])
        );
        assert_eq!(
            tokenize_input("var1".to_string()),
            Ok(vec![
                Token::Variable(Variable {
                    name: "var".to_string(),
                    value: None
                }),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(1.0)
            ])
        );

        assert_eq!(
            tokenize_input("5(3+4a)5".to_string()),
            Ok(vec![
                Token::Number(5.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Symbol(Symbol::OpeningBracket),
                Token::Number(3.0),
                Token::Func(Func::Elementary(ElementaryFunc::Addition)),
                Token::Number(4.0),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Variable(Variable {
                    name: "a".to_string(),
                    value: None
                }),
                Token::Symbol(Symbol::ClosingBracket),
                Token::Func(Func::Elementary(ElementaryFunc::Multiplication)),
                Token::Number(5.0)
            ])
        );

        assert_eq!(
            tokenize_input("var$".to_string()),
            Err(Error::new(
                "Tokenizer could not tokenize input: var$".to_string(),
                ErrorType::Tokenizer(TokenizerError::InvalidInput),
                None,
                None
            ))
        );
    }
}
