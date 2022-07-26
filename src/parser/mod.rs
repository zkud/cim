mod cds;
mod util;

use cds::entity::Entity;
use cds::field::Field;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use util::get_attribute;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

pub struct Parser {
  finished_entities: Vec<Entity>,
  entity_name: String,
  fields: HashMap<String, Field>,
  keys: Vec<String>,
  field_name: String,
  field_type: String,
  associated_target: String,
  field_attributes: Vec<OwnedAttribute>,
  schema_name: String,
  tag_parser: Option<EventReader<BufReader<File>>>,
}

impl Parser {
  pub fn new(tag_parser: EventReader<BufReader<File>>) -> Parser {
    Parser {
      finished_entities: Vec::new(),
      entity_name: String::new(),
      fields: HashMap::new(),
      keys: Vec::new(),
      field_name: String::new(),
      field_type: String::new(),
      associated_target: String::new(),
      field_attributes: Vec::new(),
      schema_name: String::new(),
      tag_parser: Some(tag_parser),
    }
  }

  pub fn parse(&mut self) -> String {
    let parser = self.tag_parser.take().unwrap();
    for e in parser {
      match e {
        Ok(XmlEvent::StartElement {
          name, attributes, ..
        }) => match name.local_name.as_ref() {
          "Schema" => {
            self.schema_name =
              get_attribute(&attributes, "Namespace").expect("Failed to get schemas's namespace");
          }
          "EntityType" => {
            self.entity_name =
              get_attribute(&attributes, "Name").expect("Failed to get entity's name");
          }
          "Property" => {
            self.field_name =
              get_attribute(&attributes, "Name").expect("Failed to get property's name");
            self.field_type =
              get_attribute(&attributes, "Type").expect("Failed to get property's type");
            self.field_attributes = attributes.clone();
          }
          "NavigationProperty" => {
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
          "PropertyRef" => {
            let field_name =
              get_attribute(&attributes, "Name").expect("Failed to get property ref's name");
            self.keys.push(field_name);
          }
          _ => (),
        },
        Ok(XmlEvent::EndElement { name }) => match name.local_name.as_ref() {
          "EntityType" => {
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
          "Property" => {
            let field =
              Field::from_odata(&self.field_name, &self.field_type, &self.field_attributes);
            self.fields.insert(self.field_name.clone(), field);
            self.field_name.clear();
            self.field_type.clear();
            self.field_attributes.clear();
          }
          "NavigationProperty" => {
            let field = Field::new_association(&self.field_name, &self.associated_target);
            self.fields.insert(self.field_name.clone(), field);
            self.associated_target.clear();
            self.field_name.clear();
          }
          _ => (),
        },
        Err(e) => {
          panic!("Error: {}", e);
        }
        _ => {}
      }
    }

    let mut cds = String::from("");
    for entity in self.finished_entities.iter() {
      cds.push_str(&entity.to_cds());
    }
    cds
  }
}
