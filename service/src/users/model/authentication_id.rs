use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The identity of the user at the authentication service.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationId(String);

/// Errors that can occur when parsing an string into an `AuthenticationId`.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseAuthenticationIdError {
    #[error("The authentication ID was blank")]
    Blank,
}

impl FromStr for AuthenticationId {
    type Err = ParseAuthenticationIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(Self::Err::Blank)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use test_case::test_case;

    use super::*;

    #[test_case("",  &ParseAuthenticationIdError::Blank  ; "blank string")]
    #[test_case("  ",  &ParseAuthenticationIdError::Blank  ; "whitespace only")]
    fn parse_authentication_id_given_invalid_input_then_error(
        input: &str,
        err: &ParseAuthenticationIdError,
    ) {
        let parsed = AuthenticationId::from_str(input);

        let_assert!(Err(e) = parsed);
        check!(&e == err);
    }

    #[test_case("15612523",  "15612523"  ; "simple string")]
    #[test_case("  15612523",  "15612523"  ; "left-padded")]
    #[test_case("15612523  ",  "15612523"  ; "right-padded")]
    #[test_case("  15612523  ",  "15612523"  ; "both-padded")]
    fn parse_authentication_id_given_valid_input_then_success(input: &str, output: &str) {
        let parsed = AuthenticationId::from_str(input);

        let_assert!(Ok(e) = parsed);
        check!(e.0 == output);
    }
}
