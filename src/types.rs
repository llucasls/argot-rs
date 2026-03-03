#[derive(Debug, PartialEq)]
pub enum ConfigEntry {
    Flag,
    Text { default: Option<String> },
    Int { default: Option<i64> },
    Count,
    List { sep: Option<String> },
    Alias { target: String },
}
