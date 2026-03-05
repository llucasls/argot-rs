use std::ops::Add;

use serde::Serialize;
use serde::ser::{SerializeStruct, SerializeSeq, Serializer};

#[derive(Debug, PartialEq)]
pub enum ConfigEntry {
    Flag,
    Text { default: Option<String> },
    Int { default: Option<i64> },
    Count,
    List { sep: Option<String> },
    Alias { target: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum OptionValue {
    Flag,
    Text(String),
    Int(i64),
    List(Vec<String>),
}

impl<T> Add<T> for OptionValue
where
    T: Into<i64>,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        match self {
            OptionValue::Int(num) => OptionValue::Int(num + other.into()),
            _ => {
                panic!("attempt to add integer to non-int OptionValue variant");
            },
        }
    }
}

impl Serialize for ConfigEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Flag => {
                let name = "FlagEntry";
                let mut entry = serializer.serialize_struct(name, 1)?;
                entry.serialize_field("type", "flag")?;
                entry.end()
            },
            Self::Text { default } => {
                let name = "TextEntry";
                if let Some(value) = default {
                    let mut entry = serializer.serialize_struct(name, 2)?;
                    entry.serialize_field("type", "text")?;
                    entry.serialize_field("default", value)?;
                    entry.end()
                } else {
                    let mut entry = serializer.serialize_struct(name, 1)?;
                    entry.serialize_field("type", "text")?;
                    entry.end()
                }
            },
            Self::Int { default } => {
                let name = "IntEntry";
                if let Some(value) = default {
                    let mut entry = serializer.serialize_struct(name, 2)?;
                    entry.serialize_field("type", "int")?;
                    entry.serialize_field("default", value)?;
                    entry.end()
                } else {
                    let mut entry = serializer.serialize_struct(name, 1)?;
                    entry.serialize_field("type", "int")?;
                    entry.end()
                }
            },
            Self::Count => {
                let name = "CountEntry";
                let mut entry = serializer.serialize_struct(name, 1)?;
                entry.serialize_field("type", "count")?;
                entry.end()
            },
            Self::List { sep } => {
                let name = "ListEntry";
                if let Some(value) = sep {
                    let mut entry = serializer.serialize_struct(name, 2)?;
                    entry.serialize_field("type", "list")?;
                    entry.serialize_field("sep", value)?;
                    entry.end()
                } else {
                    let mut entry = serializer.serialize_struct(name, 1)?;
                    entry.serialize_field("type", "list")?;
                    entry.end()
                }
            },
            Self::Alias { target } => {
                let name = "AliasEntry";
                let mut entry = serializer.serialize_struct(name, 2)?;
                entry.serialize_field("type", "alias")?;
                entry.serialize_field("target", target)?;
                entry.end()
            },
        }
    }
}

impl Serialize for OptionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Flag => serializer.serialize_bool(true),
            Self::Text(text) => serializer.serialize_str(text),
            Self::Int(num) => serializer.serialize_i64(*num),
            Self::List(list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for item in list {
                    seq.serialize_element(item)?;
                }
                seq.end()
            },
        }
    }
}

#[cfg(test)]
mod test_config_entry {
    use super::*;
    use serde_json::{json, to_value, to_string};

    #[test]
    fn serialize_config_entry_flag() {
        let value = to_value(ConfigEntry::Flag).unwrap();
        let expected = json!({ "type": "flag" });
        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_text_none() {
        let value = to_value(ConfigEntry::Text { default: None }).unwrap();
        let expected = json!({
            "type": "text",
        });
        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_text_some() {
        let value = to_value(ConfigEntry::Text {
            default: Some("xdg-open".to_string()),
        }).unwrap();

        let expected = json!({
            "type": "text",
            "default": "xdg-open",
        });

        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_int_none() {
        let value = to_value(ConfigEntry::Int { default: None }).unwrap();

        let expected = json!({
            "type": "int",
        });

        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_int_some() {
        let value = to_value(ConfigEntry::Int { default: Some(42) }).unwrap();

        let expected = json!({
            "type": "int",
            "default": 42,
        });

        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_count() {
        let value = to_value(ConfigEntry::Count).unwrap();
        let expected = json!({ "type": "count" });
        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_list_none() {
        let value = to_value(ConfigEntry::List { sep: None }).unwrap();

        let expected = json!({
            "type": "list",
        });

        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_list_some() {
        let value = to_value(ConfigEntry::List {
            sep: Some(":".to_string()),
        })
        .unwrap();

        let expected = json!({
            "type": "list",
            "sep": ":",
        });

        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_alias() {
        let value = to_value(ConfigEntry::Alias {
            target: "quiet".to_string(),
        })
        .unwrap();

        let expected = json!({
            "type": "alias",
            "target": "quiet",
        });

        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }
}

#[cfg(test)]
mod test_option_value {
    use super::*;
    use serde_json::{json, to_value, to_string};

    #[test]
    fn serialize_option_value_flag() {
        let value = to_value(OptionValue::Flag).unwrap();
        let expected = json!(true);
        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_option_value_text() {
        let value = to_value(OptionValue::Text("build".into())).unwrap();
        let expected = json!("build");
        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_option_value_int() {
        let value = to_value(OptionValue::Int(123)).unwrap();
        let expected = json!(123);
        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_option_value_list() {
        let value = to_value(OptionValue::List(vec![
            "file1.txt".into(),
            "file2.txt".into(),
        ]))
        .unwrap();

        let expected = json!(["file1.txt", "file2.txt"]);

        assert_eq!(to_string(&value).unwrap(), to_string(&expected).unwrap());
        assert_eq!(value, expected);
    }
}
