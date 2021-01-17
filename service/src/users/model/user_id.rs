use std::str::FromStr;

use uuid::Uuid;

/// The ID of a user.
#[derive(Debug, PartialEq)]
pub struct UserID(Uuid);

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Errors that can occur when parsing an string into an `UserID`.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseUserIDError {
    #[error("The UserID was blank")]
    Blank,
    #[error("The UserID was malformed")]
    Malformed,
}

impl FromStr for UserID {
    type Err = ParseUserIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(Self::Err::Blank)
        } else {
            let uuid = Uuid::parse_str(trimmed).map_err(|e| {
                tracing::debug!(err = ?e, input = s, "Failed to parse User ID");
                Self::Err::Malformed
            })?;

            Ok(Self(uuid))
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use test_case::test_case;

    use super::*;

    #[test_case("",  ParseUserIDError::Blank  ; "blank string")]
    #[test_case("  ",  ParseUserIDError::Blank  ; "whitespace only")]
    #[test_case("notAUUID",  ParseUserIDError::Malformed  ; "not a uuid")]
    #[test_case("9766f4af-f2de-4f19-8326-9f856e829d4h",  ParseUserIDError::Malformed  ; "invalid character")]
    fn parse_user_id_given_invalid_input_then_error(input: &str, err: ParseUserIDError) {
        let parsed = UserID::from_str(input);

        let_assert!(Err(e) = parsed);
        check!(e == err);
    }

    #[test_case("9766f4af-f2de-4f19-8326-9f856e829d46",  "9766f4af-f2de-4f19-8326-9f856e829d46"  ; "simple string")]
    #[test_case("  9766f4af-f2de-4f19-8326-9f856e829d46",  "9766f4af-f2de-4f19-8326-9f856e829d46"  ; "left-padded")]
    #[test_case("9766f4af-f2de-4f19-8326-9f856e829d46  ",  "9766f4af-f2de-4f19-8326-9f856e829d46"  ; "right-padded")]
    #[test_case("  9766f4af-f2de-4f19-8326-9f856e829d46  ",  "9766f4af-f2de-4f19-8326-9f856e829d46"  ; "both-padded")]
    #[test_case("9766f4aff2de4f1983269f856e829d46",  "9766f4af-f2de-4f19-8326-9f856e829d46"  ; "no hyphens")]
    fn parse_user_id_given_valid_input_then_success(input: &str, output: &str) {
        let parsed = UserID::from_str(input);

        let_assert!(Ok(e) = parsed);
        check!(e.0.to_string() == output);
    }
}
