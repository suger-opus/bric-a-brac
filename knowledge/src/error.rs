use bric_a_brac_protos::knowledge::PropertyValue;
use neo4rs::BoltType;
use tonic::Status;

#[derive(Debug)]
pub enum ApiError {
    MissingId(String),
    WrongId(String),
    UnreachableProperty(String),
    UnlabeledNode(String),
    UnsupportedBoltType(BoltType),
    UnsupportedPropertyValue(PropertyValue),
    NoneRow(),
    UnkownDatabaseError(neo4rs::Error),
    UnkownDatabaseDeError(neo4rs::DeError),
}

impl From<ApiError> for Status {
    fn from(e: ApiError) -> Self {
        tracing::error!(error=?e, "request failed");
        match e {
            ApiError::MissingId(id_key) => {
                Status::internal(format!("Missing id property: {}", id_key))
            }
            ApiError::WrongId(id_key) => Status::internal(format!("Wrong id property: {}", id_key)),
            ApiError::UnreachableProperty(prop_key) => {
                Status::internal(format!("Unreachable property: {}", prop_key))
            }
            ApiError::UnlabeledNode(label_key) => {
                Status::internal(format!("Unlabeled node: {}", label_key))
            }
            ApiError::UnsupportedBoltType(bolt) => {
                Status::internal(format!("Unsupported BoltType: {:?}", bolt))
            }
            ApiError::UnsupportedPropertyValue(value) => {
                Status::invalid_argument(format!("Unsupported PropertyValue: {:?}", value))
            }
            ApiError::NoneRow() => Status::internal("No row returned"),
            ApiError::UnkownDatabaseError(err) => {
                Status::internal(format!("Unknown database error: {:?}", err))
            }
            ApiError::UnkownDatabaseDeError(err) => {
                Status::internal(format!("Unknown database deserialization error: {:?}", err))
            }
        }
    }
}

impl From<neo4rs::Error> for ApiError {
    fn from(e: neo4rs::Error) -> Self {
        ApiError::UnkownDatabaseError(e)
    }
}

impl From<neo4rs::DeError> for ApiError {
    fn from(e: neo4rs::DeError) -> Self {
        ApiError::UnkownDatabaseDeError(e)
    }
}
