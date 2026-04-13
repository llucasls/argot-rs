pub mod json;
pub mod toml;

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::{entries, ConfigEntry};

    #[test]
    fn entries_macro() {
        let map: HashMap<String, ConfigEntry> = config! {
            "quiet" => Flag,
            "q" => Alias { target: "quiet" },
            "verbose" => Count,
            "v" => Alias { target: "verbose" },
            "dry-run" => Flag,
            "n" => Alias { target: "dry-run" },
            "j" => Int { default: 0 },
            "browser" => Text,
            "hints" => List,
        }.unwrap().into_inner();

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
}
