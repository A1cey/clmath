use std::{error::Error, fmt::Debug};

pub fn handle_errors(errors: Vec<impl Error>) {
    errors.iter().for_each(|err| println!("{:?}", err));
}

pub fn handle_error(error: impl Error) {
    println!("{:?}", error);
}

#[derive(Debug)]
pub enum IoError {
    InvalidUTF8(String),
}

#[derive(Debug)]
pub enum TokenizerErrorType {
    UnrecognizedInput,
    InvalidFunctionName,
    InvalidSymbol,
    InvalidNumber,
    EmptyToken,
}

#[derive(Debug)]
pub struct TokenizerError {
    pub error: String,
    pub error_type: TokenizerErrorType,
    pub token_start_idx: usize,
    pub curr_idx: usize,
}

impl TokenizerError {
    pub fn new(
        error: String,
        error_type: TokenizerErrorType,
        token_start_idx: usize,
        curr_idx: usize,
    ) -> Self {
        Self {
            error,
            error_type,
            token_start_idx,
            curr_idx,
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    ParserError,
}

#[derive(Debug)]
pub enum FunctionErrorType {
    FactorialError,
    DivisionByZero,
    OverflowInf,
    UnderflowInf,
}

#[derive(Debug)]
pub struct FunctionError {
    pub error: String,
    pub error_type: FunctionErrorType
}

impl FunctionError {
    pub fn new(error: String, error_type: FunctionErrorType) -> Self {
        Self {error, error_type}
    }
}


impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for IoError {}
impl std::error::Error for TokenizerError {}
impl std::error::Error for ParserError {}
impl std::error::Error for FunctionError {}
