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
                Some(ConfigEntry::Alias { target: t }) => {
                    let option = target;
                    let target = t.to_string();
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
