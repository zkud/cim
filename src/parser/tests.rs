use super::tag_parser::types::Tag;
use super::tag_parser::types::TagError;
use super::tag_parser::types::TagEvent;
use super::tag_parser::types::TagParser;
use super::Parser;
use std::collections::HashMap;

macro_rules! open_tag {
    ($tag_type: expr, $(($key: expr, $val: expr)),*) => {
      TagEvent::Open {
        tag: $tag_type,
        attributes: HashMap::from([$(map_spec!($key, $val)),*]),
      }
    };
  }
macro_rules! map_spec {
  ($key: expr, $value: expr) => {
    (String::from($key), String::from($value))
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
    open_tag!(Tag::Schema, ("Namespace", "test")),
    open_tag!(Tag::EntityType, ("Name", "Tests")),
    open_tag!(Tag::Property, ("Name", "field1"), ("Type", "Edm.Guid")),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "field2"), ("Type", "Edm.Int32")),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "field3"), ("Type", "Edm.Int64")),
    close_tag!(Tag::Property),
    close_tag!(Tag::EntityType),
    open_tag!(Tag::EntityType, ("Name", "Product")),
    open_tag!(Tag::PropertyRef, ("Name", "ID")),
    close_tag!(Tag::PropertyRef),
    open_tag!(
      Tag::Property,
      ("Name", "ID"),
      ("Type", "Edm.Int32"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Name"),
      ("Type", "Edm.String"),
      ("FC_TargetPath", "SyndicationTitle"),
      ("FC_ContentKind", "text"),
      ("FC_KeepInContent", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Description"),
      ("Type", "Edm.String"),
      ("FC_TargetPath", "SyndicationSummary"),
      ("FC_ContentKind", "text"),
      ("FC_KeepInContent", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "ReleaseDate"),
      ("Type", "Edm.DateTime"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "DiscontinuedDate"),
      ("Type", "Edm.DateTime")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Rating"),
      ("Type", "Edm.Int16"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Price"),
      ("Type", "Edm.Double"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "Categories"),
      (
        "Relationship",
        "ODataDemo.Product_Categories_Category_Products"
      ),
      ("ToRole", "Category_Products"),
      ("FromRole", "Product_Categories")
    ),
    close_tag!(Tag::NavigationProperty),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "Supplier"),
      (
        "Relationship",
        "ODataDemo.Product_Supplier_Supplier_Products"
      ),
      ("ToRole", "Supplier_Products"),
      ("FromRole", "Product_Supplier")
    ),
    close_tag!(Tag::NavigationProperty),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "ProductDetail"),
      (
        "Relationship",
        "ODataDemo.Product_ProductDetail_ProductDetail_Product"
      ),
      ("ToRole", "ProductDetail_Product"),
      ("FromRole", "Product_ProductDetail")
    ),
    close_tag!(Tag::NavigationProperty),
    close_tag!(Tag::EntityType),
    open_tag!(
      Tag::EntityType,
      ("Name", "FeaturedProduct"),
      ("BaseType", "ODataDemo.Product")
    ),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "Advertisement"),
      (
        "Relationship",
        "ODataDemo.FeaturedProduct_Advertisement_Advertisement_FeaturedProduct"
      ),
      ("ToRole", "Advertisement_FeaturedProduct"),
      ("FromRole", "FeaturedProduct_Advertisement")
    ),
    close_tag!(Tag::NavigationProperty),
    close_tag!(Tag::EntityType),
    open_tag!(Tag::EntityType, ("Name", "ProductDetail")),
    open_tag!(Tag::PropertyRef, ("Name", "ProductID")),
    close_tag!(Tag::PropertyRef),
    open_tag!(
      Tag::Property,
      ("Name", "ProductID"),
      ("Type", "Edm.Int32"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "Details"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "Product"),
      (
        "Relationship",
        "ODataDemo.Product_ProductDetail_ProductDetail_Product"
      ),
      ("ToRole", "Product_ProductDetail"),
      ("FromRole", "ProductDetail_Product")
    ),
    close_tag!(Tag::NavigationProperty),
    close_tag!(Tag::EntityType),
    open_tag!(Tag::EntityType, ("Name", "Category"), ("OpenType", "true")),
    open_tag!(Tag::PropertyRef, ("Name", "ID")),
    close_tag!(Tag::PropertyRef),
    open_tag!(
      Tag::Property,
      ("Name", "ID"),
      ("Type", "Edm.Int32"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Name"),
      ("Type", "Edm.String"),
      ("FC_TargetPath", "SyndicationTitle"),
      ("FC_ContentKind", "text"),
      ("FC_KeepInContent", "true")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "Products"),
      (
        "Relationship",
        "ODataDemo.Product_Categories_Category_Products"
      ),
      ("ToRole", "Product_Categories"),
      ("FromRole", "Category_Products")
    ),
    close_tag!(Tag::NavigationProperty),
    close_tag!(Tag::EntityType),
    open_tag!(Tag::EntityType, ("Name", "Supplier")),
    open_tag!(Tag::PropertyRef, ("Name", "ID")),
    close_tag!(Tag::PropertyRef),
    open_tag!(
      Tag::Property,
      ("Name", "ID"),
      ("Type", "Edm.Int32"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Name"),
      ("Type", "Edm.String"),
      ("FC_TargetPath", "SyndicationTitle"),
      ("FC_ContentKind", "text"),
      ("FC_KeepInContent", "true")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Concurrency"),
      ("Type", "Edm.Int32"),
      ("ConcurrencyMode", "Fixed"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "Products"),
      (
        "Relationship",
        "ODataDemo.Product_Supplier_Supplier_Products"
      ),
      ("ToRole", "Product_Supplier"),
      ("FromRole", "Supplier_Products")
    ),
    close_tag!(Tag::NavigationProperty),
    open_tag!(Tag::Property, ("Name", "Street"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "City"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "State"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "ZipCode"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "Country"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    close_tag!(Tag::EntityType),
    open_tag!(Tag::EntityType, ("Name", "Person")),
    open_tag!(Tag::PropertyRef, ("Name", "ID")),
    close_tag!(Tag::PropertyRef),
    open_tag!(
      Tag::Property,
      ("Name", "ID"),
      ("Type", "Edm.Int32"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "Name"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "PersonDetail"),
      (
        "Relationship",
        "ODataDemo.Person_PersonDetail_PersonDetail_Person"
      ),
      ("ToRole", "PersonDetail_Person"),
      ("FromRole", "Person_PersonDetail")
    ),
    close_tag!(Tag::NavigationProperty),
    close_tag!(Tag::EntityType),
    open_tag!(
      Tag::EntityType,
      ("Name", "Customer"),
      ("BaseType", "ODataDemo.Person")
    ),
    open_tag!(
      Tag::Property,
      ("Name", "TotalExpense"),
      ("Type", "Edm.Decimal"),
      ("Scale", "10"),
      ("Precision", "5"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    close_tag!(Tag::EntityType),
    open_tag!(
      Tag::EntityType,
      ("Name", "Employee"),
      ("BaseType", "ODataDemo.Person")
    ),
    open_tag!(
      Tag::Property,
      ("Name", "EmployeeID"),
      ("Type", "Edm.Int64"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "HireDate"),
      ("Type", "Edm.DateTime"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Salary"),
      ("Type", "Edm.Single"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    close_tag!(Tag::EntityType),
    open_tag!(Tag::EntityType, ("Name", "PersonDetail")),
    open_tag!(Tag::PropertyRef, ("Name", "PersonID")),
    close_tag!(Tag::PropertyRef),
    open_tag!(
      Tag::Property,
      ("Name", "PersonID"),
      ("Type", "Edm.Int32"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Age"),
      ("Type", "Edm.Byte"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Gender"),
      ("Type", "Edm.Boolean"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "Phone"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "Photo"),
      ("Type", "Edm.Stream"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "Person"),
      (
        "Relationship",
        "ODataDemo.Person_PersonDetail_PersonDetail_Person"
      ),
      ("ToRole", "Person_PersonDetail"),
      ("FromRole", "PersonDetail_Person")
    ),
    close_tag!(Tag::NavigationProperty),
    close_tag!(Tag::EntityType),
    open_tag!(
      Tag::EntityType,
      ("Name", "Advertisement"),
      ("HasStream", "true")
    ),
    open_tag!(Tag::PropertyRef, ("Name", "ID")),
    close_tag!(Tag::PropertyRef),
    open_tag!(
      Tag::Property,
      ("Name", "ID"),
      ("Type", "Edm.Guid"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(Tag::Property, ("Name", "Name"), ("Type", "Edm.String")),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::Property,
      ("Name", "AirDate"),
      ("Type", "Edm.DateTime"),
      ("Nullable", "false")
    ),
    close_tag!(Tag::Property),
    open_tag!(
      Tag::NavigationProperty,
      ("Name", "FeaturedProduct"),
      (
        "Relationship",
        "ODataDemo.FeaturedProduct_Advertisement_Advertisement_FeaturedProduct"
      ),
      ("ToRole", "FeaturedProduct_Advertisement"),
      ("FromRole", "Advertisement_FeaturedProduct")
    ),
    close_tag!(Tag::NavigationProperty),
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
  let tag_events: Vec<Result<TagEvent, TagError>> = tag_events.into_iter().map(|e| Ok(e)).collect();
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
