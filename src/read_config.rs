use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::{BufReader, ErrorKind, self};
use std::path::Path;

use serde_json::{Value as JsonValue, Map};
use toml::Value as TomlValue;

use crate::types::ConfigEntry;

fn toml_to_json(value: &TomlValue) -> JsonValue {
    match value {
        TomlValue::String(s) => JsonValue::String(s.to_string()),
        TomlValue::Integer(i) => JsonValue::Number((*i).into()),
        TomlValue::Float(f) => {
            serde_json::Number::from_f64(*f)
                .map(JsonValue::Number)
                .unwrap_or(JsonValue::Null)
        }
        TomlValue::Boolean(b) => JsonValue::Bool(*b),
        TomlValue::Datetime(dt) => JsonValue::String(dt.to_string()),
        TomlValue::Array(arr) => {
            JsonValue::Array(arr.iter().map(toml_to_json).collect())
        }
        TomlValue::Table(table) => {
            let map: Map<String, JsonValue> = table
                .into_iter()
                .map(|(k, v)| (k.to_string(), toml_to_json(v)))
                .collect();
            JsonValue::Object(map)
        }
    }
}

fn add_to_dictionary(
    dictionary: &mut HashMap<String, ConfigEntry>,
    key: &str,
    value: &JsonValue
) -> io::Result<()> {
    let value_error = Err(io::Error::new(
        ErrorKind::InvalidData,
        "configuration file has an unexpected structure",
    ));
    let JsonValue::Object(structure) = value else {
        return value_error;
    };
    let Some(JsonValue::String(entry_type)) = structure.get("type") else {
        return value_error;
    };
    let entry_value = match entry_type {
        t if t == "flag" => ConfigEntry::Flag,
        t if t == "text" => {
            let default = match structure.get("default") {
                Some(JsonValue::String(default)) => Some(default.to_string()),
                None => None,
                _ => { return value_error; },
            };
            ConfigEntry::Text { default }
        },
        t if t == "int" => {
            let default = match structure.get("default") {
                Some(JsonValue::Number(default)) => default.as_i64(),
                None => None,
                _ => { return value_error; },
            };
            ConfigEntry::Int { default }
        },
        t if t == "count" => ConfigEntry::Count,
        t if t == "list" => {
            let sep = match structure.get("sep") {
                Some(JsonValue::String(sep)) => Some(sep.to_string()),
                None => None,
                _ => { return value_error; }
            };
            ConfigEntry::List { sep }
        },
        t if t == "alias" => {
            let target: String = match structure.get("target") {
                Some(JsonValue::String(target)) => target.to_string(),
                _ => { return value_error; },
            };
            ConfigEntry::Alias { target }
        }
        _ => { return value_error; },
    };
    dictionary.insert(key.to_string(), entry_value);
    Ok(())
}

fn parse_value(json_value: JsonValue) -> io::Result<HashMap<String, ConfigEntry>> {
    let value_error = Err(io::Error::new(
        ErrorKind::InvalidData,
        "configuration file has an unexpected structure",
    ));

    let mut dictionary: HashMap<String, ConfigEntry> = HashMap::new();
    match json_value {
        JsonValue::Object(map) => {
            for (key, value) in &map {
                add_to_dictionary(&mut dictionary, key, value)?;
            }
        },
        JsonValue::Array(arr) => {
            for value in &arr {
                let Some(JsonValue::String(key)) = value.get("option") else {
                    return value_error;
                };
                add_to_dictionary(&mut dictionary, key, value)?;
            }
        },
        _ => { return value_error; },
    }

    Ok(dictionary)
}

pub fn read_json_config<P>(filename: P) -> io::Result<HashMap<String, ConfigEntry>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let config: JsonValue = serde_json::from_reader(reader)?;
    parse_value(config)
}

pub fn read_toml_config<P>(filename: P) -> io::Result<HashMap<String, ConfigEntry>>
where
    P: AsRef<Path>,
{
    let text = read_to_string(filename)?;

    let table: TomlValue = toml::from_str(&text)
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

    let msg = "'entries' key not found";
    match table.get("entries") {
        Some(config) => parse_value(toml_to_json(config)),
        None => Err(io::Error::new(ErrorKind::InvalidData, msg)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entries;

    #[test]
    fn read_json_object() {
        let map: HashMap<String, ConfigEntry> = read_json_config("config_object.json").unwrap();
        let expected = HashMap::from([
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

        assert_eq!(map, expected);
    }

    #[test]
    fn read_json_array() {
        let map: HashMap<String, ConfigEntry> = read_json_config("config_array.json").unwrap();
        let expected = HashMap::from([
            ("quiet".to_string(), ConfigEntry::Flag),
            ("q".to_string(), ConfigEntry::Alias { target: "quiet".to_string() }),
            ("verbose".to_string(), ConfigEntry::Count),
            ("v".to_string(), ConfigEntry::Alias { target: "verbose".to_string() }),
            ("dry-run".to_string(), ConfigEntry::Flag),
            ("n".to_string(), ConfigEntry::Alias { target: "dry-run".to_string() }),
            ("j".to_string(), ConfigEntry::Int { default: Some(0) }),
        ]);

        assert_eq!(map, expected);
    }

    #[test]
    fn read_toml_table() {
        let map: HashMap<String, ConfigEntry> = read_toml_config("config_table.toml").unwrap();
        let expected = HashMap::from([
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

        assert_eq!(map, expected);
    }

    #[test]
    fn read_toml_array() {
        let map: HashMap<String, ConfigEntry> = read_toml_config("config_array.toml").unwrap();
        let expected = HashMap::from([
            ("quiet".to_string(), ConfigEntry::Flag),
            ("q".to_string(), ConfigEntry::Alias { target: "quiet".to_string() }),
            ("verbose".to_string(), ConfigEntry::Count),
            ("v".to_string(), ConfigEntry::Alias { target: "verbose".to_string() }),
            ("dry-run".to_string(), ConfigEntry::Flag),
            ("n".to_string(), ConfigEntry::Alias { target: "dry-run".to_string() }),
            ("j".to_string(), ConfigEntry::Int { default: Some(0) }),
        ]);

        assert_eq!(map, expected);
    }

    #[test]
    fn entries_macro() {
        let map: HashMap<String, ConfigEntry> = entries! {
            "quiet" => Flag,
            "q" => Alias { target: "quiet" },
            "verbose" => Count,
            "v" => Alias { target: "verbose" },
            "dry-run" => Flag,
            "n" => Alias { target: "dry-run" },
            "j" => Int { default: 0 },
            "browser" => Text,
            "hints" => List,
        };

        let expected = HashMap::from([
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

        assert_eq!(map, expected);
    }
}
