use std::num::IntErrorKind;

use crate::errors::ArgotError;
use crate::types::CliArg;

pub fn get_opt_value(arg: &str) -> CliArg {
    if arg == "-" || arg == "--" {
        CliArg::Operand
    } else if let Some(stripped) = arg.strip_prefix("--") {
        let mut parts = stripped.splitn(2, '=');
        let name = parts.next().unwrap_or_default().to_string();
        let value = parts.next().map(|v| v.to_string());
        if name.is_empty() {
            CliArg::Operand
        } else {
            CliArg::Long { name, value }
        }
    } else if let Some(flags) = arg.strip_prefix('-') {
        CliArg::Short { flags: flags.to_string() }
    } else {
        let mut parts = arg.splitn(2, '=');
        let name: String = parts.next().unwrap_or_default().to_string();
        if name.is_empty() {
            CliArg::Operand
        } else if let Some(value) = parts.next().map(|v: &str| v.to_string()) {
            CliArg::Parameter(name, value)
        } else {
            CliArg::Operand
        }
    }
}

pub fn parse_int(value: &str) -> Result<i64, ArgotError> {
    match value.parse() {
        Ok(num) => Ok(num),
        Err(e) => {
            match e.kind() {
                IntErrorKind::PosOverflow =>
                    Err(ArgotError::UnsafeInteger { value: value.to_string() }),
                IntErrorKind::NegOverflow =>
                    Err(ArgotError::UnsafeInteger { value: value.to_string() }),
                _ => Err(ArgotError::InvalidInt { value: format!("{}", e) }),
            }
        }
    }
}

#[cfg(test)]
mod test_parse_int {
    use super::*;

    #[test]
    fn parse_integer() {
        let input = "64";
        let output = parse_int(input).unwrap();

        let expected: i64 = 64;

        assert_eq!(output, expected);
    }

    #[test]
    fn return_error_on_pos_overflow() {
        let input = "18446744073709551616";
        let error = parse_int(input).unwrap_err();

        let value = "18446744073709551616".to_string();
        let expected: ArgotError = ArgotError::UnsafeInteger { value };

        assert_eq!(error, expected);
        assert_eq!(format!("{}", error), format!("{}", expected));
    }

    #[test]
    fn return_error_on_neg_overflow() {
        let input = "-18446744073709551617";
        let error = parse_int(input).unwrap_err();

        let value = "-18446744073709551617".to_string();
        let expected: ArgotError = ArgotError::UnsafeInteger { value };

        assert_eq!(error, expected);
        assert_eq!(format!("{}", error), format!("{}", expected));
    }

    #[test]
    fn return_error_on_empty_input() {
        let input = "";
        let error = parse_int(input).unwrap_err();

        let value = "cannot parse integer from empty string".to_string();
        let expected: ArgotError = ArgotError::InvalidInt { value };

        assert_eq!(error, expected);
        assert_eq!(format!("{}", error), format!("{}", expected));
    }

    #[test]
    fn return_error_on_invalid_input() {
        let input = "pizza";
        let error = parse_int(input).unwrap_err();

        let value = "invalid digit found in string".to_string();
        let expected: ArgotError = ArgotError::InvalidInt { value };

        assert_eq!(error, expected);
        assert_eq!(format!("{}", error), format!("{}", expected));
    }

    #[test]
    fn return_error_on_whitespace() {
        let input = " 201 ";
        let error = parse_int(input).unwrap_err();

        let value = "invalid digit found in string".to_string();
        let expected: ArgotError = ArgotError::InvalidInt { value };

        assert_eq!(error, expected);
        assert_eq!(format!("{}", error), format!("{}", expected));
    }
}

#[cfg(test)]
mod test_get_opt_value {
    use super::*;

    #[test]
    fn return_operand_on_single_dash() {
        let input = "-";
        let output = get_opt_value(input);
        let expected = CliArg::Operand;
        assert_eq!(output, expected);
    }

    #[test]
    fn return_operand_on_double_dash() {
        let input = "--";
        let output = get_opt_value(input);
        let expected = CliArg::Operand;
        assert_eq!(output, expected);
    }

    #[test]
    fn return_long_option() {
        let input = "--user=jim";
        let output = get_opt_value(input);
        let expected = CliArg::Long {
            name: "user".to_string(),
            value: Some("jim".to_string()),
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn return_short_option() {
        let input = "-ubob";
        let output = get_opt_value(input);
        let expected = CliArg::Short {
            flags: "ubob".to_string()
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn return_parameter() {
        let input = "name=alice";
        let output = get_opt_value(input);
        let expected = CliArg::Parameter("name".into(), "alice".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn return_operand_on_empty_long_option_name() {
        let input = "--=jim";
        let output = get_opt_value(input);
        let expected = CliArg::Operand;
        assert_eq!(output, expected);
    }

    #[test]
    fn return_operand_on_empty_parameter_name() {
        let input = "=bianca";
        let output = get_opt_value(input);
        let expected = CliArg::Operand;
        assert_eq!(output, expected);
    }

    #[test]
    fn return_operand() {
        let input = "maria";
        let output = get_opt_value(input);
        let expected = CliArg::Operand;
        assert_eq!(output, expected);
    }
}
