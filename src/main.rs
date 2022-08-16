use cim::run;
use cim::Args;
use clap::Parser;

fn main() {
  let args = Args::parse();
  println!("CIM 0.3.3");
  println!("Reading {}...", args.path);
  let cds = run(args);
  print!("{cds}");
}
