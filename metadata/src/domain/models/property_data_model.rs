use std::collections::HashMap;

#[derive(Debug, Clone, derive_more::Display)]
pub enum PropertyData {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct PropertiesData(pub HashMap<String, PropertyData>);
