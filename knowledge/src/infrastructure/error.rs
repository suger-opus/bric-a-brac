use neo4rs::BoltType;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Unknown database error")]
    Unknown {
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Corrupted number '{label}' with value '{value}'")]
    CorruptedNumber { label: String, value: String },

    #[error("Corrupted state: missing id '{label}'")]
    CorruptedIdState { label: String },

    #[error("Corrupted id '{label}' with value '{value}'")]
    CorruptedId {
        label: String,
        value: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Corrupted state: missing property '{label}'")]
    CorruptedPropertyState { label: String },

    #[error("Corrupted property '{label}' with value '{value:?}'")]
    CorruptedProperty { label: String, value: BoltType },

    #[error("Corrupted state: missing node label with id '{node_data_id}'")]
    CorruptedNodeLabelState { node_data_id: String },

    #[error("No affected or returned rows by database")]
    NoRows(),

    #[error("Node not found at index {index} in path")]
    NodeNotFoundInPath { index: usize },

    #[error("Database connection error")]
    ConnectionError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Database authentication error")]
    AuthenticationError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl From<uuid::Error> for DatabaseError {
    fn from(e: uuid::Error) -> Self {
        Self::CorruptedId {
            label: "uuid".to_owned(),
            value: e.to_string(),
            source: Some(Box::new(e)),
        }
    }
}

impl From<neo4rs::Error> for DatabaseError {
    fn from(err: neo4rs::Error) -> Self {
        match &err {
            neo4rs::Error::AuthenticationError(_) => Self::AuthenticationError {
                source: Box::new(err),
            },
            neo4rs::Error::Neo4j(neo_err) => match neo_err.kind() {
                neo4rs::Neo4jErrorKind::Client(neo4rs::Neo4jClientErrorKind::Security(_)) => {
                    Self::AuthenticationError {
                        source: Box::new(err),
                    }
                }
                neo4rs::Neo4jErrorKind::Transient
                | neo4rs::Neo4jErrorKind::Client(
                    neo4rs::Neo4jClientErrorKind::SessionExpired
                    | neo4rs::Neo4jClientErrorKind::FatalDiscovery,
                ) => Self::ConnectionError {
                    source: Box::new(err),
                },
                _ => Self::Unknown {
                    source: Some(Box::new(err)),
                },
            },
            neo4rs::Error::IOError { .. }
            | neo4rs::Error::ConnectionError
            | neo4rs::Error::UrlParseError(_)
            | neo4rs::Error::UnsupportedScheme(_)
            | neo4rs::Error::InvalidDnsName(_)
            | neo4rs::Error::InvalidConfig
            | neo4rs::Error::UnsupportedVersion(_) => Self::ConnectionError {
                source: Box::new(err),
            },
            _ => Self::Unknown {
                source: Some(Box::new(err)),
            },
        }
    }
}

impl From<neo4rs::DeError> for DatabaseError {
    fn from(err: neo4rs::DeError) -> Self {
        // All DeError variants represent shape mismatches when deserializing
        // data from the database — the stored data doesn't match the expected
        // structure. This is always a corrupted/unexpected database state.
        Self::Unknown {
            source: Some(Box::new(err)),
        }
    }
}
