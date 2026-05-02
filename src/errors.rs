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
                    "cannot create an alias to another alias ('{}' => '{}')",
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

#[cfg(test)]
mod test_errors {
    use super::*;
    use ArgotError::*;

    #[test]
    fn test_invalid_int() {
        let value = "12L".to_string();
        let error = InvalidInt { value };
        let expected = "'12L' is not a valid integer";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn test_unsafe_integer() {
        let value = "18446744073709551616".to_string();
        let error = UnsafeInteger { value };
        let expected = "'18446744073709551616' is out of range for a signed 64-bit integer";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn test_null_arg() {
        {
            let option = "t".to_string();
            let target = Some("tasks".to_string());
            let error = NullArg { option, target };
            let expected = "option 't' (alias for 'tasks') must take an argument";
            assert_eq!(format!("{}", error), expected);
        }
        {
            let option = "t".to_string();
            let error = NullArg { option, target: None };
            let expected = "option 't' must take an argument";
            assert_eq!(format!("{}", error), expected);
        }
    }

    #[test]
    fn test_null_int() {
        {
            let option = "j".to_string();
            let target = Some("jobs".to_string());
            let error = NullInt { option, target };
            let expected = "option 'j' (alias for 'jobs') requires an integer number argument";
            assert_eq!(format!("{}", error), expected);
        }
        {
            let option = "j".to_string();
            let error = NullInt { option, target: None };
            let expected = "option 'j' requires an integer number argument";
            assert_eq!(format!("{}", error), expected);
        }
    }

    #[test]
    fn test_unknown_option() {
        let option = "j".to_string();
        let error = UnknownOption { option };
        let expected = "option 'j' is not supported";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn test_invalid_option_type() {
        let tag = "string".to_string();
        let error = InvalidOptionType { tag };
        let expected = "option type 'string' is not supported";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn test_alias_target_not_found() {
        let option = "x".to_string();
        let target = "xtrace".to_string();
        let error = AliasTargetNotFound { option, target };
        let expected = "target value 'xtrace' for option 'x' was not found";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn test_invalid_alias_target() {
        let option = "X".to_string();
        let target = "x".to_string();
        let error = InvalidAliasTarget { option, target };
        let expected = "cannot create an alias to another alias ('X' => 'x')";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn test_missing_option_property() {
        let option = "retries".to_string();
        let property = "min".to_string();
        let error = MissingOptionProperty { option, property };
        let expected = "option 'retries' is missing required property 'min'";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn test_missin_option_type() {
        let option = "a".to_string();
        let error = MissingOptionType { option };
        let expected = "option 'a' is missing required property 'type'";
        assert_eq!(format!("{}", error), expected);
    }

    #[test]
    fn get_value() {
        let error1 = InvalidInt { value: " 21".to_string() };
        let error2 = UnsafeInteger { value: "18446744073709551616".to_string() };
        let error3 = NullArg { option: "x".to_string(), target: None };

        assert_eq!(error1.value(), Some(" 21"));
        assert_eq!(error2.value(), Some("18446744073709551616"));
        assert_eq!(error3.value(), None);
    }

    #[test]
    fn get_option() {
        let error1 = NullArg { option: "b".to_string(), target: None };
        let error2 = NullInt { option: "i".to_string(), target: None };
        let error3 = UnknownOption { option: "a".to_string() };
        let error4 = AliasTargetNotFound { option: "a".to_string(), target: "alpha".to_string() };
        let error5 = InvalidAliasTarget { option: "A".to_string(), target: "a".to_string() };
        let error6 = MissingOptionProperty { option: "v".to_string(), property: "target".to_string() };
        let error7 = MissingOptionType { option: "version".to_string() };
        let error8 = InvalidInt { value: "	3".to_string() };

        assert_eq!(error1.option(), Some("b"));
        assert_eq!(error2.option(), Some("i"));
        assert_eq!(error3.option(), Some("a"));
        assert_eq!(error4.option(), Some("a"));
        assert_eq!(error5.option(), Some("A"));
        assert_eq!(error6.option(), Some("v"));
        assert_eq!(error7.option(), Some("version"));
        assert_eq!(error8.option(), None);
    }

    #[test]
    fn get_target() {
        let error1 = NullArg { option: "u".to_string(), target: Some("user".to_string()) };
        let error2 = NullInt { option: "i".to_string(), target: Some("id".to_string()) };
        let error3 = InvalidInt { value: "	4".to_string() };

        assert_eq!(error1.target(), Some("user"));
        assert_eq!(error2.target(), Some("id"));
        assert_eq!(error3.target(), None);
    }

    #[test]
    fn get_type() {
        let error1 = InvalidOptionType { tag: "number".to_string() };
        let error2 = InvalidAliasTarget { option: "A".to_string(), target: "a".to_string() };

        assert_eq!(error1.r#type(), Some("number"));
        assert_eq!(error2.r#type(), None);

        // alias method
        assert_eq!(error1.tag(), Some("number"));
        assert_eq!(error2.tag(), None);
    }

    #[test]
    fn get_property() {
        let error1 = MissingOptionProperty { option: "m".to_string(), property: "target".to_string() };
        let error2 = MissingOptionType { option: "n".to_string() };

        assert_eq!(error1.property(), Some("target"));
        assert_eq!(error2.property(), None);
    }

    #[test]
    fn get_io_error_from_argot_error() {
        let error: io::Error = InvalidInt { value: "0o777".to_string() }.into();

        let kind = io::ErrorKind::InvalidData;

        assert_eq!(error.kind(), kind);
        assert_eq!(format!("{}", error), "'0o777' is not a valid integer");
    }
}
