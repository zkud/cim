use cim::run;
use cim::Args;
use clap::Parser;
use std::process::exit;

fn main() {
  let args = Args::parse();
  println!("CIM {}", env!("CARGO_PKG_VERSION"));
  println!("Reading {}...", args.path);
  match run(args) {
    Ok(cds) => {
      print!("{cds}");
    }
    Err(error) => {
      eprintln!("{error}");
      exit(1);
    }
  }
}
