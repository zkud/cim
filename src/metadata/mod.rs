mod cds;
pub mod error;

#[cfg(test)]
mod tests;

use super::xml_tags::types::Tag;
use super::xml_tags::types::TagEvent;
use super::xml_tags::types::TagParser;
use cds::entity::Entity;
use cds::field::Field;
use error::ParserError;
use std::collections::HashMap;
use std::error::Error;

pub struct Parser {
  finished_entities: Vec<Entity>,
  entity_name: String,
  fields: HashMap<String, Field>,
  fields_order: Vec<String>,
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
      fields_order: Vec::new(),
      keys: Vec::new(),
      field_name: String::new(),
      field_type: String::new(),
      associated_target: String::new(),
      field_attributes: HashMap::new(),
      schema_name: String::new(),
      tag_parser: Some(tag_parser),
    }
  }

  pub fn parse(&mut self) -> Result<String, Box<dyn Error>> {
    if let Some(tag_parser) = self.tag_parser.take() {
      for e in tag_parser {
        match e {
          Ok(TagEvent::Open { tag, attributes }) => match tag {
            Tag::Schema => self.on_schema_start(&attributes)?,
            Tag::EntityType => self.on_entity_start(&attributes)?,
            Tag::Property => self.on_property_start(&attributes)?,
            Tag::NavigationProperty => self.on_navigation_property_start(&attributes)?,
            Tag::PropertyRef => self.on_property_ref(attributes)?,
          },
          Ok(TagEvent::Close { tag }) => match tag {
            Tag::EntityType => self.on_entity_close()?,
            Tag::Property => self.on_property_close()?,
            Tag::NavigationProperty => self.on_navigation_property_close(),
            _ => (),
          },
          Err(e) => {
            return Err(Box::new(e));
          }
        }
      }
    }
    Ok(self.compose_cds_string())
  }

  fn on_schema_start(
    &mut self,
    attributes: &HashMap<String, String>,
  ) -> Result<(), Box<dyn Error>> {
    self.schema_name = attributes
      .get("Namespace")
      .ok_or(ParserError::new_boxed("Failed to get schema name"))?
      .to_string();
    Ok(())
  }

  fn on_entity_start(
    &mut self,
    attributes: &HashMap<String, String>,
  ) -> Result<(), Box<dyn Error>> {
    self.entity_name = attributes
      .get("Name")
      .ok_or(ParserError::new_boxed("Failed to get entity's name"))?
      .to_string();
    Ok(())
  }

  fn on_property_start(
    &mut self,
    attributes: &HashMap<String, String>,
  ) -> Result<(), Box<dyn Error>> {
    self.field_name = attributes
      .get("Name")
      .ok_or(ParserError::new_boxed("Failed to get property's name"))?
      .to_string();
    self.field_type = attributes
      .get("Type")
      .ok_or(ParserError::new_boxed("Failed to get property's type"))?
      .to_string();
    self.field_attributes = attributes.clone();
    Ok(())
  }

  fn on_navigation_property_start(
    &mut self,
    attributes: &HashMap<String, String>,
  ) -> Result<(), Box<dyn Error>> {
    self.field_name = attributes
      .get("Name")
      .ok_or(ParserError::new_boxed("Failed to get nav. property's name"))?
      .to_string();
    self.associated_target = attributes
      .get("Type")
      .ok_or(ParserError::new_boxed(
        "Failed to get nav. property's target",
      ))?
      .to_string();
    if self.schema_name.len() > 0 {
      self.associated_target = self
        .associated_target
        .replace(&(self.schema_name.clone() + "."), "");
    }
    self.associated_target = self.associated_target.replace("Collection(", "");
    self.associated_target = self.associated_target.replace(")", "");
    Ok(())
  }

  fn on_property_ref(&mut self, attributes: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let field_name = attributes
      .get("Name")
      .ok_or(ParserError::new_boxed("Failed to get property ref's name"))?
      .to_string();
    self.keys.push(field_name);
    Ok(())
  }

  fn on_entity_close(&mut self) -> Result<(), Box<dyn Error>> {
    for key in self.keys.iter() {
      self
        .fields
        .get_mut(key)
        .ok_or(ParserError::new_boxed("Unknown property in property ref"))?
        .set_as_key()
    }
    let entity_fields = self
      .fields_order
      .iter()
      .map(|name| self.fields.get(name).unwrap().clone())
      .collect();
    let entity = Entity::new(&self.entity_name, &entity_fields);
    self.keys.clear();
    self.finished_entities.push(entity);
    self.entity_name.clear();
    self.fields.clear();
    self.fields_order.clear();
    Ok(())
  }

  fn on_property_close(&mut self) -> Result<(), Box<dyn Error>> {
    let field = Field::from_odata(&self.field_name, &self.field_type, &self.field_attributes)?;
    self.fields.insert(self.field_name.clone(), field);
    self.fields_order.push(self.field_name.clone());
    self.field_name.clear();
    self.field_type.clear();
    self.field_attributes.clear();
    Ok(())
  }

  fn on_navigation_property_close(&mut self) {
    let field = Field::new_association(&self.field_name, &self.associated_target);
    self.fields.insert(self.field_name.clone(), field);
    self.fields_order.push(self.field_name.clone());
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
