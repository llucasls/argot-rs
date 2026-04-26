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
