use std::collections::HashMap;

pub fn get_attribute(attributes: &HashMap<String, String>, name: &str) -> Option<String> {
  attributes.get(name).and_then(|value| Some(value.clone()))
}
