pub fn handle_error(err: ErrorTypes) {
    println!("There was a problem: {:?}", err);
}

#[derive(Debug)]
pub enum ErrorTypes {
    IoError(std::io::Error),
    ParserError(String),
}
