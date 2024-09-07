use std::fmt::{Debug, Display, Formatter};
use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    source: String,
    message: String,
}

impl Error {
    pub fn other(message: &str) -> Error {
        Error::new("Other", message)
    }
    pub fn new(source: &str, message: &str) -> Self {
        Self { source: source.to_string(), message: message.to_string() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.message, self.source)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            source: String::from("std::io"),
            message: err.to_string(),
        }
    }
}
