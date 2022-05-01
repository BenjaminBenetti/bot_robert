use std::fmt::{Formatter, Display};
use std::fmt;

#[derive(Debug)]
pub struct JsonDecodeError {
    pub message: String,
}

impl JsonDecodeError {
    pub fn new<S: Into<String>>(message: S) -> JsonDecodeError {
        JsonDecodeError {
            message: message.into(),
        }
    }
}

impl Display for JsonDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for JsonDecodeError {

}