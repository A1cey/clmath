use core::fmt::Display;
use core::panic;
use phf_macros::phf_map;

use crate::error::{CLMathError, TokenizerError, TokenizerErrorType};

use crate::functions::{
    ElementaryFunc, Func, ELEMENTARY_FUNC_KEYWORDS, HIGHER_ORDER_FUNC_KEYWORDS,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Option<f64>,
}

impl Variable {
    pub fn new(name: String, value: Option<f64>) -> Variable {
        Variable { name, value }
    }
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
    Function(Func),
    Number(f64),
    Variable(Variable),
    Symbol(Symbol),
    Empty,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    HigherOrderFunc,
    ElementaryFunc,
    Number,
    Variable,
    Symbol,
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    OpeningBracket,
    ClosingBracket,
    Comma,
}

const SYMBOLS: phf::Map<char, Symbol> = phf_map! {
    '(' => Symbol::OpeningBracket,
    ')' => Symbol::ClosingBracket,
    ',' => Symbol::Comma
};

struct Tokenizer {
    curr_idx: usize,
    token_start_idx: usize,
    input: String,
    input_len: usize,
    tokens: Vec<Token>,
    curr_token_type: TokenType,
    is_done: bool,
    errors: Vec<TokenizerError>,
}

impl Tokenizer {
    fn from(input: String) -> Self {
        Tokenizer {
            curr_idx: 0,
            token_start_idx: 0,
            input_len: input.len(),
            is_done: input.is_empty(),
            input,
            tokens: Vec::new(),
            curr_token_type: TokenType::Empty,
            errors: Vec::new(),
        }
    }

    fn step(&mut self) {
        self.curr_idx += 1;

        if self.input_len == self.curr_idx {
            self.is_done = true;
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.curr_idx + 1)
    }

    fn consume(&mut self) {
        let token = if self.token_start_idx > self.curr_idx || self.curr_idx >= self.input_len {
            self.tokenize_empty()
        } else {
            let token_value = self
                .input
                .get(self.token_start_idx..=self.curr_idx)
                .unwrap()
                .to_string();

            match self.curr_token_type {
                TokenType::Empty => self.tokenize_empty(),
                TokenType::Number => self.tokenize_number(&token_value),
                TokenType::Variable => self.tokenize_variable(&token_value),
                TokenType::ElementaryFunc | TokenType::HigherOrderFunc => {
                    self.tokenize_function(&token_value)
                }
                TokenType::Symbol => {
                    if token_value.len() > 1 {
                        panic!("Symbols can only be one char. This function should not be called with a string of more than one char.")
                    }
                    self.tokenize_symbol(&token_value.chars().nth(0).unwrap())
                }
            }
        };

        self.step();
        self.token_start_idx = self.curr_idx;

        self.tokens.push(token);
    }

    fn tokenize_empty(&mut self) -> Token {
        self.add_error(TokenizerErrorType::EmptyToken, None, None);
        Token::Empty
    }

    fn tokenize_number(&mut self, token_value: &str) -> Token {
        Token::Number(token_value.parse::<f64>().unwrap_or_else(|err| {
            self.add_error(
                TokenizerErrorType::InvalidNumber,
                Some(token_value),
                Some(err.to_string()),
            );
            f64::NAN
        }))
    }

    fn tokenize_variable(&self, token_value: &str) -> Token {
        Token::Variable(Variable::new(token_value.to_string(), None))
    }

    fn tokenize_function(&mut self, token_value: &str) -> Token {
        match self.curr_token_type {
            TokenType::ElementaryFunc => {
                if token_value.len() > 1 {
                    panic!("Elementary functions can only be one char. This function should not be called with a string of more than one char if the current token type is 'ElementaryFunc'.")
                }

                ELEMENTARY_FUNC_KEYWORDS.get(&token_value.chars().nth(0).unwrap()).map_or_else(
                || {
                    self.add_error(TokenizerErrorType::InvalidFunctionName, Some(token_value), None);
                    Token::Empty
                },
                |func| Token::Function(Func::Elementary(func.clone())),
            )},

            TokenType::HigherOrderFunc => HIGHER_ORDER_FUNC_KEYWORDS.get(token_value).map_or_else(
                || {
                    self.add_error(TokenizerErrorType::InvalidFunctionName, Some(token_value), None);
                    Token::Empty
                },
                |func| Token::Function(Func::HigherOrder(func.clone())),
            ),
            _ => panic!("This function should not be called, when the token type is not elementary or higher order function.")
        }
    }

    fn tokenize_symbol(&mut self, token_value: &char) -> Token {
        if let Some(symbol) = SYMBOLS.get(token_value) {
            match symbol {
                Symbol::OpeningBracket => Token::Symbol(Symbol::OpeningBracket),
                Symbol::ClosingBracket => Token::Symbol(Symbol::ClosingBracket),
                Symbol::Comma => Token::Symbol(Symbol::Comma),
            }
        } else {
            self.add_error(
                TokenizerErrorType::InvalidSymbol,
                Some(token_value.to_string().as_str()),
                None,
            );
            Token::Empty
        }
    }

    fn add_error(
        &mut self,
        error_type: TokenizerErrorType,
        token_value: Option<&str>,
        err_value: Option<String>,
    ) {
        let error = match error_type {
            TokenizerErrorType::UnrecognizedInput => {
                format!(
                    "Input '{}' was not recognized. '{}' is not a valid symbol.",
                    self.input,
                    token_value.unwrap()
                )
            }

            TokenizerErrorType::EmptyToken => "Tried to tokenize an empty token.".to_string(),
            TokenizerErrorType::InvalidFunctionName => {
                format!("'{}' is an invalid function name.", token_value.unwrap())
            }
            TokenizerErrorType::InvalidSymbol => {
                format!("'{}' is an invalid symbol.", token_value.unwrap())
            }
            TokenizerErrorType::InvalidNumber => {
                format!(
                    "'{}' is an invalid number, because:\n {}",
                    token_value.unwrap(),
                    err_value.unwrap()
                )
            }
        };

        self.errors.push(TokenizerError::new(
            error,
            error_type,
            self.token_start_idx,
            self.curr_idx,
        ))
    }

    fn get_char(&self) -> char {
        match self.input.chars().nth(self.curr_idx) {
            Some(c) => c,
            None => panic!("The index should not be greater or equal to the length of the input. This should never happen.")
        }
    }

    fn remove_empty_tokens(&mut self) {
        self.tokens.retain(|token| *token != Token::Empty);
    }

    fn is_symbol(c: &char) -> bool {
        SYMBOLS.contains_key(c)
    }

    fn is_elementary_function(c: &char) -> bool {
        ELEMENTARY_FUNC_KEYWORDS.contains_key(c)
    }

    fn is_number(c: &char) -> bool {
        c.is_numeric() || *c == '.' 
    }

    fn add_multiplications(&mut self) {
        let mut multiplication_idx: Vec<usize> = vec![];

        for (idx, token) in self.tokens.iter().enumerate() {
            if idx + 1 != self.tokens.len() {
                match token {
                    Token::Number(_)
                    | Token::Symbol(Symbol::ClosingBracket)
                    | Token::Variable(_) => {
                        match self.tokens.get(idx + 1).unwrap() {
                            Token::Function(Func::HigherOrder(_))
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
            self.tokens.insert(
                *idx,
                Token::Function(Func::Elementary(ElementaryFunc::Multiplication)),
            );
        }
    }

    fn run(&mut self) {
        while !self.is_done {
            match self.get_char() {
                c if c.is_whitespace() => self.step(),
                c if Tokenizer::is_symbol(&c) => {
                    self.curr_token_type = TokenType::Symbol;
                    self.consume();
                }
                c if Tokenizer::is_elementary_function(&c) => {
                    self.curr_token_type = TokenType::ElementaryFunc;
                    self.consume();
                }
                c if c.is_alphabetic() => {
                    if c.is_uppercase() {
                        self.curr_token_type = TokenType::HigherOrderFunc;
                    } else {
                        self.curr_token_type = TokenType::Variable;
                    }

                    while let Some(x) = self.peek() {
                        if !x.is_alphabetic() {
                            break;
                        }

                        self.step();
                    }
                    self.consume();
                }
                c if c.is_numeric() => {
                    self.curr_token_type = TokenType::Number;
                    while let Some(x) = self.peek() {
                        if !Tokenizer::is_number(&x) {
                            break;
                        }
                        self.step();
                    }

                    self.consume();
                }
                c => {
                    self.add_error(
                        TokenizerErrorType::UnrecognizedInput,
                        Some(c.to_string().as_str()),
                        None,
                    );
                    self.curr_token_type = TokenType::Empty;
                    self.consume();
                }
            }
        }

        self.remove_empty_tokens();
        self.add_multiplications();
    }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, Vec<CLMathError>> {
    let mut tokenizer = Tokenizer::from(input);

    tokenizer.run();

    if !tokenizer.errors.is_empty() {
        return Err(tokenizer
            .errors
            .into_iter()
            .map(CLMathError::Tokenizer)
            .collect());
    }

    Ok(tokenizer.tokens)
}
