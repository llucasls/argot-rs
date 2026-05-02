#![cfg(feature = "json")]
use std::collections::HashMap;
use argot_cli::{
    ParserConfig,
    ConfigEntry,
    ConfigEntries,
};
use argot_cli::read_config::json::read_config_file;

#[test]
fn read_json_object() {
    let configs: ParserConfig = read_config_file("tests/config_object.json").unwrap();
    let entries = HashMap::from([
        ("quiet".to_string(), ConfigEntry::Flag),
        ("q".to_string(), ConfigEntry::Alias { target: "quiet".to_string() }),
        ("verbose".to_string(), ConfigEntry::Count),
        ("v".to_string(), ConfigEntry::Alias { target: "verbose".to_string() }),
        ("dry-run".to_string(), ConfigEntry::Flag),
        ("n".to_string(), ConfigEntry::Alias { target: "dry-run".to_string() }),
        ("j".to_string(), ConfigEntry::Int { default: Some(0) }),
        ("browser".to_string(), ConfigEntry::Text { default: None }),
        ("hints".to_string(), ConfigEntry::List { sep: None }),
    ]);
    let expected = ParserConfig::new(ConfigEntries::Map(entries)).unwrap();

    assert_eq!(configs, expected);
}

#[test]
fn read_json_array() {
    let configs: ParserConfig = read_config_file("tests/config_array.json").unwrap();
    let entries = HashMap::from([
        ("quiet".to_string(), ConfigEntry::Flag),
        ("q".to_string(), ConfigEntry::Alias { target: "quiet".to_string() }),
        ("verbose".to_string(), ConfigEntry::Count),
        ("v".to_string(), ConfigEntry::Alias { target: "verbose".to_string() }),
        ("dry-run".to_string(), ConfigEntry::Flag),
        ("n".to_string(), ConfigEntry::Alias { target: "dry-run".to_string() }),
        ("j".to_string(), ConfigEntry::Int { default: Some(0) }),
    ]);
    let expected = ParserConfig::new(ConfigEntries::Map(entries)).unwrap();

    assert_eq!(configs, expected);
}
