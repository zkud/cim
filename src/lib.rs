mod cds;
mod parser;
mod util;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(help = "Metadata file path")]
  pub path: String,
}

pub fn run(args: Args) -> String {
  let path = args.path;
  let cds = parser::parse(&path);
  cds
}
