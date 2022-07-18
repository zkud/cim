use xml::attribute::OwnedAttribute;

pub fn get_attribute(attributes: &Vec<OwnedAttribute>, name: &str) -> Option<String> {
  attributes
    .iter()
    .find(|attribute| attribute.name.local_name == name)
    .and_then(|attr| Some(attr.value.to_string()))
}
