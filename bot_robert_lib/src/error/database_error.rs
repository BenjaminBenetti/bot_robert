use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct DatabaseError {
    pub message: String,
}

impl DatabaseError {
    pub fn new(message: &str) -> DatabaseError {
        DatabaseError {
            message: String::from(message),
        }
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for DatabaseError {

}