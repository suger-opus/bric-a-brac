use crate::presentation::errors::TonicError;
use bric_a_brac_dtos::DtosConversionError;
use neo4rs::BoltType;
use tonic::Status;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Conversion(#[from] DtosConversionError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error("Internal error: {context}")]
    Internal { context: String },

    #[error("Not found: {entity}")]
    NotFound { entity: String },

    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        TonicError::from(err).into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database: unknown error")]
    Unknown {
        #[source]
        source: neo4rs::Error,
    },

    #[error("Database: unknown de-error")]
    UnknownDe {
        #[source]
        source: neo4rs::DeError,
    },

    #[error("Database: number conversion error for property '{property_name}' with value {value}")]
    NumberConversion {
        property_name: String,
        value: String,
    },

    #[error("Database: missing id property: {id}")]
    MissingId { id: String },

    #[error("Database: wrong id property: {id}")]
    WrongId { id: String },

    #[error("Database: unreachable property: {property_key}")]
    UnreachableProperty { property_key: String },

    #[error("Database: unlabeled node: {node_data_id}")]
    UnlabeledNode { node_data_id: String },

    #[error("Database: unsupported BoltType: {bolt_type:?}")]
    UnsupportedBoltType { bolt_type: BoltType },

    #[error("Database: unsupported property value: {value:?}")]
    UnsupportedPropertyValue { value: String },

    #[error("Database: no row returned")]
    NoneRow(),

    #[error("Database: invalid depth value: {value} (must be 1-10)")]
    InvalidDepth { value: i32 },
}

impl From<uuid::Error> for DatabaseError {
    fn from(e: uuid::Error) -> Self {
        DatabaseError::WrongId { id: e.to_string() }
    }
}

impl From<neo4rs::Error> for DatabaseError {
    fn from(err: neo4rs::Error) -> Self {
        DatabaseError::Unknown { source: err }
    }
}

impl From<neo4rs::DeError> for DatabaseError {
    fn from(err: neo4rs::DeError) -> Self {
        DatabaseError::UnknownDe { source: err }
    }
}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        DatabaseError::from(err).into()
    }
}
