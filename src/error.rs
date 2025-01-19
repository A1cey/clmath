use std::fmt::Debug;

pub fn handle_errors(errors: Vec<CLMathError>) {
    errors.into_iter().for_each(handle_error);
}

pub fn handle_error(error: CLMathError) {
    match error {
        CLMathError::Function(error) => print_function_error(error),
        CLMathError::Tokenizer(error) => print_tokenizer_error(error),
        CLMathError::Parser(error) => print_parser_error(error),
        CLMathError::Io(error) => print_io_error(error),
    }
}

fn print_parser_error(error: ParserError) {
    match error {
        ParserError::ExpectedClosingBracket => println!(
            "An error occured while trying to evaluate the input: ExpectedClosingBracket\nA closing bracket was expected but not found.",
        ),
        ParserError::ExpectedMathExpression => println!(
            "An error occured while trying to evaluate the input: ExpectedMathExpression\nA math expression was expected but not found.",
        ),
        ParserError::ExpressionEmpty => println!(
            "An error occured while trying to evaluate the input: ExpressionEmpty\nThe expression was empty.",
        ),
        ParserError::ExpectedOpeningBracket => println!(
            "An error occured while trying to evaluate the input: ExpectedOpeningBracket\nAn opening bracket was expected but not found.",
        ),
        ParserError::ExpectedComma => println!(
            "An error occured while trying to evaluate the input: ExpectedComma\nA comma was expected but not found.",
        ),
        ParserError::NoLhsExpressionProvided => println!(
            "An error occured while trying to evaluate the input: NoLhsExpressionProvided\nNo left hand side expression for an elementary function was found.",
        ),
        ParserError::ExpectedElementaryFunction => println!(
            "An error occured while trying to evaluate the input: ExpectedElementaryFunction\nAn elementary function was expected but not found.",
        )
    }
}

fn print_io_error(error: IoError) {
    match error {
        IoError::InvalidUTF8(error) => println!(
            "An error occured while trying to read the input: InvalidUTF8\n{}",
            error
        ),
    }
}

fn print_tokenizer_error(error: TokenizerError) {
    println!(
        "An error occurred while trying evaluate the input: {:?}, Start: {}, End: {}\n{}",
        error.error_type, error.token_start_idx, error.curr_idx, error.error
    );
}

fn print_function_error(error: FunctionError) {
    println!(
        "An error occurred while trying run the input: {:?}\n{}",
        error.error_type, error.error,
    );
}

#[derive(Debug)]
pub enum CLMathError {
    Io(IoError),
    Tokenizer(TokenizerError),
    Parser(ParserError),
    Function(FunctionError),
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
    ExpectedMathExpression,
    ExpressionEmpty,
    ExpectedElementaryFunction,
    ExpectedOpeningBracket,
    ExpectedClosingBracket,
    ExpectedComma,
    NoLhsExpressionProvided,
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
    pub error_type: FunctionErrorType,
}

impl FunctionError {
    pub fn new(error: String, error_type: FunctionErrorType) -> Self {
        Self { error, error_type }
    }
}

impl std::fmt::Display for CLMathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
