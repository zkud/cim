use std::fs::File;
use std::io::BufReader;

use super::super::types::Tag;
use super::super::types::TagError;
use super::super::types::TagEvent;
use super::super::types::TagParser;

use super::XmlTagParser;

#[test]
fn with_usual_metadata_it_raises_valid_tags() {}
