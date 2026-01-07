use bric_a_brac_protos::knowledge::property_value::Value as ProtoValue;
use bric_a_brac_protos::knowledge::PropertyValue as ProtoPropertyValue;
use std::collections::HashMap;

use crate::dto::{EdgeResponse, GraphDataResponse, NodeResponse, PropertyValueDto};

// Wrapper type to satisfy orphan rule
pub struct PropertyValueWrapper<'a>(pub &'a ProtoPropertyValue);

// Convert DTO PropertyValue to Proto PropertyValue
impl From<PropertyValueDto> for ProtoPropertyValue {
    fn from(dto: PropertyValueDto) -> Self {
        let value = match dto {
            PropertyValueDto::String(s) => ProtoValue::StringValue(s),
            PropertyValueDto::Int(i) => ProtoValue::IntValue(i),
            PropertyValueDto::Float(f) => ProtoValue::FloatValue(f),
            PropertyValueDto::Bool(b) => ProtoValue::BoolValue(b),
        };
        ProtoPropertyValue { value: Some(value) }
    }
}

// Convert Proto PropertyValue to DTO PropertyValue using wrapper
impl<'a> From<PropertyValueWrapper<'a>> for Option<PropertyValueDto> {
    fn from(wrapper: PropertyValueWrapper<'a>) -> Self {
        wrapper.0.value.as_ref().map(|v| match v {
            ProtoValue::StringValue(s) => PropertyValueDto::String(s.clone()),
            ProtoValue::IntValue(i) => PropertyValueDto::Int(*i),
            ProtoValue::FloatValue(f) => PropertyValueDto::Float(*f),
            ProtoValue::BoolValue(b) => PropertyValueDto::Bool(*b),
        })
    }
}

// Convert Proto Node to DTO NodeResponse
impl From<bric_a_brac_protos::knowledge::Node> for NodeResponse {
    fn from(node: bric_a_brac_protos::knowledge::Node) -> Self {
        // Parse properties from JSON string
        let properties = serde_json::from_str(&node.properties_json).unwrap_or_default();

        NodeResponse {
            id: node.id,
            label: node.label,
            properties,
        }
    }
}

// Convert Proto Edge to DTO EdgeResponse
impl From<bric_a_brac_protos::knowledge::Edge> for EdgeResponse {
    fn from(edge: bric_a_brac_protos::knowledge::Edge) -> Self {
        // Parse properties from JSON string
        let properties = serde_json::from_str(&edge.properties_json).unwrap_or_default();

        EdgeResponse {
            id: edge.id,
            from_id: edge.from_id,
            to_id: edge.to_id,
            label: edge.label,
            properties,
        }
    }
}

// Convert Proto GraphDataResponse to DTO GraphDataResponse
impl From<bric_a_brac_protos::knowledge::GraphDataResponse> for GraphDataResponse {
    fn from(proto: bric_a_brac_protos::knowledge::GraphDataResponse) -> Self {
        GraphDataResponse {
            nodes: proto.nodes.into_iter().map(NodeResponse::from).collect(),
            edges: proto.edges.into_iter().map(EdgeResponse::from).collect(),
        }
    }
}

// Helper to convert HashMap<String, PropertyValueDto> to HashMap<String, ProtoPropertyValue>
pub fn dto_properties_to_proto(
    props: HashMap<String, PropertyValueDto>,
) -> HashMap<String, ProtoPropertyValue> {
    props
        .into_iter()
        .map(|(k, v)| (k, ProtoPropertyValue::from(v)))
        .collect()
}
