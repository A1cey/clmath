pub fn handle_error(err: Error) {
    println!("There was a problem: {:?}", err.error);
}

#[derive(Debug, PartialEq)]
pub struct Error {
    error: String,
    error_type: ErrorType,
    idx: Option<usize>,
}

impl Error {
    pub fn new(error: String, error_type: ErrorType, idx: Option<usize>) -> Error {
        Error {
            error,
            error_type,
            idx,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    IoError,
    Tokenizer(TokenizerError),
    Parser(ParserError),
    Func(FunctionError),
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    InvalidInputIndexing,
    InvalidInput,
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
