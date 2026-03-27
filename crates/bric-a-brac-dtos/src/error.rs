#[derive(Debug, thiserror::Error)]
pub enum DtosConversionError {
    #[error("Invalid UUID")]
    InvalidUuid {
        #[source]
        source: uuid::Error,
    },

    #[error("Invalid timestamp '{value}'")]
    InvalidTimestamp { value: String },

    #[error("Missing value for property '{label}'")]
    MissingPropertyValue { label: String },
}

impl From<uuid::Error> for DtosConversionError {
    fn from(e: uuid::Error) -> Self {
        Self::InvalidUuid { source: e }
    }
}
