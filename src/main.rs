mod cds;
mod parser;
mod util;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("CIM 0.2.0");
    println!("CDS Implementation generation by a Metadata document");
    println!("Incorrect usage path arg is missing, try cim <filename> instead");
    return;
  }
  let filename = &args[1];
  println!("CIM 0.2.0");
  println!("Reading {filename}...");
  let cds = parser::parse(filename);
  print!("{cds}");
}
