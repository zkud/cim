use std::collections::HashMap;
use std::error;
use std::fmt;

pub trait TagParser: Iterator<Item = Result<TagEvent, TagError>> {}

pub enum TagEvent {
  Open {
    tag: Tag,
    attributes: HashMap<String, String>,
  },
  Close {
    tag: Tag,
  },
}

pub enum Tag {
  Schema,
  EntityType,
  Property,
  NavigationProperty,
  PropertyRef,
  Unknown
}

#[derive(fmt::Debug, Clone, Hash, PartialEq, Eq)]
pub struct TagError {
  message: String,
}

impl TagError {
  pub fn new<M: AsRef<str>>(message: M) -> TagError {
    TagError {
      message: message.as_ref().to_string(),
    }
  }
}

impl fmt::Display for TagError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "Tag Parser Backend Error, reason: {}",
      self.message
    )
  }
}

impl error::Error for TagError {}
