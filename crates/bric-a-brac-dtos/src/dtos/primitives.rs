use rand::RngExt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use utoipa::{PartialSchema, ToSchema};
use validator::Validate;

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[allow(clippy::expect_used)]
static COLOR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#[0-9A-Fa-f]{6}$").expect("Invalid color regex"));
#[allow(clippy::expect_used)]
static KEY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9]{7}$").expect("Invalid key regex"));

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, derive_more::Display)]
#[display("{value}")]
#[serde(transparent)]
pub struct LabelDto {
    #[validate(length(min = 1, max = 25))]
    value: String,
}

impl LabelDto {
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl PartialSchema for LabelDto {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::SchemaType::new(
                utoipa::openapi::schema::Type::String,
            ))
            .min_length(Some(1))
            .max_length(Some(25))
            .build()
            .into()
    }
}

impl ToSchema for LabelDto {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("LabelDto")
    }
}

impl From<String> for LabelDto {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl From<LabelDto> for String {
    fn from(s: LabelDto) -> Self {
        s.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, derive_more::Display)]
#[display("{value}")]
#[serde(transparent)]
pub struct ColorDto {
    #[validate(regex(path = "*COLOR_REGEX"))]
    value: String,
}

impl ColorDto {
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl PartialSchema for ColorDto {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::SchemaType::new(
                utoipa::openapi::schema::Type::String,
            ))
            .pattern(Some("^#[0-9A-Fa-f]{6}$"))
            .build()
            .into()
    }
}

impl ToSchema for ColorDto {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ColorDto")
    }
}

impl From<String> for ColorDto {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl From<ColorDto> for String {
    fn from(s: ColorDto) -> Self {
        s.value
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Validate,
    derive_more::Display,
)]
#[display("{value}")]
#[serde(transparent)]
pub struct KeyDto {
    #[validate(regex(path = "*KEY_REGEX"))]
    value: String,
}

impl KeyDto {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let first = *LETTERS.get(rng.random_range(0..52)).unwrap_or(&b'a') as char;
        let rest: String = (0..7)
            .map(|_| *BASE62.get(rng.random_range(0..62)).unwrap_or(&b'0') as char)
            .collect();
        Self {
            value: format!("{first}{rest}"),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl PartialSchema for KeyDto {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::SchemaType::new(
                utoipa::openapi::schema::Type::String,
            ))
            .pattern(Some("^[a-zA-Z][a-zA-Z0-9]{7}$"))
            .build()
            .into()
    }
}

impl ToSchema for KeyDto {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("KeyDto")
    }
}

impl From<String> for KeyDto {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl From<KeyDto> for String {
    fn from(s: KeyDto) -> Self {
        s.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_dto_new_format() {
        let key = KeyDto::new();
        let s = key.as_str();
        assert_eq!(s.len(), 8);
        assert!(s.chars().next().unwrap().is_ascii_alphabetic());
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_key_dto_new_uniqueness() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for _ in 0..1000 {
            assert!(
                set.insert(KeyDto::new().to_string()),
                "Duplicate key generated"
            );
        }
    }
}
