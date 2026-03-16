use super::{CreateGraphDataDto, GraphSchemaDto, PropertySchemaDto, PropertyTypeDto};
use validator::Validate;

/// A single schema compliance violation with a human- and AI-readable message.
///
/// Each variant carries enough context for an AI model to understand exactly what went
/// wrong and how to fix it in the next generation attempt.
#[derive(Debug, thiserror::Error)]
pub enum SchemaComplianceError {
    #[error(
        "Node #{index}: type '{key}' is not defined in the schema. \
        Valid node types: [{valid_keys}]. \
        Fix: use one of the valid type keys."
    )]
    UnknownNodeType {
        index: usize,
        key: String,
        valid_keys: String,
    },

    #[error("Validation failed: {0}")]
    Validation(validator::ValidationErrors),

    #[error(
        "Edge #{index}: type '{key}' is not defined in the schema. \
        Valid edge types: [{valid_keys}]. \
        Fix: use one of the valid type keys."
    )]
    UnknownEdgeType {
        index: usize,
        key: String,
        valid_keys: String,
    },

    #[error(
        "{element}: required property '{prop_key}' (label: '{prop_label}') is missing. \
        Fix: add a '{prop_key}' column in the corresponding CSV section."
    )]
    MissingProperty {
        element: String,
        prop_key: String,
        prop_label: String,
    },

    #[error(
        "{element}: property '{prop_key}' is not defined in the schema for this type. \
        Valid properties: [{valid_props}]. \
        Fix: remove the '{prop_key}' column."
    )]
    UnknownProperty {
        element: String,
        prop_key: String,
        valid_props: String,
    },

    #[error(
        "{element}: property '{prop_key}' has wrong type — \
        expected {expected_type}, got {actual_type} (value: {actual_value}). \
        Fix: provide a raw {expected_type} value, not a {actual_type}."
    )]
    WrongPropertyType {
        element: String,
        prop_key: String,
        expected_type: String,
        actual_type: String,
        actual_value: String,
    },

    #[error(
        "{element}: property '{prop_key}' (label: '{prop_label}') \
        has invalid select value '{value}'. Valid options: [{valid_options}]. \
        Fix: use one of the listed options exactly as written."
    )]
    InvalidSelectOption {
        element: String,
        prop_key: String,
        prop_label: String,
        value: String,
        valid_options: String,
    },
}

impl CreateGraphDataDto {
    /// Validates that every node and edge instance conforms to the given `schema`.
    ///
    /// All violations are collected before returning so the caller sees the full picture
    /// in one shot — ideal for feeding back to an AI model in a retry loop.
    ///
    /// Returns `Ok(())` when there are no violations.
    /// Returns `Err(errors)` with a non-empty list otherwise.
    pub fn validate_against_schema(
        &self,
        schema: &GraphSchemaDto,
    ) -> Result<(), Vec<SchemaComplianceError>> {
        self.validate()
            .map_err(|e| vec![SchemaComplianceError::Validation(e)])?;

        let mut errors: Vec<SchemaComplianceError> = Vec::new();

        let valid_node_keys = schema
            .nodes
            .iter()
            .map(|n| format!("'{}'", n.key))
            .collect::<Vec<_>>()
            .join(", ");

        let valid_edge_keys = schema
            .edges
            .iter()
            .map(|e| format!("'{}'", e.key))
            .collect::<Vec<_>>()
            .join(", ");

        for (i, node) in self.nodes.iter().enumerate() {
            let index = i + 1;
            let node_key = node.key.to_string();

            match schema.nodes.iter().find(|s| s.key == node.key) {
                None => {
                    errors.push(SchemaComplianceError::UnknownNodeType {
                        index,
                        key: node_key,
                        valid_keys: valid_node_keys.clone(),
                    });
                    // Cannot validate properties without a matching schema entry
                }
                Some(node_schema) => {
                    let element = format!("Node #{index} (type '{node_key}')");
                    let valid_props = node_schema
                        .properties
                        .iter()
                        .map(|p| format!("'{}'", p.key))
                        .collect::<Vec<_>>()
                        .join(", ");

                    // Check for extra properties that are not in the schema
                    for prop_key in node.properties.values.keys() {
                        if !node_schema
                            .properties
                            .iter()
                            .any(|p| p.key.as_str() == prop_key.as_str())
                        {
                            errors.push(SchemaComplianceError::UnknownProperty {
                                element: element.clone(),
                                prop_key: prop_key.clone(),
                                valid_props: valid_props.clone(),
                            });
                        }
                    }

                    // Check for missing properties and value type / option violations
                    for prop_schema in &node_schema.properties {
                        let prop_key = prop_schema.key.as_str();
                        match node.properties.values.get(prop_key) {
                            None => errors.push(SchemaComplianceError::MissingProperty {
                                element: element.clone(),
                                prop_key: prop_key.to_string(),
                                prop_label: prop_schema.label.to_string(),
                            }),
                            Some(value) => {
                                collect_property_errors(&mut errors, &element, prop_schema, value);
                            }
                        }
                    }
                }
            }
        }

        for (i, edge) in self.edges.iter().enumerate() {
            let index = i + 1;
            let edge_key = edge.key.to_string();

            match schema.edges.iter().find(|s| s.key == edge.key) {
                None => {
                    errors.push(SchemaComplianceError::UnknownEdgeType {
                        index,
                        key: edge_key,
                        valid_keys: valid_edge_keys.clone(),
                    });
                }
                Some(edge_schema) => {
                    let element = format!("Edge #{index} (type '{edge_key}')");
                    let valid_props = edge_schema
                        .properties
                        .iter()
                        .map(|p| format!("'{}'", p.key))
                        .collect::<Vec<_>>()
                        .join(", ");

                    for prop_key in edge.properties.values.keys() {
                        if !edge_schema
                            .properties
                            .iter()
                            .any(|p| p.key.as_str() == prop_key.as_str())
                        {
                            errors.push(SchemaComplianceError::UnknownProperty {
                                element: element.clone(),
                                prop_key: prop_key.clone(),
                                valid_props: valid_props.clone(),
                            });
                        }
                    }

                    for prop_schema in &edge_schema.properties {
                        let prop_key = prop_schema.key.as_str();
                        match edge.properties.values.get(prop_key) {
                            None => errors.push(SchemaComplianceError::MissingProperty {
                                element: element.clone(),
                                prop_key: prop_key.to_string(),
                                prop_label: prop_schema.label.to_string(),
                            }),
                            Some(value) => {
                                collect_property_errors(&mut errors, &element, prop_schema, value);
                            }
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Appends any type or option violations for a single property value into `errors`.
fn collect_property_errors(
    errors: &mut Vec<SchemaComplianceError>,
    element: &str,
    prop_schema: &PropertySchemaDto,
    value: &serde_json::Value,
) {
    let prop_key = prop_schema.key.as_str();

    let type_ok = match &prop_schema.property_type {
        PropertyTypeDto::Number => matches!(value, serde_json::Value::Number(_)),
        PropertyTypeDto::String => matches!(value, serde_json::Value::String(_)),
        PropertyTypeDto::Boolean => matches!(value, serde_json::Value::Bool(_)),
        PropertyTypeDto::Select => matches!(value, serde_json::Value::String(_)),
    };

    if !type_ok {
        errors.push(SchemaComplianceError::WrongPropertyType {
            element: element.to_string(),
            prop_key: prop_key.to_string(),
            expected_type: expected_type_name(&prop_schema.property_type).to_string(),
            actual_type: json_type_name(value).to_string(),
            actual_value: value.to_string(),
        });
        // No point checking select options if the type is already wrong
        return;
    }

    if let PropertyTypeDto::Select = &prop_schema.property_type {
        if let serde_json::Value::String(s) = value {
            if let Some(options) = &prop_schema.metadata.options {
                let valid: Vec<String> = options.iter().map(|o| o.to_string()).collect();
                if !valid.iter().any(|o| o == s) {
                    errors.push(SchemaComplianceError::InvalidSelectOption {
                        element: element.to_string(),
                        prop_key: prop_key.to_string(),
                        prop_label: prop_schema.label.to_string(),
                        value: s.clone(),
                        valid_options: valid
                            .iter()
                            .map(|o| format!("'{}'", o))
                            .collect::<Vec<_>>()
                            .join(", "),
                    });
                }
            }
        }
    }
}

fn json_type_name(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Bool(_) => "Boolean",
        serde_json::Value::Number(_) => "Number",
        serde_json::Value::String(_) => "String",
        serde_json::Value::Array(_) => "Array",
        serde_json::Value::Object(_) => "Object",
        serde_json::Value::Null => "Null",
    }
}

fn expected_type_name(pt: &PropertyTypeDto) -> &'static str {
    match pt {
        PropertyTypeDto::Number => "Number",
        PropertyTypeDto::String | PropertyTypeDto::Select => "String",
        PropertyTypeDto::Boolean => "Boolean",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ColorDto, CreateEdgeDataDto, CreateNodeDataDto, EdgeSchemaDto, EdgeSchemaIdDto, GraphIdDto,
        KeyDto, LabelDto, NodeDataIdDto, NodeSchemaDto, NodeSchemaIdDto, PropertiesDataDto,
        PropertyMetadataDto, PropertySchemaDto, PropertySchemaIdDto,
    };
    use chrono::Utc;
    use std::collections::HashMap;
    use std::str::FromStr;

    fn make_node_schema(key: &str, properties: Vec<PropertySchemaDto>) -> NodeSchemaDto {
        NodeSchemaDto {
            node_schema_id: NodeSchemaIdDto::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap(),
            graph_id: GraphIdDto::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            label: LabelDto::from("Person".to_string()),
            key: KeyDto::from(key.to_string()),
            color: ColorDto::from("#FF0000".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            properties,
        }
    }

    fn make_edge_schema(key: &str, properties: Vec<PropertySchemaDto>) -> EdgeSchemaDto {
        EdgeSchemaDto {
            edge_schema_id: EdgeSchemaIdDto::from_str("00000000-0000-0000-0000-000000000003")
                .unwrap(),
            graph_id: GraphIdDto::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            label: LabelDto::from("Knows".to_string()),
            key: KeyDto::from(key.to_string()),
            color: ColorDto::from("#00FF00".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            properties,
        }
    }

    fn make_property_schema(key: &str, label: &str, pt: PropertyTypeDto) -> PropertySchemaDto {
        PropertySchemaDto {
            property_schema_id: PropertySchemaIdDto::from_str(
                "00000000-0000-0000-0000-000000000004",
            )
            .unwrap(),
            node_schema_id: None,
            edge_schema_id: None,
            label: LabelDto::from(label.to_string()),
            key: KeyDto::from(key.to_string()),
            property_type: pt,
            metadata: PropertyMetadataDto { options: None },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn make_select_property_schema(
        key: &str,
        label: &str,
        options: Vec<&str>,
    ) -> PropertySchemaDto {
        PropertySchemaDto {
            property_schema_id: PropertySchemaIdDto::from_str(
                "00000000-0000-0000-0000-000000000005",
            )
            .unwrap(),
            node_schema_id: None,
            edge_schema_id: None,
            label: LabelDto::from(label.to_string()),
            key: KeyDto::from(key.to_string()),
            property_type: PropertyTypeDto::Select,
            metadata: PropertyMetadataDto {
                options: Some(options.into_iter().map(|s| s.to_string().into()).collect()),
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn node_id() -> NodeDataIdDto {
        NodeDataIdDto::new()
    }

    fn schema_with_node(node_key: &str, props: Vec<PropertySchemaDto>) -> GraphSchemaDto {
        GraphSchemaDto {
            nodes: vec![make_node_schema(node_key, props)],
            edges: vec![],
        }
    }

    #[test]
    fn valid_data_passes() {
        let schema = schema_with_node(
            "per12345",
            vec![
                make_property_schema("nme12345", "Name", PropertyTypeDto::String),
                make_property_schema("age12345", "Age", PropertyTypeDto::Number),
            ],
        );
        let data = CreateGraphDataDto {
            nodes: vec![CreateNodeDataDto {
                node_data_id: node_id(),
                key: KeyDto::from("per12345".to_string()),
                properties: PropertiesDataDto {
                    values: HashMap::from([
                        (
                            "nme12345".to_string(),
                            serde_json::Value::String("Alice".to_string()),
                        ),
                        (
                            "age12345".to_string(),
                            serde_json::Value::Number(serde_json::Number::from(30)),
                        ),
                    ]),
                },
            }],
            edges: vec![],
        };
        assert!(data.validate_against_schema(&schema).is_ok());
    }

    #[test]
    fn unknown_node_type_is_reported() {
        let schema = schema_with_node("per12345", vec![]);
        let data = CreateGraphDataDto {
            nodes: vec![CreateNodeDataDto {
                node_data_id: node_id(),
                key: KeyDto::from("xyz00000".to_string()),
                properties: PropertiesDataDto {
                    values: HashMap::new(),
                },
            }],
            edges: vec![],
        };
        let errors = data.validate_against_schema(&schema).unwrap_err();
        assert!(errors.len() == 1);
        assert!(matches!(
            errors[0],
            SchemaComplianceError::UnknownNodeType { .. }
        ));
        assert!(errors[0].to_string().contains("per12345"));
    }

    #[test]
    fn missing_property_is_reported() {
        let schema = schema_with_node(
            "per12345",
            vec![make_property_schema(
                "nme12345",
                "Name",
                PropertyTypeDto::String,
            )],
        );
        let data = CreateGraphDataDto {
            nodes: vec![CreateNodeDataDto {
                node_data_id: node_id(),
                key: KeyDto::from("per12345".to_string()),
                properties: PropertiesDataDto {
                    values: HashMap::new(),
                },
            }],
            edges: vec![],
        };
        let errors = data.validate_against_schema(&schema).unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, SchemaComplianceError::MissingProperty { .. })));
        assert!(errors[0].to_string().contains("nme12345"));
    }

    #[test]
    fn unknown_property_is_reported() {
        let schema = schema_with_node("per12345", vec![]);
        let data = CreateGraphDataDto {
            nodes: vec![CreateNodeDataDto {
                node_data_id: node_id(),
                key: KeyDto::from("per12345".to_string()),
                properties: PropertiesDataDto {
                    values: HashMap::from([(
                        "ghost000".to_string(),
                        serde_json::Value::String("?".to_string()),
                    )]),
                },
            }],
            edges: vec![],
        };
        let errors = data.validate_against_schema(&schema).unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, SchemaComplianceError::UnknownProperty { .. })));
    }

    #[test]
    fn wrong_property_type_is_reported() {
        let schema = schema_with_node(
            "per12345",
            vec![make_property_schema(
                "age12345",
                "Age",
                PropertyTypeDto::Number,
            )],
        );
        let data = CreateGraphDataDto {
            nodes: vec![CreateNodeDataDto {
                node_data_id: node_id(),
                key: KeyDto::from("per12345".to_string()),
                properties: PropertiesDataDto {
                    values: HashMap::from([(
                        "age12345".to_string(),
                        // String instead of Number
                        serde_json::Value::String("thirty".to_string()),
                    )]),
                },
            }],
            edges: vec![],
        };
        let errors = data.validate_against_schema(&schema).unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, SchemaComplianceError::WrongPropertyType { .. })));
        let msg = errors[0].to_string();
        assert!(msg.contains("Number"));
        assert!(msg.contains("String"));
    }

    #[test]
    fn invalid_select_option_is_reported() {
        let schema = schema_with_node(
            "per12345",
            vec![make_select_property_schema(
                "gen12345",
                "Gender",
                vec!["Male", "Female"],
            )],
        );
        let data = CreateGraphDataDto {
            nodes: vec![CreateNodeDataDto {
                node_data_id: node_id(),
                key: KeyDto::from("per12345".to_string()),
                properties: PropertiesDataDto {
                    values: HashMap::from([(
                        "gen12345".to_string(),
                        serde_json::Value::String("M".to_string()),
                    )]),
                },
            }],
            edges: vec![],
        };
        let errors = data.validate_against_schema(&schema).unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, SchemaComplianceError::InvalidSelectOption { .. })));
        let msg = errors[0].to_string();
        assert!(msg.contains("Male"));
        assert!(msg.contains("Female"));
    }

    #[test]
    fn multiple_errors_are_all_collected() {
        let schema = GraphSchemaDto {
            nodes: vec![make_node_schema(
                "per12345",
                vec![
                    make_property_schema("nme12345", "Name", PropertyTypeDto::String),
                    make_property_schema("age12345", "Age", PropertyTypeDto::Number),
                ],
            )],
            edges: vec![],
        };
        // wrong type for age, missing name, extra unknown property
        let data = CreateGraphDataDto {
            nodes: vec![CreateNodeDataDto {
                node_data_id: node_id(),
                key: KeyDto::from("per12345".to_string()),
                properties: PropertiesDataDto {
                    values: HashMap::from([
                        (
                            "age12345".to_string(),
                            serde_json::Value::String("not-a-number".to_string()),
                        ),
                        (
                            "ghost000".to_string(),
                            serde_json::Value::String("extra".to_string()),
                        ),
                    ]),
                },
            }],
            edges: vec![],
        };
        let errors = data.validate_against_schema(&schema).unwrap_err();
        // Expects: UnknownProperty(ghost000), MissingProperty(nme12345), WrongPropertyType(age12345)
        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn unknown_edge_type_is_reported() {
        let schema = GraphSchemaDto {
            nodes: vec![],
            edges: vec![make_edge_schema("kno12345", vec![])],
        };
        let id1 = node_id();
        let id2 = node_id();
        let data = CreateGraphDataDto {
            nodes: vec![],
            edges: vec![CreateEdgeDataDto {
                key: KeyDto::from("hat00000".to_string()),
                from_node_data_id: id1,
                to_node_data_id: id2,
                properties: PropertiesDataDto {
                    values: HashMap::new(),
                },
            }],
        };
        let errors = data.validate_against_schema(&schema).unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, SchemaComplianceError::UnknownEdgeType { .. })));
        assert!(errors[0].to_string().contains("kno12345"));
    }
}
