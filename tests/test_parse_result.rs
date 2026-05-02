use std::fs::File;
use std::io::BufReader;
use serde_json::Error;
use argot_cli::types::ParseResult;

#[test]
fn parse_bad_result() {
    let file = File::open("tests/bad_result.json").unwrap();
    let reader = BufReader::new(file);
    let res: Result<ParseResult, Error> = serde_json::from_reader(reader);
    let err = res.unwrap_err();

    let msg = "data did not match any variant of untagged enum OptionValue at line 3 column 19";
    assert_eq!(format!("{}", err), msg);
}
