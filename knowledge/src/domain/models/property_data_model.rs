use std::collections::HashMap;

pub enum PropertyValueModel {
    Bool(bool),
    String(String),
    Number(f64),
}

pub struct PropertiesDataModel {
    pub values: HashMap<String, PropertyValueModel>,
}
