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

    ParserConfig::new(configs).map_err(|res| res.into())
}
