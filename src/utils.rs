use std::num::IntErrorKind;

use crate::errors::ArgotError;

pub fn parse_int(value: &str) -> Result<i64, ArgotError> {
    match value.parse() {
        Ok(num) => Ok(num),
        Err(e) => {
            match e.kind() {
                IntErrorKind::PosOverflow =>
                    Err(ArgotError::UnsafeInteger { value: format!("{}", e) }),
                IntErrorKind::NegOverflow =>
                    Err(ArgotError::UnsafeInteger { value: format!("{}", e) }),
                _ => Err(ArgotError::InvalidInt { value: format!("{}", e) }),
            }
        }
    }
}

#[cfg(test)]
mod test {
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
        let output = parse_int(input).unwrap_err();

        let value = "number too large to fit in target type".to_string();
        let expected: ArgotError = ArgotError::UnsafeInteger { value };

        assert_eq!(output, expected);
    }

    #[test]
    fn return_error_on_neg_overflow() {
        let input = "-18446744073709551617";
        let output = parse_int(input).unwrap_err();

        let value = "number too small to fit in target type".to_string();
        let expected: ArgotError = ArgotError::UnsafeInteger { value };

        assert_eq!(output, expected);
    }

    #[test]
    fn return_error_on_empty_input() {
        let input = "";
        let output = parse_int(input).unwrap_err();

        let value = "cannot parse integer from empty string".to_string();
        let expected: ArgotError = ArgotError::InvalidInt { value };

        assert_eq!(output, expected);
    }

    #[test]
    fn return_error_on_invalid_input() {
        let input = "pizza";
        let output = parse_int(input).unwrap_err();

        let value = "invalid digit found in string".to_string();
        let expected: ArgotError = ArgotError::InvalidInt { value };

        assert_eq!(output, expected);
    }

    #[test]
    fn return_error_on_whitespace() {
        let input = " 201 ";
        let output = parse_int(input).unwrap_err();

        let value = "invalid digit found in string".to_string();
        let expected: ArgotError = ArgotError::InvalidInt { value };

        assert_eq!(output, expected);
    }
}
