use cim::run;
use cim::Args;
use clap::Parser;

#[cfg(not(tarpaulin_include))]
fn main() {
  let args = Args::parse();
  println!("CIM 0.3.2");
  println!("Reading {}...", args.path);
  let cds = run(args);
  print!("{cds}");
}
