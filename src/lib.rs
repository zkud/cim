mod metadata;
mod xml_tags;

use clap::Parser;
use xml_tags::xml_rs::XmlTagParser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(help = "Metadata file path")]
  pub path: String,
}

pub fn run(args: Args) -> String {
  let path = args.path;
  let tag_parser = XmlTagParser::new(path);
  let mut parser = metadata::Parser::new(Box::new(tag_parser));
  let cds = parser.parse();
  cds
}
