#[derive(Debug, thiserror::Error)]
pub enum DtosConversionError {
    #[error("Uuid conversion error")]
    Uuid {
        #[source]
        source: uuid::Error,
    },

    #[error("Invalid enum value for {name}: {value}")]
    Enum { name: String, value: i32 },

    #[error("Invalid timestamp: {value}")]
    Timestamp { value: String },

    #[error("Invalid number value for property {property_name}: {value}")]
    Number { property_name: String, value: f64 },

    #[error("Missing value for property: {property_name}")]
    NoPropertyValue { property_name: String },

    #[error("Missing field: {field_name}")]
    NoField { field_name: String },
}

impl From<uuid::Error> for DtosConversionError {
    fn from(e: uuid::Error) -> Self {
        DtosConversionError::Uuid { source: e }
    }
}
