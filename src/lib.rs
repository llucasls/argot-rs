pub mod arg_parser;
pub mod read_config;
pub mod types;

pub use arg_parser::ArgParser;
pub use read_config::{read_json_config, read_toml_config};
pub use types::{ConfigEntry, OptionValue};

#[macro_export]
macro_rules! entries {
    (
        $( $key:literal => $variant:ident $( { $( $field:ident : $value:expr ),* $(,)? } )? ),* $(,)?
    ) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert(
                $key.to_string(),
                entries!(@build $variant $( { $( $field : $value ),* } )?)
            );
        )*
        map
    }};

    (@build Flag $({})?) => {
        ConfigEntry::Flag
    };

    (@build Text $({})?) => {
        ConfigEntry::Text { default: None }
    };

    (@build Text { default: $val:expr }) => {
        ConfigEntry::Text { default: Some($val.into()) }
    };

    (@build Int $({})?) => {
        ConfigEntry::Int { default: None }
    };

    (@build Int { default: $val:expr }) => {
        ConfigEntry::Int { default: Some($val) }
    };

    (@build Count $({})?) => {
        ConfigEntry::Count
    };

    (@build List $({})?) => {
        ConfigEntry::List { sep: None }
    };

    (@build List { sep: $val:expr }) => {
        ConfigEntry::List { sep: Some($val.into()) }
    };

    (@build Alias { target: $val:expr }) => {
        ConfigEntry::Alias { target: $val.into() }
    };
}
