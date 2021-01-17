use std::str::FromStr;

use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;

/// The email address of a user.
#[derive(Debug, Serialize, FromSql)]
pub struct Email(String);

/// Errors that can occur when parsing an string into an `Email`.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseEmailError {
    #[error("The email address was blank")]
    Blank,
}

impl FromStr for Email {
    type Err = ParseEmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(Self::Err::Blank)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl ToSql for Email {
    accepts!(TEXT, VARCHAR);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use test_case::test_case;

    use super::*;

    #[test_case("",  ParseEmailError::Blank  ; "blank string")]
    #[test_case("  ",  ParseEmailError::Blank  ; "whitespace only")]
    fn parse_email_given_invalid_input_then_error(input: &str, err: ParseEmailError) {
        let parsed = Email::from_str(input);

        let_assert!(Err(e) = parsed);
        check!(e == err);
    }

    #[test_case("test@example.com",  "test@example.com"  ; "simple string")]
    #[test_case("  test@example.com",  "test@example.com"  ; "left-padded")]
    #[test_case("test@example.com  ",  "test@example.com"  ; "right-padded")]
    #[test_case("  test@example.com  ",  "test@example.com"  ; "both-padded")]
    fn parse_email_given_valid_input_then_success(input: &str, output: &str) {
        let parsed = Email::from_str(input);

        let_assert!(Ok(e) = parsed);
        check!(e.0 == output);
    }
}
