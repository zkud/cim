use std::collections::HashMap;
use std::error;
use std::fmt;

#[cfg(not(tarpaulin_include))]
pub trait TagParser: Iterator<Item = Result<TagEvent, TagError>> {}

#[cfg(not(tarpaulin_include))]
pub enum TagEvent {
  Open {
    tag: Tag,
    attributes: HashMap<String, String>,
  },
  Close {
    tag: Tag,
  },
}

#[cfg(not(tarpaulin_include))]
pub enum Tag {
  Schema,
  EntityType,
  Property,
  NavigationProperty,
  PropertyRef,
}

#[cfg(not(tarpaulin_include))]
#[derive(fmt::Debug, Clone, Hash, PartialEq, Eq)]
pub struct TagError {
  message: String,
}

#[cfg(not(tarpaulin_include))]
impl TagError {
  pub fn new<M: AsRef<str>>(message: M) -> TagError {
    TagError {
      message: message.as_ref().to_string(),
    }
  }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for TagError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "Tag Parser Backend Error, reason: {}",
      self.message
    )
  }
}

#[cfg(not(tarpaulin_include))]
impl error::Error for TagError {}
