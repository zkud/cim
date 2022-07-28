mod cds;
pub mod tag_parser;
mod util;

use cds::entity::Entity;
use cds::field::Field;
use std::collections::HashMap;
use tag_parser::types::Tag;
use tag_parser::types::TagEvent;
use tag_parser::types::TagParser;
use util::get_attribute;

pub struct Parser {
  finished_entities: Vec<Entity>,
  entity_name: String,
  fields: HashMap<String, Field>,
  keys: Vec<String>,
  field_name: String,
  field_type: String,
  associated_target: String,
  field_attributes: HashMap<String, String>,
  schema_name: String,
  tag_parser: Option<Box<dyn TagParser>>,
}

impl Parser {
  pub fn new(tag_parser: Box<dyn TagParser>) -> Parser {
    Parser {
      finished_entities: Vec::new(),
      entity_name: String::new(),
      fields: HashMap::new(),
      keys: Vec::new(),
      field_name: String::new(),
      field_type: String::new(),
      associated_target: String::new(),
      field_attributes: HashMap::new(),
      schema_name: String::new(),
      tag_parser: Some(tag_parser),
    }
  }

  pub fn parse(&mut self) -> String {
    if let Some(tag_parser) = self.tag_parser.take() {
      for e in tag_parser {
        match e {
          Ok(TagEvent::Open { tag, attributes }) => match tag {
            Tag::Schema => self.on_schema_start(&attributes),
            Tag::EntityType => self.on_entity_start(&attributes),
            Tag::Property => self.on_property_start(&attributes),
            Tag::NavigationProperty => self.on_navigation_property_start(&attributes),
            Tag::PropertyRef => self.on_property_ref(attributes),
          },
          Ok(TagEvent::Close { tag }) => match tag {
            Tag::EntityType => self.on_entity_close(),
            Tag::Property => self.on_property_close(),
            Tag::NavigationProperty => self.on_navigation_property_close(),
            _ => (),
          },
          Err(e) => {
            panic!("Error: {}", e);
          }
        }
      }
    }
    self.compose_cds_string()
  }

  fn on_schema_start(&mut self, attributes: &HashMap<String, String>) {
    self.schema_name =
      get_attribute(&attributes, "Namespace").expect("Failed to get schemas's namespace");
  }

  fn on_entity_start(&mut self, attributes: &HashMap<String, String>) {
    self.entity_name = get_attribute(&attributes, "Name").expect("Failed to get entity's name");
  }

  fn on_property_start(&mut self, attributes: &HashMap<String, String>) {
    self.field_name = get_attribute(&attributes, "Name").expect("Failed to get property's name");
    self.field_type = get_attribute(&attributes, "Type").expect("Failed to get property's type");
    self.field_attributes = attributes.clone();
  }

  fn on_navigation_property_start(&mut self, attributes: &HashMap<String, String>) {
    self.field_name =
      get_attribute(&attributes, "Name").expect("Failed to get nav. property's name");
    self.associated_target = get_attribute(&attributes, "Type")
      .or(get_attribute(&attributes, "ToRole"))
      .expect("Failed to get nav. property's target");
    if self.schema_name.len() > 0 {
      self.associated_target = self
        .associated_target
        .replace(&(self.schema_name.clone() + "."), "");
    }
    self.associated_target = self.associated_target.replace("Collection(", "");
    self.associated_target = self.associated_target.replace(")", "");
  }

  fn on_property_ref(&mut self, attributes: HashMap<String, String>) {
    let field_name = get_attribute(&attributes, "Name").expect("Failed to get property ref's name");
    self.keys.push(field_name);
  }

  fn on_entity_close(&mut self) {
    for key in self.keys.iter() {
      self
        .fields
        .get_mut(key)
        .expect("Unknown property in property ref")
        .set_as_key()
    }
    let entity_fields = self.fields.clone().into_values().collect();
    let entity = Entity::new(&self.entity_name, &entity_fields);
    self.keys.clear();
    self.finished_entities.push(entity);
    self.entity_name.clear();
    self.fields.clear();
  }

  fn on_property_close(&mut self) {
    let field = Field::from_odata(&self.field_name, &self.field_type, &self.field_attributes);
    self.fields.insert(self.field_name.clone(), field);
    self.field_name.clear();
    self.field_type.clear();
    self.field_attributes.clear();
  }

  fn on_navigation_property_close(&mut self) {
    let field = Field::new_association(&self.field_name, &self.associated_target);
    self.fields.insert(self.field_name.clone(), field);
    self.associated_target.clear();
    self.field_name.clear();
  }

  fn compose_cds_string(&self) -> String {
    let mut cds = String::from("");
    for entity in self.finished_entities.iter() {
      cds.push_str(&entity.to_cds());
    }
    cds
  }
}

#[cfg(test)]
mod tests {
  use super::tag_parser::types::Tag;
  use super::tag_parser::types::TagError;
  use super::tag_parser::types::TagEvent;
  use super::tag_parser::types::TagParser;
  use super::Parser;
  use std::collections::HashMap;

  macro_rules! open_tag {
    ($tag_type: expr, $attributes: expr) => {
      TagEvent::Open {
        tag: $tag_type,
        attributes: HashMap::from($attributes),
      }
    };
  }
  macro_rules! close_tag {
    ($tag_type: expr) => {
      TagEvent::Close { tag: $tag_type }
    };
  }

  #[test]
  fn with_usual_input_it_generates_valid_cds() {
    let tags = vec![
      open_tag!(
        Tag::Schema,
        [(String::from("Namespace"), String::from("test"))]
      ),
      open_tag!(
        Tag::EntityType,
        [(String::from("Name"), String::from("Tests"))]
      ),
      open_tag!(
        Tag::Property,
        [
          (String::from("Name"), String::from("field1")),
          (String::from("Type"), String::from("Edm.Guid"))
        ]
      ),
      close_tag!(Tag::Property),
      open_tag!(
        Tag::Property,
        [
          (String::from("Name"), String::from("field2")),
          (String::from("Type"), String::from("Edm.Int32"))
        ]
      ),
      close_tag!(Tag::Property),
      open_tag!(
        Tag::Property,
        [
          (String::from("Name"), String::from("field3")),
          (String::from("Type"), String::from("Edm.Int64"))
        ]
      ),
      close_tag!(Tag::Property),
      close_tag!(Tag::EntityType),
      close_tag!(Tag::Schema),
    ];
    let cds = parse(tags);
    assert!(cds.contains("entity Tests"));
    assert!(cds.contains("field1: UUID;"));
    assert!(cds.contains("field2: Integer;"));
    assert!(cds.contains("field3: Integer64;"));
  }

  fn parse(tag_events: Vec<TagEvent>) -> String {
    let tag_events: Vec<Result<TagEvent, TagError>> =
      tag_events.into_iter().map(|e| Ok(e)).collect();
    let mut parser = build_parser(tag_events);
    parser.parse()
  }

  fn build_parser(events: Vec<Result<TagEvent, TagError>>) -> Parser {
    let tag_parser = VecTagParser::new(events);
    Parser::new(Box::new(tag_parser))
  }

  struct VecTagParser {
    events: std::vec::IntoIter<Result<TagEvent, TagError>>,
  }

  impl VecTagParser {
    pub fn new(events: Vec<Result<TagEvent, TagError>>) -> Self {
      let events = events.into_iter();
      VecTagParser { events }
    }
  }

  impl Iterator for VecTagParser {
    type Item = Result<TagEvent, TagError>;

    fn next(&mut self) -> Option<Result<TagEvent, TagError>> {
      self.events.next()
    }
  }

  impl TagParser for VecTagParser {}
}
