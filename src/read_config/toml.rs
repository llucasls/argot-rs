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
