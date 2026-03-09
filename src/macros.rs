#[macro_export]
macro_rules! entries {
    (
        $( $key:literal => $variant:ident $( { $( $field:ident : $value:expr ),* $(,)? } )? ),* $(,)?
    ) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert(
                $key.into(),
                $crate::entries!(@build $variant $( { $( $field : $value ),* } )?)
            );
        )*
        $crate::arg_parser::ParserConfig::new($crate::types::ConfigEntries::Map(map))
    }};

    (@build Flag $({})?) => {
        $crate::types::ConfigEntry::Flag
    };

    (@build Text $({})?) => {
        $crate::types::ConfigEntry::Text { default: None }
    };

    (@build Text { default: $val:expr }) => {
        $crate::types::ConfigEntry::Text { default: Some($val.into()) }
    };

    (@build Int $({})?) => {
        $crate::types::ConfigEntry::Int { default: None }
    };

    (@build Int { default: $val:expr }) => {
        $crate::types::ConfigEntry::Int { default: Some($val) }
    };

    (@build Count $({})?) => {
        $crate::types::ConfigEntry::Count
    };

    (@build List $({})?) => {
        $crate::types::ConfigEntry::List { sep: None }
    };

    (@build List { sep: $val:expr }) => {
        $crate::types::ConfigEntry::List { sep: Some($val.into()) }
    };

    (@build Alias { target: $val:expr }) => {
        $crate::types::ConfigEntry::Alias { target: $val.into() }
    };

    (@build $unknown:ident $($rest:tt)*) => {
        compile_error!(concat!("unsupported entry type: ", stringify!($unknown)));
    };
}
