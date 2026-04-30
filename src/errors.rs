use std::fmt;
use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArgotError {
    // Parsing errors
    InvalidInt { value: String },
    UnsafeInteger { value: String },
    NullArg { option: String, target: Option<String> },
    NullInt { option: String, target: Option<String> },
    UnknownOption { option: String },

    // Config errors
    InvalidOptionType { tag: String },
    AliasTargetNotFound { option: String, target: String },
    InvalidAliasTarget { option: String, target: String },
    MissingOptionProperty { option: String, property: String },
    MissingOptionType { option: String },
}

impl fmt::Display for ArgotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgotError::InvalidInt { value } =>
                write!(f, "'{}' is not a valid integer", value),
            ArgotError::UnsafeInteger { value } =>
                write!(f, "'{}' is out of range for a signed 64-bit integer", value),
            ArgotError::NullArg { option, target } => {
                match target {
                    Some(target) => {
                        write!(
                            f,
                            "option '{}' (alias for '{}') must take an argument",
                            option,
                            target)
                    },
                    None => {
                        write!(
                            f,
                            "option '{}' must take an argument",
                            option)
                    }
                }
            },
            ArgotError::NullInt { option, target } => {
                match target {
                    Some(target) => {
                        write!(
                            f,
                            "option '{}' (alias for '{}') requires an integer number argument",
                            option,
                            target)
                    },
                    None => {
                        write!(
                            f,
                            "option '{}' requires an integer number argument",
                            option)
                    }
                }
            }
            ArgotError::UnknownOption { option } =>
                write!(f, "option '{}' is not supported", option),
            ArgotError::InvalidOptionType { tag } =>
                write!(f, "option type '{}' is not supported", tag),
            ArgotError::AliasTargetNotFound { option, target } =>
                write!(f,
                    "target value '{}' for option '{}' was not found",
                    target,
                    option),
            ArgotError::InvalidAliasTarget { option, target } =>
                write!(f,
                    "cannot create an alias to another alias ({} => {})",
                    option,
                    target),
            ArgotError::MissingOptionProperty { option, property } =>
                write!(
                    f,
                    "option '{}' is missing required property '{}'",
                    option,
                    property),
            ArgotError::MissingOptionType { option } =>
                write!(
                    f,
                    "option '{}' is missing required property 'type'",
                    option)
        }
    }
}

impl ArgotError {
    pub fn value(&self) -> Option<&str> {
        match self {
            ArgotError::InvalidInt { value } => Some(value),
            ArgotError::UnsafeInteger { value } => Some(value),
            _ => None,
        }
    }

    pub fn option(&self) -> Option<&str> {
        match self {
            ArgotError::NullArg { option, .. } => Some(option),
            ArgotError::NullInt { option, .. } => Some(option),
            ArgotError::UnknownOption { option } => Some(option),
            ArgotError::AliasTargetNotFound { option, .. } => Some(option),
            ArgotError::InvalidAliasTarget { option, .. } => Some(option),
            ArgotError::MissingOptionProperty { option, .. } => Some(option),
            ArgotError::MissingOptionType { option } => Some(option),
            _ => None,
        }
    }

    pub fn target(&self) -> Option<&str> {
        match self {
            ArgotError::NullArg { target, .. } => target.as_deref(),
            ArgotError::NullInt { target, .. } => target.as_deref(),
            _ => None,
        }
    }

    pub fn tag(&self) -> Option<&str> {
        match self {
            ArgotError::InvalidOptionType { tag } => Some(tag),
            _ => None,
        }
    }

    pub fn r#type(&self) -> Option<&str> {
        match self {
            ArgotError::InvalidOptionType { tag } => Some(tag),
            _ => None,
        }
    }

    pub fn property(&self) -> Option<&str> {
        match self {
            ArgotError::MissingOptionProperty { property, .. } => Some(property),
            _ => None,
        }
    }
}

impl std::error::Error for ArgotError {}

impl From<ArgotError> for io::Error {
    fn from(err: ArgotError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, err)
    }
}
