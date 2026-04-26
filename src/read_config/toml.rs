#![cfg(feature = "toml")]
use std::fs::read_to_string;
use std::io::{ErrorKind, self};
use std::path::Path;

use serde::Deserialize;
use toml::from_str;

use crate::types::ConfigEntries;
use crate::parser_config::ParserConfig;


#[derive(Deserialize)]
struct Table {
    entries: ConfigEntries
}

pub fn read_config_file<P>(filename: P) -> io::Result<ParserConfig>
where
    P: AsRef<Path>,
{
    let text: String = read_to_string(filename)?;

    let table: Table = from_str(&text)
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

    ParserConfig::new(table.entries).map_err(|res| res.into())
}

#[cfg(test)]
mod test_toml {
    use super::*;
    use std::collections::HashMap;
    use crate::types::ConfigEntry;

    #[test]
    fn read_toml_table() {
        let configs: ParserConfig = read_config_file("config_table.toml").unwrap();
        let map: HashMap<String, ConfigEntry> = configs.into_inner();
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
        let configs: ParserConfig = read_config_file("config_array.toml").unwrap();
        let map: HashMap<String, ConfigEntry> = configs.into_inner();
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
}
