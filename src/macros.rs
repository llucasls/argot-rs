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
        $crate::ParserConfig::new($crate::ConfigEntries::Map(map))
    }};

    (@build Flag $({})?) => {
        $crate::ConfigEntry::Flag
    };

    (@build Text $({})?) => {
        $crate::ConfigEntry::Text { default: None }
    };

    (@build Text { default: $val:expr }) => {
        $crate::ConfigEntry::Text { default: Some($val.into()) }
    };

    (@build Int $({})?) => {
        $crate::ConfigEntry::Int { default: None }
    };

    (@build Int { default: $val:expr }) => {
        $crate::ConfigEntry::Int { default: Some($val) }
    };

    (@build Count $({})?) => {
        $crate::ConfigEntry::Count
    };

    (@build List $({})?) => {
        $crate::ConfigEntry::List { sep: None }
    };

    (@build List { sep: $val:expr }) => {
        $crate::ConfigEntry::List { sep: Some($val.into()) }
    };

    (@build Alias { target: $val:expr }) => {
        $crate::ConfigEntry::Alias { target: $val.into() }
    };

    (@build $unknown:ident $($rest:tt)*) => {
        compile_error!(concat!("unsupported entry type: ", stringify!($unknown)));
    };
}
