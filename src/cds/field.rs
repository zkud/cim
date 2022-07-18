use super::super::util::get_attribute;
use std::fmt::{Display, Formatter, Result};
use xml::attribute::OwnedAttribute;

#[derive(Clone)]
pub struct Field {
  name: String,
  cds_type: CDSType,
}

impl Field {
  pub fn from_odata(name: &str, odata_type: &str, attributes: &Vec<OwnedAttribute>) -> Self {
    Field::new(
      name.to_owned(),
      CDSType::from_odata(odata_type.to_owned(), attributes),
    )
  }

  fn new(name: String, cds_type: CDSType) -> Self {
    Field { name, cds_type }
  }

  pub fn to_cds(&self) -> String {
    format!("{}: {};\n", self.name, self.cds_type)
  }
}

#[derive(Clone, Debug)]
enum CDSType {
  UUID,
  Boolean,
  Integer,
  Integer64,
  Decimal { precision: String, scale: String },
  Double,
  Date,
  Time,
  DateTime,
  String { length: Option<String> },
  Binary,
}

impl Display for CDSType {
  fn fmt(&self, fmt: &mut Formatter) -> Result {
    let type_string = match self {
      CDSType::UUID => String::from("UUID"),
      CDSType::Boolean => String::from("Boolean"),
      CDSType::Integer => String::from("Integer"),
      CDSType::Integer64 => String::from("Integer64"),
      CDSType::Decimal { precision, scale } => format!("Decimal({precision}, {scale})"),
      CDSType::Double => String::from("Double"),
      CDSType::Date => String::from("Date"),
      CDSType::Time => String::from("Time"),
      CDSType::DateTime => String::from("DateTime"),
      CDSType::String { length } => match length {
        Some(length) => format!("String({length})"),
        None => format!("String"),
      },
      CDSType::Binary => String::from("Binary"),
    };

    write!(fmt, "{}", type_string)
  }
}

impl CDSType {
  fn from_odata(odata_type: String, attributes: &Vec<OwnedAttribute>) -> Self {
    match odata_type.as_str() {
      "Edm.Guid" => Self::UUID,
      "Edm.Boolean" => Self::Boolean,
      "Edm.Int32" => Self::Integer,
      "Edm.Int64" => Self::Integer64,
      "Edm.Decimal" => {
        let scale = get_attribute(&attributes, "scale");
        let precision = get_attribute(&attributes, "precision");
        match (scale, precision) {
          (Some(scale), Some(precision)) => Self::Decimal { scale, precision },
          _ => panic!("Failed to parse a Decimal type, scale or precision is missing"),
        }
      }
      "Edm.Double" => Self::Double,
      "Edm.Date" => Self::Date,
      "Edm.TimeOfDay" | "Edm.Time" => Self::Time,
      "Edm.DateTime" | "Edm.DateTimeOffset" => Self::DateTime,
      "Edm.String" => {
        let length = get_attribute(&attributes, "MaxLength");
        Self::String { length }
      }
      "Edm.Binary" => Self::Binary,
      _ => panic!("Unknown/Unsupported OData Type"),
    }
  }
}
