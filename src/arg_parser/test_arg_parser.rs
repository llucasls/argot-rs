use super::*;
use std::hash::Hash;
use std::sync::LazyLock;
use crate::parser_config;

fn assert_mapping_equals<K, V>(
    actual: &HashMap<K, V>,
    expected: &HashMap<K, V>
)
where
    K: std::fmt::Debug + Eq + Hash,
    V: std::fmt::Debug + std::cmp::PartialEq,
{
    let mut unexpected: HashMap<&K, &V> = HashMap::with_capacity(actual.len());
    let mut not_found: HashMap<&K, &V> = HashMap::with_capacity(expected.len());
    let mut different: HashMap<&K, (&V, &V)> = HashMap::with_capacity(expected.len());
    for (key, expected_value) in expected.iter() {
        match actual.get(&key) {
            Some(actual_value) => {
                if actual_value != expected_value {
                    different.insert(key, (actual_value, expected_value));
                }
            },
            None => {
                not_found.insert(key, expected_value);
            },
        }
    }

    for (key, actual_value) in actual.iter() {
        match expected.get(&key) {
            Some(expected_value) => {
                if actual_value != expected_value {
                    different.insert(key, (actual_value, expected_value));
                }
            },
            None => {
                unexpected.insert(key, actual_value);
            }
        }
    }

    match (!unexpected.is_empty(), !not_found.is_empty(), !different.is_empty()) {
        (false, false, false) => { /* no error */ },
        (false, false, true) => {
            panic!("hash maps did not match:\n    different: {:?}", different);
        },
        (false, true, false) => {
            panic!("hash maps did not match:\n    not found: {:?}", not_found);
        },
        (false, true, true) => {
            panic!("hash maps did not match:\n    not found: {:?}\n    different: {:?}", not_found, different)
        },
        (true, false, false) => {
            panic!("hash maps did not match:\n    unexpected: {:?}", unexpected);
        },
        (true, false, true) => {
            panic!("hash maps did not match:\n    unexpected: {:?}\n    different: {:?}", unexpected, different);
        },
        (true, true, false) => {
            panic!("hash maps did not match:\n    unexpected: {:?}\n    not found: {:?}", unexpected, not_found);
        },
        (true, true, true) => {
            panic!("hash maps did not match:\n    unexpected: {:?}\n    not found: {:?}\n    different: {:?}",
                unexpected,
                not_found,
                different);
        },
    }
}

static ARG_PARSER_CELL: LazyLock<ArgParser> = LazyLock::new(|| {
    let configs: ParserConfig = parser_config! {
        "strict" => Flag,
        "output" => Text,
        "output-file" => Alias { target: "output" },
        "logfile" => Text { default: "access.log" },
        "log-file" => Alias { target: "logfile" },
        "retries" => Int,
        "retry" => Alias { target: "retries" },
        "threads" => Int { default: 0 },
        "jobs" => Alias { target: "threads" },
        "loglevel" => Count,
        "verbosity" => Alias { target: "loglevel" },
        "tasks" => List,
        "path" => Alias { target: "P" },
        "dry-run" => Alias { target: "n" },
        "user" => Alias { target: "u" },
        "id" => Alias { target: "i" },
        "targets" => Alias { target: "t" },
        "permission" => Alias { target: "p" },
        "n" => Flag,
        "u" => Text,
        "U" => Alias { target: "u" },
        "g" => Text,
        "G" => Alias { target: "g" },
        "e" => Text { default: "test" },
        "i" => Int,
        "I" => Alias { target: "i" },
        "j" => Int,
        "J" => Alias { target: "threads" },
        "a" => Int { default: 0 },
        "t" => List,
        "p" => Count,
        "v" => Alias { target: "loglevel" },
        "s" => Alias { target: "strict" },
        "o" => Alias { target: "output" },
        "O" => Alias { target: "logfile" },
        "r" => Alias { target: "retries" },
        "f" => Alias { target: "logfile" },
        "P" => List { sep: ":" },
        "T" => Alias { target: "tasks" },
        "E" => List,
    }.unwrap();
    ArgParser::new(configs)
});

#[test]
fn parse_parameters() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["CC=clang", "ENV=", "=", "=test"];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("CC".to_string(), "clang".to_string()),
        ("ENV".to_string(), "".to_string()),
    ]);

    assert_mapping_equals(result.parameters(), &expected);
    assert_eq!(result.operands(), ["=", "=test"]);
}

#[test]
fn parse_flag_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["--strict", "CC=clang", "main.o", "-n"];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("strict".to_string(), OptionValue::Flag),
        ("n".to_string(), OptionValue::Flag),
    ]);

    assert_eq!(result.operands(), ["main.o"]);
    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn parse_text_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = [
        "--output=doc.txt",
        "--logfile",
        "-ubob",
        "-g",
        "users",
        "-e",
    ];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("output".to_string(), OptionValue::Text("doc.txt".to_string())),
        ("logfile".to_string(), OptionValue::Text("access.log".to_string())),
        ("u".to_string(), OptionValue::Text("bob".to_string())),
        ("g".to_string(), OptionValue::Text("users".to_string())),
        ("e".to_string(), OptionValue::Text("test".to_string())),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn parse_int_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = [
        "--retries=3",
        "--threads",
        "-j4",
        "-i",
        "2",
        "-a",
    ];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("retries".to_string(), OptionValue::Int(3)),
        ("threads".to_string(), OptionValue::Int(0)),
        ("i".to_string(), OptionValue::Int(2)),
        ("j".to_string(), OptionValue::Int(4)),
        ("a".to_string(), OptionValue::Int(0)),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn parse_count_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["--loglevel=2", "-pp", "-p"];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("loglevel".to_string(), OptionValue::Int(2)),
        ("p".to_string(), OptionValue::Int(3)),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn parse_list_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = [
        "--tasks=build,test",
        "--path=~/.local/bin:~/bin",
        "-P",
        "~/.cargo/bin",
        "-T",
        "all",
        "-Etest,staging",
        "-E",
        "build",
    ];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("tasks".to_string(),
        OptionValue::List(vec![
            "build".to_string(),
            "test".to_string(),
            "all".to_string(),
        ])),
        ("P".to_string(),
        OptionValue::List(vec![
            "~/.local/bin".to_string(),
            "~/bin".to_string(),
            "~/.cargo/bin".to_string(),
        ])),
        ("E".to_string(),
        OptionValue::List(vec![
            "test".to_string(),
            "staging".to_string(),
            "build".to_string(),
        ])),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn parse_alias_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = [
        "-vvv",
        "-so",
        "doc.txt",
        "-Ujohn",
        "-G",
        "staff",
        "-O",
        "-J",
        "-r4",
        "-I",
        "12",
        "-Tbuild",
        "-T",
        "check",
        "-T",
        "",
    ];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("loglevel".to_string(), OptionValue::Int(3)),
        ("logfile".to_string(), OptionValue::Text("access.log".to_string())),
        ("strict".to_string(), OptionValue::Flag),
        ("output".to_string(), OptionValue::Text("doc.txt".to_string())),
        ("threads".to_string(), OptionValue::Int(0)),
        ("retries".to_string(), OptionValue::Int(4)),
        ("tasks".to_string(),
        OptionValue::List(vec![
            "build".to_string(),
            "check".to_string(),
        ])),
        ("u".to_string(), OptionValue::Text("john".to_string())),
        ("g".to_string(), OptionValue::Text("staff".to_string())),
        ("i".to_string(), OptionValue::Int(12)),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn parse_options_as_operands() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["-vvv", "--", "-so", "--", "doc.txt"];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("loglevel".to_string(), OptionValue::Int(3)),
    ]);

    assert_mapping_equals(result.options(), &expected);
    assert_eq!(result.operands(), ["-so", "--", "doc.txt"]);
}

#[test]
fn parse_list_option_with_empty_values() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["--tasks=", "-P", ""];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("tasks".to_string(), OptionValue::List(Vec::new())),
        ("P".to_string(), OptionValue::List(Vec::new())),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn parse_count_option_without_value() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["--loglevel"];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("loglevel".to_string(), OptionValue::Int(1)),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn return_error_on_text_option_without_associated_value() {
    let parser: &ArgParser = &ARG_PARSER_CELL;

    let input1 = ["--output"];
    let result1: ArgotError = parser.parse(input1).unwrap_err();
    let expected1 = ArgotError::NullArg {
        option: "output".to_string(),
        target: None,
    };

    let input2 = ["-u"];
    let result2: ArgotError = parser.parse(input2).unwrap_err();
    let expected2 = ArgotError::NullArg {
        option: "u".to_string(),
        target: None,
    };

    let input3 = ["--output-file"];
    let result3: ArgotError = parser.parse(input3).unwrap_err();
    let expected3 = ArgotError::NullArg {
        option: "output-file".to_string(),
        target: Some("output".to_string()),
    };

    let input4 = ["-o"];
    let result4: ArgotError = parser.parse(input4).unwrap_err();
    let expected4 = ArgotError::NullArg {
        option: "o".to_string(),
        target: Some("output".to_string()),
    };

    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
    assert_eq!(result3, expected3);
    assert_eq!(result4, expected4);
}

#[test]
fn return_error_on_int_option_without_associated_value() {
    let parser: &ArgParser = &ARG_PARSER_CELL;

    let input1 = ["--retries"];
    let result1: ArgotError = parser.parse(input1).unwrap_err();
    let expected1 = ArgotError::NullInt {
        option: "retries".to_string(),
        target: None,
    };

    let input2 = ["-j"];
    let result2: ArgotError = parser.parse(input2).unwrap_err();
    let expected2 = ArgotError::NullInt {
        option: "j".to_string(),
        target: None,
    };

    let input3 = ["--retry"];
    let result3: ArgotError = parser.parse(input3).unwrap_err();
    let expected3 = ArgotError::NullInt {
        option: "retry".to_string(),
        target: Some("retries".to_string()),
    };

    let input4 = ["-r"];
    let result4: ArgotError = parser.parse(input4).unwrap_err();
    let expected4 = ArgotError::NullInt {
        option: "r".to_string(),
        target: Some("retries".to_string()),
    };

    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
    assert_eq!(result3, expected3);
    assert_eq!(result4, expected4);
}

#[test]
fn return_error_on_list_option_without_associated_value() {
    let parser: &ArgParser = &ARG_PARSER_CELL;

    let input1 = ["--tasks"];
    let result1: ArgotError = parser.parse(input1).unwrap_err();
    let expected1 = ArgotError::NullArg {
        option: "tasks".to_string(),
        target: None,
    };

    let input2 = ["-P"];
    let result2: ArgotError = parser.parse(input2).unwrap_err();
    let expected2 = ArgotError::NullArg {
        option: "P".to_string(),
        target: None,
    };

    let input3 = ["--path"];
    let result3: ArgotError = parser.parse(input3).unwrap_err();
    let expected3 = ArgotError::NullArg {
        option: "path".to_string(),
        target: Some("P".to_string()),
    };

    let input4 = ["-T"];
    let result4: ArgotError = parser.parse(input4).unwrap_err();
    let expected4 = ArgotError::NullArg {
        option: "T".to_string(),
        target: Some("tasks".to_string()),
    };

    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
    assert_eq!(result3, expected3);
    assert_eq!(result4, expected4);
}

#[test]
fn parse_alias_long_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = [
        "--dry-run",
        "--user=bob",
        "--id=7525",
        "--targets=build,ci,test",
        "--permission=3",
        "--jobs",
        "--log-file",
        "--verbosity",
        "--path=",
    ];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("i".to_string(), OptionValue::Int(7525)),
        ("logfile".to_string(), OptionValue::Text("access.log".to_string())),
        ("loglevel".to_string(), OptionValue::Int(1)),
        ("n".to_string(), OptionValue::Flag),
        ("p".to_string(), OptionValue::Int(3)),
        ("t".to_string(),
        OptionValue::List(vec![
            "build".to_string(),
            "ci".to_string(),
            "test".to_string(),
        ])),
        ("threads".to_string(), OptionValue::Int(0)),
        ("u".to_string(), OptionValue::Text("bob".to_string())),
        ("P".to_string(), OptionValue::List(Vec::new())),
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn return_error_on_unsupported_options() {
    let parser: &ArgParser = &ARG_PARSER_CELL;

    let input1 = ["--esto", "--no-ecxiste"];
    let result1: ArgotError = parser.parse(input1).unwrap_err();
    let expected1 = ArgotError::UnknownOption {
        option: "esto".to_string(),
    };

    let input2 = ["-Z"];
    let result2: ArgotError = parser.parse(input2).unwrap_err();
    let expected2 = ArgotError::UnknownOption {
        option: "Z".to_string(),
    };

    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[test]
fn return_error_on_int_option_unsafe_value() {
    let parser: &ArgParser = &ARG_PARSER_CELL;

    let input = ["-i", "18446744073709551616"];
    let result: ArgotError = parser.parse(input).unwrap_err();
    let expected = ArgotError::UnsafeInteger {
        value: "18446744073709551616".to_string(),
    };

    assert_eq!(result, expected);
}

#[test]
fn sum_count_option() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["--loglevel=2", "--loglevel=3"];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("loglevel".to_string(), OptionValue::Int(5))
    ]);

    assert_mapping_equals(result.options(), &expected);
}

#[test]
fn concatenate_list_option() {
    let parser: &ArgParser = &ARG_PARSER_CELL;
    let input = ["--tasks=test", "--tasks=build"];
    let result: ParseResult = parser.parse(input).unwrap();

    let expected = HashMap::from([
        ("tasks".to_string(), OptionValue::List(vec![
             "test".to_string(),
             "build".to_string()
        ]))
    ]);

    assert_mapping_equals(result.options(), &expected);
}
