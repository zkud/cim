use std::error;
use std::fmt;

#[derive(fmt::Debug, Clone, Hash, PartialEq, Eq)]
pub struct ParserError {
  message: String,
}

impl ParserError {
  pub fn new_boxed<M: AsRef<str>>(message: M) -> Box<ParserError> {
    Box::new(Self::new(message))
  }

  pub fn new<M: AsRef<str>>(message: M) -> ParserError {
    ParserError {
      message: message.as_ref().to_string(),
    }
  }
}

impl fmt::Display for ParserError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "Metadata Parser Error, reason: {}", self.message)
  }
}

impl error::Error for ParserError {}
