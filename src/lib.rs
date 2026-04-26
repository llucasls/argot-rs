pub mod arg_parser;
pub mod parser_config;
pub mod read_config;
pub mod types;
pub mod errors;
pub mod utils;
pub mod macros;

pub use arg_parser::ArgParser;
pub use parser_config::ParserConfig;
pub use types::{ConfigEntry, ConfigEntries, OptionValue};

#[cfg(feature = "json")]
pub use read_config::json::read_config_file as read_json_config;

#[cfg(feature = "toml")]
pub use read_config::toml::read_config_file as read_toml_config;
