#![cfg(feature = "json")]
use std::fs::File;
use std::io::{BufReader, self};
use std::path::Path;

use crate::types::ConfigEntries;
use crate::parser_config::ParserConfig;

pub fn read_config_file<P>(filename: P) -> io::Result<ParserConfig>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let configs: ConfigEntries = serde_json::from_reader(reader)?;

    ParserConfig::new(configs)
}

#[cfg(test)]
mod test_json {
    use super::*;
    use std::collections::HashMap;
    use crate::types::ConfigEntry;

    #[test]
    fn read_json_object() {
        let configs: ParserConfig = read_config_file("config_object.json").unwrap();
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
    fn read_json_array() {
        let configs: ParserConfig = read_config_file("config_array.json").unwrap();
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
