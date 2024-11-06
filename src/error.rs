pub fn handle_error(err: Error) {
    println!("There was a problem: {:?}", err);
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IoError(String),
    ParserError(String),
    FactorialError(String),
    AdditionError(String)
}
