use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArgotError {
    InvalidInt(String),
    NullArg(String),
    NullInt(String),
    ConfigsNotFound,
    TargetNotFound(String),
    Alias2Alias,
    TypeNotSupported(String),
    OptionNotSupported(String),
    EmptyList,
    Generic(String),
}

impl fmt::Display for ArgotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgotError::InvalidInt(value) =>
                write!(f, "'{}' is not a valid number", value),
            ArgotError::NullArg(name) =>
                write!(f, "option '{}' must take an argument", name),
            ArgotError::NullInt(name) =>
                write!(f, "option '{}' requires a numeric argument", name),
            ArgotError::ConfigsNotFound =>
                write!(f, "no configuration options were found"),
            ArgotError::TargetNotFound(name) =>
                write!(f, "alias target '{}' was not found", name),
            ArgotError::Alias2Alias =>
                write!(f, "cannot create an alias to another alias"),
            ArgotError::TypeNotSupported(name) =>
                write!(f, "type '{}' is not supported", name),
            ArgotError::OptionNotSupported(name) =>
                write!(f, "option '{}' is not supported", name),
            ArgotError::EmptyList =>
                write!(f, "arg_list cannot be empty"),
            ArgotError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ArgotError {}
