use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Field {
  is_key: bool,
  name: String,
  cds_type: CDSType,
}

impl Field {
  pub fn from_odata(name: &str, odata_type: &str, attributes: &HashMap<String, String>) -> Self {
    Field::new(
      name.to_owned(),
      CDSType::from_odata(odata_type.to_owned(), attributes),
    )
  }

  pub fn new_association(name: &str, target: &str) -> Self {
    Field::new(
      name.to_owned(),
      CDSType::Association {
        target: target.to_owned(),
      },
    )
  }

  fn new(name: String, cds_type: CDSType) -> Self {
    Field {
      name,
      cds_type,
      is_key: false,
    }
  }

  pub fn to_cds(&self) -> String {
    if self.is_key {
      format!("key {}: {};\n", self.name, self.cds_type)
    } else {
      format!("{}: {};\n", self.name, self.cds_type)
    }
  }

  pub fn set_as_key(&mut self) {
    self.is_key = true;
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
  Single,
  Byte,
  SByte,
  Stream,
  Association { target: String },
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
      CDSType::Single => String::from("Double @odata.Type: 'Edm.Single'"),
      CDSType::Byte => String::from("Integer @odata.Type: 'Edm.Byte'"),
      CDSType::SByte => String::from("Integer @odata.Type: 'Edm.SByte'"),
      CDSType::Stream => String::from("LargeBinary @odata.Type: 'Edm.Stream'"),
      CDSType::Association { target } => format!("Association to {target} on ..."),
    };

    write!(fmt, "{}", type_string)
  }
}

impl CDSType {
  fn from_odata(odata_type: String, attributes: &HashMap<String, String>) -> Self {
    match odata_type.as_str() {
      "Edm.Guid" => Self::UUID,
      "Edm.Boolean" => Self::Boolean,
      "Edm.Int16" => Self::Integer,
      "Edm.Int32" => Self::Integer,
      "Edm.Int64" => Self::Integer64,
      "Edm.Decimal" => {
        let scale = attributes.get("scale").cloned();
        let scale = scale.or(attributes.get("Scale").cloned());
        let precision = attributes.get("precision").cloned();
        let precision = precision.or(attributes.get("Precision").cloned());
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
        let length = attributes.get("MaxLength").cloned();
        Self::String { length }
      }
      "Edm.Binary" => Self::Binary,
      "Edm.Single" => Self::Single,
      "Edm.Byte" => Self::Byte,
      "Edm.SByte" => Self::SByte,
      "Edm.Stream" => Self::Stream,
      _ => panic!("Unknown/Unsupported OData Type '{odata_type}'"),
    }
  }
}
