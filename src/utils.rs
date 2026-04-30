use std::num::IntErrorKind;

use crate::errors::ArgotError;

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
mod test_utils {
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
