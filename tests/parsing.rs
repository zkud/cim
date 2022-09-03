use cim::run;
use cim::Args;
use std::fs::read_to_string;

#[test]
fn with_usual_metadata_it_returns_valid_cds() {
  let args = Args {
    path: "./tests/examples/parsing/ok/metadata/1.xml".to_string(),
  };

  let cds = run(args).unwrap();
  let valid_cds = read_to_string("./tests/examples/parsing/ok/expected/1.cds").unwrap();
  let valid_cds = valid_cds.replace("\r\n", "\n"); // For windows compatibility

  assert_eq!(cds, valid_cds);
}

#[test]
fn with_incorrect_metadata_it_returns_errors() {
  for test_index in 1..6 {
    let args = Args {
      path: format!("./tests/examples/parsing/error/metadata/{}.xml", test_index),
    };

    if let Err(error) = run(args) {
      let error_message = error.to_string();
      let expected_path = format!("./tests/examples/parsing/error/expected/{}.txt", test_index);
      let expected_message = read_to_string(expected_path).unwrap();
      assert_eq!(error_message, expected_message);
      continue;
    }
    panic!("Missed parsing error");
  }
}

#[test]
#[should_panic]
fn with_missing_file_it_returns_error() {
  let args = Args {
    path: "./invldpath.xml".to_string(),
  };
  run(args).unwrap();
}
