use super::cds::entity::Entity;
use super::cds::field::Field;
use super::util::get_attribute;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

pub fn parse(filename: &str) -> String {
  let file = File::open(filename).unwrap();
  let file = BufReader::new(file);

  let parser = EventReader::new(file);
  let mut finished_entities: Vec<Entity> = Vec::new();
  let mut entity_name: String = String::from("");
  let mut fields: HashMap<String, Field> = HashMap::new();
  let mut keys: Vec<String> = Vec::new();
  let mut field_name: String = String::from("");
  let mut field_type: String = String::from("");
  let mut associated_target: String = String::from("");
  let mut field_attributes: Vec<OwnedAttribute> = Vec::new();
  let mut schema_name: String = String::from("");
  for e in parser {
    match e {
      Ok(XmlEvent::StartElement {
        name, attributes, ..
      }) => match name.local_name.as_ref() {
        "Schema" => {
          schema_name =
            get_attribute(&attributes, "Namespace").expect("Failed to get schemas's namespace");
        }
        "EntityType" => {
          entity_name = get_attribute(&attributes, "Name").expect("Failed to get entity's name");
        }
        "Property" => {
          field_name = get_attribute(&attributes, "Name").expect("Failed to get property's name");
          field_type = get_attribute(&attributes, "Type").expect("Failed to get property's type");
          field_attributes = attributes.clone();
        }
        "NavigationProperty" => {
          field_name =
            get_attribute(&attributes, "Name").expect("Failed to get nav. property's name");
          associated_target =
            get_attribute(&attributes, "Type").expect("Failed to get nav. property's target");
          if schema_name.len() > 0 {
            associated_target = associated_target.replace(&(schema_name.clone() + "."), "");
          }
          associated_target = associated_target.replace("Collection(", "");
          associated_target = associated_target.replace(")", "");
        }
        "PropertyRef" => {
          let field_name =
            get_attribute(&attributes, "Name").expect("Failed to get property ref's name");
          keys.push(field_name);
        }
        _ => (),
      },
      Ok(XmlEvent::EndElement { name }) => match name.local_name.as_ref() {
        "EntityType" => {
          for key in keys.iter() {
            fields
              .get_mut(key)
              .expect("Unknown property in property ref")
              .set_as_key()
          }
          let entity_fields = fields.clone().into_values().collect();
          let entity = Entity::new(&entity_name, &entity_fields);
          keys.clear();
          finished_entities.push(entity);
          entity_name.clear();
          fields.clear();
        }
        "Property" => {
          let field = Field::from_odata(&field_name, &field_type, &field_attributes);
          fields.insert(field_name.clone(), field);
          field_name.clear();
          field_type.clear();
          field_attributes.clear();
        }
        "NavigationProperty" => {
          let field = Field::new_association(&field_name, &associated_target);
          fields.insert(field_name.clone(), field);
          associated_target.clear();
          field_name.clear();
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
  for entity in finished_entities {
    cds.push_str(&entity.to_cds());
  }
  cds
}
