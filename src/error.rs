pub fn handle_errors(errors: Vec<Error>) {
    errors.iter().for_each(|err| println!("{}", err.error));
}

#[derive(Debug)]
pub struct Error {
    pub error: String,
    pub error_type: Box<dyn std::error::Error>,
    pub token_start_idx: Option<usize>,
    pub curr_idx: Option<usize>,
}

impl Error {
    pub fn new(
        error: String,
        error_type: Box<dyn std::error::Error>,
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

#[derive(Debug)]
pub enum IoError {
    InvalidUTF8,
}

#[derive(Debug)]
pub enum TokenizerError {
    UnrecognizedInput,
    InvalidFunctionName,
    InvalidSymbol,
    InvalidNumber,
    EmptyToken,
}

#[derive(Debug)]
pub enum ParserError {
    ParserError,
}

#[derive(Debug)]
pub enum FunctionError {
    FactorialError,
    DivisionByZero,
    OverflowInf,
    UnderflowInf,
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
