mod parser;

use std::fs::File;
use std::io::BufReader;

use clap::Parser;
use xml::reader::EventReader;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(help = "Metadata file path")]
  pub path: String,
}

pub fn run(args: Args) -> String {
  let path = args.path;

  let file = File::open(path).unwrap();
  let file = BufReader::new(file);
  let tag_parser = EventReader::new(file);

  let mut parser = parser::Parser::new(tag_parser);
  let cds = parser.parse();
  cds
}
