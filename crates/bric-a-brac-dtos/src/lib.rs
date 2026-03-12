mod dtos;
mod error;
mod openapi;
mod utils;

// !! TODO: improve all validations (key, label, colors, ...) and call validation on each side
pub use dtos::*;
pub use error::DtosConversionError;
pub use openapi::generate_graph_schema_doc;
