/// Generates a type-safe ID wrapper around `UUIDv7`.
///
/// Creates a newtype struct that wraps a UUID, providing type safety to prevent
/// mixing different ID types. IDs are generated using `UUIDv7` by default, which
/// includes timestamp information and is sortable.
///
/// # Generated Implementations
///
/// The macro generates:
/// - Standard traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`
/// - Serialization: `Serialize`, `Deserialize` (serde)
/// - Database: `sqlx::Type` (`PostgreSQL` UUID mapping)
/// - Display and parsing: `Display`, `FromStr`, `TryFrom<&str>`
/// - Access: `AsRef<Uuid>`, `Deref` to inner UUID
///
/// # Example
///
/// ```
/// use bric_a_brac_id::id;
///
/// id!(UserId);
/// id!(OrderId);
///
/// let user_id = UserId::new();
/// let order_id = OrderId::new();
///
/// // Type safety: this won't compile
/// // fn get_user(id: UserId) {}
/// // get_user(order_id); // ❌ Compile error!
///
/// // String parsing
/// let parsed: UserId = "bd70feb5-accc-47dc-a97d-3c152ae5a1ef".try_into().unwrap();
///
/// // Access inner UUID
/// let uuid: &uuid::Uuid = user_id.as_ref();
/// ```
#[macro_export]
macro_rules! id {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::sqlx::Type,
            ::derive_more::From,
        )]
        #[sqlx(transparent)]
        pub struct $name(::uuid::Uuid);

        impl $name {
            pub fn nil() -> Self {
                Self(::uuid::Uuid::nil())
            }
        }

        impl $name {
            #[allow(clippy::new_without_default)]
            #[must_use]
            pub fn new() -> Self {
                Self(::uuid::Uuid::now_v7())
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let id = ::uuid::Uuid::parse_str(s)?;
                Ok(Self(id))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::std::convert::AsRef<::uuid::Uuid> for $name {
            fn as_ref(&self) -> &::uuid::Uuid {
                &self.0
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = ::uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::TryFrom<&str> for $name {
            type Error = ::uuid::Error;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                let id = ::uuid::Uuid::parse_str(s)?;
                Ok(Self(id))
            }
        }

        impl $name {
            /// Extracts the timestamp from the UUIDv7.
            ///
            /// Returns `Some(SystemTime)` if this is a valid UUIDv7 with timestamp information,
            /// or `None` if the UUID is not v7 or lacks timestamp data.
            #[must_use]
            pub fn timestamp(&self) -> Option<::std::time::SystemTime> {
                self.0
                    .get_timestamp()
                    .and_then(|ts| {
                        let (seconds, nanos) = ts.to_unix();
                        ::std::time::UNIX_EPOCH.checked_add(::std::time::Duration::new(seconds, nanos))
                    })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    id!(TestId);

    #[test]
    fn test_create() {
        let id1 = TestId::new();
        let id2 = TestId::new();
        assert_ne!(id1, id2, "two created id are !=");
    }

    #[test]
    fn test_should_parse() {
        let input = "bd70feb5-accc-47dc-a97d-3c152ae5a1ef";
        let result = input.parse::<TestId>().expect("should parse uuid");
        assert_eq!(result.to_string(), input);
    }

    #[test]
    fn test_should_serde() {
        let json = "\"bd70feb5-accc-47dc-a97d-3c152ae5a1ef\"";
        let id = serde_json::from_str::<TestId>(json).expect("valid id");
        let result = serde_json::to_string(&id).expect("valid json");
        assert_eq!(result.to_string(), json);
    }

    #[test]
    fn test_default() {
        let id1 = TestId::default();
        let id2 = TestId::nil();
        assert_ne!(id1, id2, "default id is NOT nil");
    }

    #[test]
    fn test_as_ref() {
        let id = TestId::new();
        let uuid_ref: &uuid::Uuid = id.as_ref();
        assert_eq!(uuid_ref.to_string(), id.to_string());
    }

    #[test]
    fn test_deref() {
        let id = TestId::new();
        let version = id.get_version();
        assert_eq!(version, Some(uuid::Version::SortRand));
    }

    #[test]
    fn test_try_from_str() {
        let input = "bd70feb5-accc-47dc-a97d-3c152ae5a1ef";
        let result: Result<TestId, _> = input.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), input);
    }

    #[test]
    fn test_try_from_str_invalid() {
        let input = "not-a-uuid";
        let result: Result<TestId, _> = input.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_timestamp() {
        let id = TestId::new();
        let timestamp = id.timestamp();
        assert!(timestamp.is_some(), "UUIDv7 should have timestamp");
        
        let now = std::time::SystemTime::now();
        let ts = timestamp.unwrap();
        
        // Timestamp should be very recent (within last second)
        let duration = now.duration_since(ts).expect("timestamp should be in the past");
        assert!(duration.as_secs() < 1, "timestamp should be very recent");
    }

    #[test]
    fn test_timestamp_nil() {
        let id = TestId::nil();
        let timestamp = id.timestamp();
        // Nil UUID won't have v7 timestamp
        assert!(timestamp.is_none() || timestamp.unwrap() == std::time::UNIX_EPOCH);
    }
}
