use std::collections::HashMap;
use std::ops::Add;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use serde::ser::{SerializeStruct, Serializer};

#[cfg(feature = "serde")]
type SResult<S> = Result<<S as Serializer>::Ok, <S as Serializer>::Error>;

#[cfg(feature = "serde")]
fn serialize_text_entry<S>(default: &Option<String>, serializer: S) -> SResult<S>
where
    S: Serializer
{
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
}

#[cfg(feature = "serde")]
fn serialize_int_entry<S>(default: &Option<i64>, serializer: S) -> SResult<S>
where
    S: Serializer
{
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
}

#[cfg(feature = "serde")]
fn serialize_list_entry<S>(sep: &Option<String>, serializer: S) -> SResult<S>
where
    S: Serializer,
{
    let name = "ListEntry";
    if let Some(value) = sep {
        let mut entry = serializer.serialize_struct(name, 2)?;
        entry.serialize_field("type", "list")?;
        entry.serialize_field("sep", &value)?;
        entry.end()
    } else {
        let mut entry = serializer.serialize_struct(name, 1)?;
        entry.serialize_field("type", "list")?;
        entry.end()
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "type", rename_all = "lowercase"))]
pub enum ConfigEntry {
    Flag,

    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_text_entry"))]
    Text { default: Option<String> },

    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_int_entry"))]
    Int { default: Option<i64> },

    Count,

    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_list_entry"))]
    List { sep: Option<String> },

    Alias { target: String },
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct LabeledEntry {
    pub option: String,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub entry: ConfigEntry,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(untagged))]
pub enum ConfigEntries {
    Map(HashMap<String, ConfigEntry>),
    List(Vec<LabeledEntry>),
}

impl ConfigEntries {
    pub fn len(&self) -> usize {
        match self {
            Self::Map(map) => map.len(),
            Self::List(list) => list.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod test_config_entries {
    use super::*;

    #[test]
    fn detect_empty_entries() {
        let entries = ConfigEntries::Map(HashMap::new());
        assert!(entries.is_empty());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CliArg {
    /// A GNU-style long option
    Long { name: String, value: Option<String> },

    /// A Unix-style short option
    Short { flags: String },

    /// A name=value parameter assignment
    Parameter(String, String),

    /// A positional argument
    Operand,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum OptionValue {
    #[cfg_attr(feature = "serde", serde(with = "serde_flag"))]
    Flag,
    Text(String),
    Int(i64),
    List(Vec<String>),
}

impl OptionValue {
    pub fn unwrap_text(self) -> String {
        match self {
            Self::Flag => {
                panic!("called `OptionValue::unwrap_text` on a Flag value");
            },
            Self::Text(text) => text,
            Self::Int(_) => {
                panic!("called `OptionValue::unwrap_text` on an Int value: {:?}", self);
            },
            Self::List(_) => {
                panic!("called `OptionValue::unwrap_text` on a List value: {:?}", self);
            },
        }
    }

    pub fn unwrap_int(self) -> i64 {
        match self {
            Self::Flag => {
                panic!("called `OptionValue::unwrap_int` on a Flag value");
            },
            Self::Text(_) => {
                panic!("called `OptionValue::unwrap_int` on a Text value: {:?}", self);
            },
            Self::Int(int) => int,
            Self::List(_) => {
                panic!("called `OptionValue::unwrap_int` on a List value: {:?}", self);
            },
        }
    }

    pub fn unwrap_list(self) -> Vec<String> {
        match self {
            Self::Flag => {
                panic!("called `OptionValue::unwrap_list` on a Flag value");
            },
            Self::Text(_) => {
                panic!("called `OptionValue::unwrap_list` on a Text value: {:?}", self);
            },
            Self::Int(_) => {
                panic!("called `OptionValue::unwrap_list` on an Int value: {:?}", self);
            },
            Self::List(list) => list,
        }
    }
}

impl<T> Add<T> for OptionValue
where
    T: Into<i64>,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        match self {
            OptionValue::Int(num) => OptionValue::Int(num + other.into()),
            _ => { unreachable!(); },
        }
    }
}

#[cfg(test)]
mod test_option_value {
    use super::*;

    #[test]
    fn add_int_option_value() {
        let value = OptionValue::Int(27);
        let expected = OptionValue::Int(43);
        assert_eq!(value + 16, expected);
    }

    #[test]
    fn test_unwrap_text() {
        let value = OptionValue::Text("doc.txt".to_string());
        let expected = "doc.txt".to_string();
        assert_eq!(value.unwrap_text(), expected);
    }

    #[test]
    fn test_unwrap_int() {
        let value = OptionValue::Int(14);
        let expected = 14;
        assert_eq!(value.unwrap_int(), expected);
    }

    #[test]
    fn test_unwrap_list() {
        let value = OptionValue::List(vec![
            "build".to_string(),
            "test".to_string(),
        ]);
        let expected = vec![
            "build".to_string(),
            "test".to_string(),
        ];
        assert_eq!(value.unwrap_list(), expected);
    }

    #[test]
    #[should_panic]
    fn test_unwrap_text_on_flag_value() {
        let value = OptionValue::Flag;
        value.unwrap_text();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_text_on_int_value() {
        let value = OptionValue::Int(14);
        value.unwrap_text();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_text_on_list_value() {
        let value = OptionValue::List(vec![
            "build".to_string(),
            "test".to_string(),
        ]);
        value.unwrap_text();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_int_on_flag_value() {
        let value = OptionValue::Flag;
        value.unwrap_int();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_int_on_text_value() {
        let value = OptionValue::Text("doc.txt".to_string());
        value.unwrap_int();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_int_on_list_value() {
        let value = OptionValue::List(vec![
            "build".to_string(),
            "test".to_string(),
        ]);
        value.unwrap_int();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_list_on_flag_value() {
        let value = OptionValue::Flag;
        value.unwrap_list();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_list_on_text_value() {
        let value = OptionValue::Text("doc.txt".to_string());
        value.unwrap_list();
    }

    #[test]
    #[should_panic]
    fn test_unwrap_list_on_int_value() {
        let value = OptionValue::Int(14);
        value.unwrap_list();
    }
}

#[cfg(feature = "serde")]
mod serde_flag {
    use std::fmt;
    use serde::ser::Serializer;
    use serde::de::{self, Deserializer, Visitor};

    use super::SResult;

    pub fn serialize<S>(serializer: S) -> SResult<S>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }

    struct FlagVisitor;

    impl<'de> Visitor<'de> for FlagVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean true")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value {
                Ok(())
            } else {
                Err(E::custom("flag cannot be false"))
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bool(FlagVisitor)
    }
}

#[cfg(all(test, feature = "json"))]
mod test_config_entry_serialize {
    use super::*;
    use serde_json::{json, to_value};

    #[test]
    fn serialize_config_entry_flag() {
        let value = to_value(ConfigEntry::Flag).unwrap();
        let expected = json!({ "type": "flag" });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_text_none() {
        let value = to_value(ConfigEntry::Text { default: None }).unwrap();
        let expected = json!({ "type": "text" });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_text_some() {
        let value = to_value(ConfigEntry::Text {
            default: Some("xdg-open".to_string()),
        }).unwrap();
        let expected = json!({ "type": "text", "default": "xdg-open" });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_int_none() {
        let value = to_value(ConfigEntry::Int { default: None }).unwrap();
        let expected = json!({ "type": "int" });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_int_some() {
        let value = to_value(ConfigEntry::Int { default: Some(42) }).unwrap();
        let expected = json!({ "type": "int", "default": 42 });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_count() {
        let value = to_value(ConfigEntry::Count).unwrap();
        let expected = json!({ "type": "count" });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_list_none() {
        let value = to_value(ConfigEntry::List { sep: None }).unwrap();
        let expected = json!({ "type": "list" });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_list_some() {
        let value = to_value(ConfigEntry::List {
            sep: Some(":".to_string()),
        }).unwrap();
        let expected = json!({ "type": "list", "sep": ":" });

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_config_entry_alias() {
        let value = to_value(ConfigEntry::Alias {
            target: "quiet".to_string(),
        }).unwrap();
        let expected = json!({ "type": "alias", "target": "quiet" });

        assert_eq!(value, expected);
    }
}

#[cfg(all(test, feature = "json"))]
mod test_config_entry_deserialize {
    use super::*;
    use serde_json::{json, from_value};

    #[test]
    fn deserialize_config_entry_flag() {
        let value = json!({ "type": "flag" });
        let entry: ConfigEntry = from_value(value).unwrap();

        assert_eq!(entry, ConfigEntry::Flag);
    }

    #[test]
    fn deserialize_config_entry_text_none() {
        let value = json!({ "type": "text" });
        let entry: ConfigEntry = from_value(value).unwrap();

        assert_eq!(entry, ConfigEntry::Text { default: None });
    }

    #[test]
    fn deserialize_config_entry_text_some() {
        let value = json!({ "type": "text", "default": "all" });
        let entry: ConfigEntry = from_value(value).unwrap();

        let default = Some("all".to_string());
        assert_eq!(entry, ConfigEntry::Text { default });
    }

    #[test]
    fn deserialize_config_entry_int_none() {
        let value = json!({ "type": "int" });
        let entry: ConfigEntry = from_value(value).unwrap();

        assert_eq!(entry, ConfigEntry::Int { default: None });
    }

    #[test]
    fn deserialize_config_entry_int_some() {
        let value = json!({ "type": "int", "default": 12 });
        let entry: ConfigEntry = from_value(value).unwrap();

        let default = Some(12);
        assert_eq!(entry, ConfigEntry::Int { default });
    }

    #[test]
    fn deserialize_config_entry_count() {
    }

    #[test]
    fn deserialize_config_entry_list_none() {
        let value = json!({ "type": "list" });
        let entry: ConfigEntry = from_value(value).unwrap();

        assert_eq!(entry, ConfigEntry::List { sep: None });
    }

    #[test]
    fn deserialize_config_entry_list_some() {
        let value = json!({ "type": "list", "sep": ":" });
        let entry: ConfigEntry = from_value(value).unwrap();

        let sep = Some(":".to_string());
        assert_eq!(entry, ConfigEntry::List { sep });
    }

    #[test]
    fn deserialize_config_entry_alias() {
        let value = json!({ "type": "alias", "target": "quiet" });
        let entry: ConfigEntry = from_value(value).unwrap();

        let target = "quiet".to_string();
        assert_eq!(entry, ConfigEntry::Alias { target });
    }

    #[test]
    fn deserialize_config_entries_object() {
        let value = json!({
            "quiet": { "type": "flag" },
            "q": { "type": "alias", "target": "quiet" },
            "verbose": { "type": "count" },
            "v": { "type": "alias", "target": "verbose" },
            "j": { "type": "int", "default": 0 },
            "browser": { "type": "text" },
            "hints": { "type": "list" },
        });
        let configs: HashMap<String, ConfigEntry> = from_value(value).unwrap();
        let expected = HashMap::from([
            ("quiet".to_string(), ConfigEntry::Flag),
            ("q".to_string(), ConfigEntry::Alias { target: "quiet".to_string() }),
            ("verbose".to_string(), ConfigEntry::Count),
            ("v".to_string(), ConfigEntry::Alias { target: "verbose".to_string() }),
            ("j".to_string(), ConfigEntry::Int { default: Some(0) }),
            ("browser".to_string(), ConfigEntry::Text { default: None }),
            ("hints".to_string(), ConfigEntry::List { sep: None }),
        ]);

        assert_eq!(configs, expected);
    }

    #[test]
    fn deserialize_config_entries_array() {
        let value = json!([
            { "option": "quiet", "type": "flag" },
            { "option": "q", "type": "alias", "target": "quiet" },
            { "option": "verbose", "type": "count" },
            { "option": "v", "type": "alias", "target": "verbose" },
            { "option": "j", "type": "int", "default": 0 },
            { "option": "browser", "type": "text" },
            { "option": "hints", "type": "list" },
        ]);
        let configs: Vec<LabeledEntry> = from_value(value).unwrap();
        let expected = vec![
            LabeledEntry {
                option: "quiet".to_string(),
                entry: ConfigEntry::Flag
            },
            LabeledEntry {
                option: "q".to_string(),
                entry: ConfigEntry::Alias { target: "quiet".to_string() }
            },
            LabeledEntry {
                option: "verbose".to_string(),
                entry: ConfigEntry::Count
            },
            LabeledEntry {
                option: "v".to_string(),
                entry: ConfigEntry::Alias { target: "verbose".to_string() }
            },
            LabeledEntry {
                option: "j".to_string(),
                entry: ConfigEntry::Int { default: Some(0) }
            },
            LabeledEntry {
                option: "browser".to_string(),
                entry: ConfigEntry::Text { default: None }
            },
            LabeledEntry {
                option: "hints".to_string(),
                entry: ConfigEntry::List { sep: None }
            },
        ];

        assert_eq!(configs, expected);
    }

    #[test]
    fn deserialize_config_entries_object_into_wrapper() {
        let value = json!({
            "quiet": { "type": "flag" },
            "q": { "type": "alias", "target": "quiet" },
            "verbose": { "type": "count" },
            "v": { "type": "alias", "target": "verbose" },
            "j": { "type": "int", "default": 0 },
            "browser": { "type": "text" },
            "hints": { "type": "list" },
        });

        let configs: ConfigEntries = from_value(value).unwrap();
        let map = HashMap::from([
            ("quiet".to_string(), ConfigEntry::Flag),
            ("q".to_string(), ConfigEntry::Alias { target: "quiet".to_string() }),
            ("verbose".to_string(), ConfigEntry::Count),
            ("v".to_string(), ConfigEntry::Alias { target: "verbose".to_string() }),
            ("j".to_string(), ConfigEntry::Int { default: Some(0) }),
            ("browser".to_string(), ConfigEntry::Text { default: None }),
            ("hints".to_string(), ConfigEntry::List { sep: None }),
        ]);
        let expected = ConfigEntries::Map(map);

        assert_eq!(configs, expected);
    }

    #[test]
    fn deserialize_config_entries_array_into_wrapper() {
        let value = json!([
            { "option": "quiet", "type": "flag" },
            { "option": "q", "type": "alias", "target": "quiet" },
            { "option": "verbose", "type": "count" },
            { "option": "v", "type": "alias", "target": "verbose" },
            { "option": "j", "type": "int", "default": 0 },
            { "option": "browser", "type": "text" },
            { "option": "hints", "type": "list" },
        ]);
        let configs: ConfigEntries = from_value(value).unwrap();
        let list = vec![
            LabeledEntry {
                option: "quiet".to_string(),
                entry: ConfigEntry::Flag
            },
            LabeledEntry {
                option: "q".to_string(),
                entry: ConfigEntry::Alias { target: "quiet".to_string() }
            },
            LabeledEntry {
                option: "verbose".to_string(),
                entry: ConfigEntry::Count
            },
            LabeledEntry {
                option: "v".to_string(),
                entry: ConfigEntry::Alias { target: "verbose".to_string() }
            },
            LabeledEntry {
                option: "j".to_string(),
                entry: ConfigEntry::Int { default: Some(0) }
            },
            LabeledEntry {
                option: "browser".to_string(),
                entry: ConfigEntry::Text { default: None }
            },
            LabeledEntry {
                option: "hints".to_string(),
                entry: ConfigEntry::List { sep: None }
            },
        ];
        let expected = ConfigEntries::List(list);

        assert_eq!(configs, expected);
    }
}

#[cfg(all(test, feature = "json"))]
mod test_option_value_serialize {
    use super::*;
    use serde_json::{json, to_value};

    #[test]
    fn serialize_option_value_flag() {
        let value = to_value(OptionValue::Flag).unwrap();
        let expected = json!(true);

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_option_value_text() {
        let value = to_value(OptionValue::Text("build".into())).unwrap();
        let expected = json!("build");

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_option_value_int() {
        let value = to_value(OptionValue::Int(123)).unwrap();
        let expected = json!(123);

        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_option_value_list() {
        let value = to_value(OptionValue::List(vec![
            "file1.txt".into(),
            "file2.txt".into(),
        ])).unwrap();
        let expected = json!(["file1.txt", "file2.txt"]);

        assert_eq!(value, expected);
    }
}

#[cfg(all(test, feature = "json"))]
mod test_option_value_deserialize {
    use super::*;
    use serde_json::{json, from_value};

    #[test]
    fn deserialize_option_value_flag() {
        let input = json!(true);
        let output: OptionValue = from_value(input).unwrap();
        let expected = OptionValue::Flag;

        assert_eq!(output, expected);
    }

    #[test]
    fn deserialize_option_value_text() {
        let input = json!("install");
        let output: OptionValue = from_value(input).unwrap();
        let expected = OptionValue::Text("install".to_string());

        assert_eq!(output, expected);
    }

    #[test]
    fn deserialize_option_value_int() {
        let input = json!(7);
        let output: OptionValue = from_value(input).unwrap();
        let expected = OptionValue::Int(7);

        assert_eq!(output, expected);
    }

    #[test]
    fn deserialize_option_value_list() {
        let input = json!(["all", "install"]);
        let output: OptionValue = from_value(input).unwrap();
        let expected = OptionValue::List(vec![
            "all".to_string(),
            "install".to_string(),
        ]);

        assert_eq!(output, expected);
    }

    #[test]
    fn deserialize_option_values_object() {
        let input = json!({
            "verbose": 2,
            "dry-run": true,
            "browser": "chromium",
            "hints": ["test", "ui"]
        });
        let output: HashMap<String, OptionValue> = from_value(input).unwrap();
        let expected = HashMap::from([
            ("verbose".to_string(), OptionValue::Int(2)),
            ("dry-run".to_string(), OptionValue::Flag),
            ("browser".to_string(), OptionValue::Text("chromium".to_string())),
            (
                "hints".to_string(),
                OptionValue::List(vec!["test".to_string(), "ui".to_string()]),
            ),
        ]);

        assert_eq!(output, expected);
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParseResult {
    options: HashMap<String, OptionValue>,
    parameters: HashMap<String, String>,
    operands: Vec<String>,
}

impl ParseResult {
    pub fn new(
        options: HashMap<String, OptionValue>,
        parameters: HashMap<String, String>,
        operands: Vec<String>,
    ) -> Self {
        Self { options, parameters, operands }
    }

    pub fn options(&self) -> &HashMap<String, OptionValue> {
        &self.options
    }

    pub fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }

    pub fn operands(&self) -> &[String] {
        &self.operands
    }
}
