use crate::types::ErrorType;

pub fn handle_error(err: ErrorType) {
    println!("There was a problem: {:?}", err);
}
