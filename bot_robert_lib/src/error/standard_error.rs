
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct StandardError {
    pub message: String,
}

impl StandardError {
    pub fn new(message: &str) -> StandardError {
        StandardError {
            message: String::from(message),
        }
    }
}

impl Display for StandardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for StandardError {

}