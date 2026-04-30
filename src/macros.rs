#[macro_export]
macro_rules! parser_config {
    (
        $( $key:literal => $variant:ident $( { $( $field:ident : $value:expr ),* $(,)? } )? ),* $(,)?
    ) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert(
                $key.into(),
                $crate::parser_config!(@build $key, $variant $( { $( $field : $value ),* } )?)
            );
        )*
        $crate::parser_config::ParserConfig::new($crate::types::ConfigEntries::Map(map))
    }};

    (@build $key:literal, Flag $({})?) => {
        $crate::types::ConfigEntry::Flag
    };

    (@build $key:literal, Text $({})?) => {
        $crate::types::ConfigEntry::Text { default: None }
    };

    (@build $key:literal, Text { default: $val:expr }) => {
        $crate::types::ConfigEntry::Text { default: Some($val.into()) }
    };

    (@build $key:literal, Int $({})?) => {
        $crate::types::ConfigEntry::Int { default: None }
    };

    (@build $key:literal, Int { default: $val:expr }) => {
        $crate::types::ConfigEntry::Int { default: Some($val) }
    };

    (@build $key:literal, Count $({})?) => {
        $crate::types::ConfigEntry::Count
    };

    (@build $key:literal, List $({})?) => {
        $crate::types::ConfigEntry::List { sep: None }
    };

    (@build $key:literal, List { sep: $val:expr }) => {
        $crate::types::ConfigEntry::List { sep: Some($val.into()) }
    };

    (@build $key:literal, Alias { target: $val:expr }) => {
        $crate::types::ConfigEntry::Alias { target: $val.into() }
    };

    (@build $key:literal, Alias $({})?) => {
        compile_error!(
            concat!("Alias option \"",
                $key,
                "\" requires a target"))
    };

    (@build $key:literal, $unknown:ident $($rest:tt)*) => {
        compile_error!(
            concat!("unsupported entry type: ",
                stringify!($unknown),
                "\nvalid options must be one of: Flag, Text, Int, Count, List, Alias"))
    };
}
