use super::cds::entity::Entity;
use super::cds::field::Field;
use super::util::get_attribute;
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
  let mut fields: Vec<Field> = Vec::new();
  let mut field_name: String = String::from("");
  let mut field_type: String = String::from("");
  let mut field_attributes: Vec<OwnedAttribute> = Vec::new();
  for e in parser {
    match e {
      Ok(XmlEvent::StartElement {
        name, attributes, ..
      }) => match name.local_name.as_ref() {
        "EntityType" => {
          entity_name = get_attribute(&attributes, "Name").expect("Failed to get entity's name");
        }
        "Property" => {
          field_name = get_attribute(&attributes, "Name").expect("Failed to get property's name");
          field_type = get_attribute(&attributes, "Type").expect("Failed to get property's type");
          field_attributes = attributes.clone();
        }
        _ => (),
      },
      Ok(XmlEvent::EndElement { name }) => match name.local_name.as_ref() {
        "EntityType" => {
          let entity = Entity::new(&entity_name, &fields);
          finished_entities.push(entity);
          entity_name.clear();
          fields.clear();
        }
        "Property" => {
          let field = Field::from_odata(&field_name, &field_type, &field_attributes);
          fields.push(field);
          field_name.clear();
          field_type.clear();
          field_attributes.clear();
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
