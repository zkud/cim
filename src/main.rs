mod cds;
mod parser;
mod util;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  println!("CIM 0.1.0");
  println!("Reading {filename}...");
  let cds = parser::parse(filename);
  print!("{cds}");
}
