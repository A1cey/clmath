use std::io;

pub fn handle_error(err: Error) {
    println!("There was a problem: {:?}", err.error);
}

#[derive(Debug, PartialEq)]
pub struct Error {
    error: String,
    error_type: ErrorType,
    token_start_idx: Option<usize>,
    curr_idx: Option<usize>,
}

impl Error {
    pub fn new(
        error: String,
        error_type: ErrorType,
        token_start_idx: Option<usize>,
        curr_idx: Option<usize>,
    ) -> Error {
        Error {
            error,
            error_type,
            token_start_idx,
            curr_idx,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    IoError(IoError),
    Tokenizer(TokenizerError),
    Parser(ParserError),
    Func(FunctionError),
}

#[derive(Debug, PartialEq)]
pub enum IoError {
    InvalidUTF8,
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    InvalidInputIndexing,
    InvalidInput,
    InvalidFunctionName,
    InvalidSymbol,
    InvalidNumber,
    EmptyToken,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ParserError,
}

#[derive(Debug, PartialEq)]
pub enum FunctionError {
    FactorialError,
    DivisionByZero,
    OverflowInf,
    UnderflowInf,
}
