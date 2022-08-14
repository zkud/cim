use cim::run;
use cim::Args;
use std::fs::read_to_string;

#[test]
fn with_usual_metadata_it_returns_valid_cds() {
  let args = Args {
    path: "./tests/examples/parsing/metadata.xml".to_string(),
  };

  let cds = run(args);
  let valid_cds = read_to_string("./tests/examples/parsing/expected.cds").unwrap();

  assert_eq!(cds, valid_cds);
}