use super::types::Tag;
use super::types::TagError;
use super::types::TagEvent;
use super::types::TagParser;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, Events, XmlEvent};

pub struct XmlTagParser {
  tag_parser: Events<BufReader<File>>,
}

impl Iterator for XmlTagParser {
  type Item = Result<TagEvent, TagError>;

  fn next(&mut self) -> Option<Result<TagEvent, TagError>> {
    loop {
      match self.tag_parser.next() {
        Some(Ok(XmlEvent::StartElement {
          name, attributes, ..
        })) => return Some(Ok(Self::build_open_tag_event(name.local_name, &attributes))),
        Some(Ok(XmlEvent::EndElement { name })) => {
          return Some(Ok(Self::build_close_tag_event(name.local_name)))
        }
        Some(Err(e)) => return Some(Err(TagError::new(e.msg()))),
        None => return None,
        _ => continue,
      }
    }
  }
}

impl TagParser for XmlTagParser {}

impl XmlTagParser {
  pub fn new(path: String) -> Self {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let tag_parser = EventReader::new(file);
    let tag_parser = tag_parser.into_iter();
    XmlTagParser { tag_parser }
  }

  fn build_open_tag_event(name: String, attributes: &Vec<OwnedAttribute>) -> TagEvent {
    let tag = Self::build_tag(name);
    let attributes = Self::parse_attributes(attributes);
    TagEvent::Open {
      tag,
      attributes,
    }
  }

  fn parse_attributes(attributes: &Vec<OwnedAttribute>) -> HashMap<String, String> {
    let mut attributes_map = HashMap::new();
    for attribute in attributes.iter() {
      let key = attribute.name.local_name.clone();
      let value = attribute.value.clone();
      attributes_map.insert(key, value);
    }
    attributes_map
  }

  fn build_close_tag_event(name: String) -> TagEvent {
    let tag = Self::build_tag(name);
    TagEvent::Close { tag }
  }

  fn build_tag(name: String) -> Tag {
    match name.as_str() {
      "Schema" => Tag::Schema,
      "EntityType" => Tag::EntityType,
      "Property" => Tag::Property,
      "NavigationProperty" => Tag::NavigationProperty,
      "PropertyRef" => Tag::PropertyRef,
      _ => Tag::Unknown,
    }
  }
}
