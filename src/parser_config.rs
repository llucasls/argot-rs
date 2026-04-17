use std::io;
use std::collections::HashMap;

use crate::types::{ConfigEntry, ConfigEntries, LabeledEntry};

pub struct ParserConfig {
    configs: HashMap<String, ConfigEntry>,
}

impl ParserConfig {
    pub fn new(entries: ConfigEntries) -> io::Result<Self> {
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

        for (name, target) in aliases {
            if !configs.contains_key(&target) {
                let kind = io::ErrorKind::InvalidData;
                let msg = format!(
                    "target value '{}' for option '{}' was not found",
                    target,
                    name
                );
                return Err(io::Error::new(kind, msg));
            }
        }

        Ok(Self { configs })
    }

    pub fn into_inner(self) -> HashMap<String, ConfigEntry> {
        let Self { configs } = self;
        configs
    }
}
