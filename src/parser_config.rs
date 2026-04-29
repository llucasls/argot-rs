use std::collections::{HashMap, hash_map};

use crate::errors::ArgotError;
use crate::types::{ConfigEntry, ConfigEntries, LabeledEntry};
use ArgotError::{InvalidAliasTarget, AliasTargetNotFound};

#[derive(Debug, PartialEq)]
pub struct ParserConfig {
    configs: HashMap<String, ConfigEntry>,
}

impl ParserConfig {
    pub fn new(entries: ConfigEntries) -> Result<Self, ArgotError> {
        let size: usize = entries.len();
        let mut aliases: Vec<(String, String)> = Vec::with_capacity(size);
        let mut configs = HashMap::with_capacity(size);
        match entries {
            ConfigEntries::Map(map) => {
                for (option, config) in map {
                    configs.insert(option.to_string(), config.clone());
                    if let ConfigEntry::Alias { target } = config {
                        aliases.push((option.to_string(), target.to_string()));
                    }
                }
            },
            ConfigEntries::List(list) => {
                for LabeledEntry { option, entry } in list {
                    configs.insert(option.to_string(), entry.clone());
                    if let ConfigEntry::Alias { target } = entry {
                        aliases.push((option.to_string(), target.to_string()));
                    }
                }
            },
        };

        for (option, target) in aliases {
            match configs.get(&target) {
                None => {
                    return Err(AliasTargetNotFound { option, target });
                },
                Some(ConfigEntry::Alias { .. }) => {
                    return Err(InvalidAliasTarget { option, target });
                },
                _ => {}
            }
        }

        Ok(Self { configs })
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.configs.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&ConfigEntry> {
        self.configs.get(key)
    }

    pub fn get_key_value(&self, key: &str) -> Option<(&str, &ConfigEntry)> {
        self.configs.get_key_value(key).map(|(k, v)| (k.as_ref(), v))
    }

    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }

    pub fn iter(&self) -> hash_map::Iter<'_, String, ConfigEntry> {
        self.configs.iter()
    }

    pub fn keys(&self) -> hash_map::Keys<'_, String, ConfigEntry> {
        self.configs.keys()
    }

    pub fn len(&self) -> usize {
        self.configs.len()
    }

    pub fn values(&self) -> hash_map::Values<'_, String, ConfigEntry> {
        self.configs.values()
    }
}

#[cfg(test)]
mod test_parser_config {
    use super::*;
    use std::sync::LazyLock;
    use crate::parser_config;

    static PARSER_CONFIG_CELL: LazyLock<ParserConfig> = LazyLock::new(|| {
        parser_config! {
            "strict" => Flag,
            "output" => Text,
            "loglevel" => Count,
            "tasks" => List,
            "u" => Text,
            "U" => Alias { target: "u" },
            "g" => Text,
            "G" => Alias { target: "g" },
            "user" => Alias { target: "u" },
            "group" => Alias { target: "g" },
        }.unwrap()
    });

    #[test]
    fn test_contains_key() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;
        assert!(config.contains_key("strict"));
        assert!(!config.contains_key("quiet"));
    }

    #[test]
    fn test_get() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;
        let expected = Some(ConfigEntry::Text { default: None });
        assert_eq!(config.get("output"), expected.as_ref());
    }

    #[test]
    fn test_get_key_value() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;
        let expected = ("output", &ConfigEntry::Text { default: None });
        assert_eq!(config.get_key_value("output"), Some(expected));
    }

    #[test]
    fn test_is_empty() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;
        let empty_config = ParserConfig::new(ConfigEntries::Map(HashMap::new()))
            .unwrap();
        assert!(!config.is_empty());
        assert!(empty_config.is_empty());
    }

    #[test]
    fn test_iter() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;

        let expected: HashMap<String, ConfigEntry> = HashMap::from([
            ("strict".to_string(), ConfigEntry::Flag),
            ("output".to_string(), ConfigEntry::Text { default: None }),
            ("loglevel".to_string(), ConfigEntry::Count),
            ("tasks".to_string(), ConfigEntry::List { sep: None }),
            ("u".to_string(), ConfigEntry::Text { default: None }),
            ("U".to_string(), ConfigEntry::Alias { target: "u".to_string() }),
            ("g".to_string(), ConfigEntry::Text { default: None }),
            ("G".to_string(), ConfigEntry::Alias { target: "g".to_string() }),
            ("user".to_string(), ConfigEntry::Alias { target: "u".to_string() }),
            ("group".to_string(), ConfigEntry::Alias { target: "g".to_string() }),
        ]);

        for (option, entry) in config.iter() {
            assert_eq!(Some(entry), expected.get(option));
        }
    }

    #[test]
    fn test_keys() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;

        for key in config.keys() {
            assert!(config.contains_key(key));
        }
    }

    #[test]
    fn test_len() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;

        assert_eq!(config.len(), 10);
    }

    #[test]
    fn test_values() {
        let config: &ParserConfig = &PARSER_CONFIG_CELL;
        let values: Vec<ConfigEntry> = Vec::from([
            ConfigEntry::Flag,
            ConfigEntry::Text { default: None },
            ConfigEntry::Count,
            ConfigEntry::List { sep: None },
            ConfigEntry::Text { default: None },
            ConfigEntry::Alias { target: "u".to_string() },
            ConfigEntry::Text { default: None },
            ConfigEntry::Alias { target: "g".to_string() },
            ConfigEntry::Alias { target: "u".to_string() },
            ConfigEntry::Alias { target: "g".to_string() },
        ]);

        for value in config.values() {
            assert!(values.contains(value));
        }
    }

    #[test]
    fn return_error_on_missing_alias_target() {
        let config_result = parser_config! {
            "tasks" => List,
            "T" => Alias { target: "Tasks" },
        };
        let error = config_result.unwrap_err();
        let expected = ArgotError::AliasTargetNotFound {
            option: "T".to_string(),
            target: "Tasks".to_string(),
        };

        assert_eq!(error, expected);
        assert_eq!(format!("{}", error), format!("{}", expected));
    }

    #[test]
    fn return_error_on_invalid_alias_target() {
        let config_result = parser_config! {
            "tasks" => List,
            "Tasks" => Alias { target: "tasks" },
            "T" => Alias { target: "Tasks" },
        };
        let error = config_result.unwrap_err();
        let expected = ArgotError::InvalidAliasTarget {
            option: "T".to_string(),
            target: "Tasks".to_string(),
        };

        assert_eq!(error, expected);
        assert_eq!(format!("{}", error), format!("{}", expected));
    }
}
