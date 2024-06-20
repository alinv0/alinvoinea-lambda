use std::fmt;

#[derive(Debug, Clone)]
pub struct NotYetImplementedError;

const NOT_YET_IMPLEMENTED_MESSAGE: &str = "Operation not yet implemented!";

impl fmt::Display for NotYetImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", NOT_YET_IMPLEMENTED_MESSAGE)
    }
}

pub fn not_yet_implemented() -> String {
    let message = "Operation not yet implemented!".to_string();
    println!("{}", message);
    message
}