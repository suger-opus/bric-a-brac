use std::collections::HashMap;

#[derive(Debug, derive_more::Display)]
pub enum PropertyData {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug)]
pub struct PropertiesData(pub HashMap<String, PropertyData>);
