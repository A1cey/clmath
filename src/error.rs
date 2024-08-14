use crate::types::ErrorTypes;

pub fn handle_error(err: ErrorTypes) {
    println!("There was a problem: {:?}", err);
}
