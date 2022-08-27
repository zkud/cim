use super::field::Field;

pub struct Entity {
  name: String,
  fields: Vec<Field>,
}

impl Entity {
  pub fn new(name: &str, fields: &[Field]) -> Self {
    Entity {
      name: name.to_owned(),
      fields: fields.to_vec(),
    }
  }

  pub fn to_cds(&self) -> String {
    let mut cds = format!("entity {} {{\n", self.name);
    for field in &self.fields {
      cds.push_str("  ");
      cds.push_str(&field.to_cds());
    }
    cds.push_str("}\n");
    cds
  }
}
