use crate::{infrastructure::clients::OpenRouterClient, presentation::errors::AppError};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateGraphDataDto, CreateNodeDataDto, GraphSchemaDto, NodeDataIdDto,
    PropertiesDataDto, PropertyTypeDto,
};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;

pub struct DataService {
    openrouter_client: OpenRouterClient,
}

impl DataService {
    pub fn new(openrouter_client: OpenRouterClient) -> Self {
        Self { openrouter_client }
    }

    #[tracing::instrument(
        level = "trace",
        name = "data_service.generate_data",
        skip(self, schema, file_content)
    )]
    pub async fn generate_data(
        &self,
        schema: GraphSchemaDto,
        file_content: Vec<u8>,
    ) -> Result<CreateGraphDataDto, AppError> {
        let parsed_content =
            std::str::from_utf8(&file_content).map_err(|err| AppError::FileParsing {
                message: "Failed to parse file".to_string(),
                source: err,
            })?;

        let (legend, csv) = schema_to_template_csv(schema.clone()).map_err(|_| AppError::Internal {
            context: "Failed to build template CSV".to_string(),
        })?;
        let system_prompt = build_system_prompt(&legend, &csv);
        let user_prompt = build_user_prompt(&parsed_content);

        let generated_data = self
            .openrouter_client
            .chat(&system_prompt, &user_prompt, None, None)
            .await?;

        let csv_data = generated_data.as_str().ok_or_else(|| AppError::Internal {
            context: "Expected generated data to be a string".to_string(),
        })?;
        // tracing::debug!("Generated graph data:\n{}", csv_data);

        let property_types: HashMap<String, PropertyTypeDto> = schema
            .nodes
            .iter()
            .flat_map(|n| n.properties.iter())
            .chain(schema.edges.iter().flat_map(|e| e.properties.iter()))
            .map(|p| (p.key.to_string(), p.property_type.clone()))
            .collect();

        let graph_data = csv_to_graph_data(csv_data, &property_types).map_err(|err| AppError::Internal {
            context: format!("Failed to convert CSV to graph data: {}", err),
        })?;

        // !! TODO: validation CreateGraphDataDto
        // !! TODO: verify data is conform to schema

        Ok(graph_data)
    }
}

fn build_system_prompt(legend: &str, csv: &str) -> String {
    format!(
        r##"You are a graph data extractor assistant. Your task is to extract data from a document and populate a graph.

A graph schema defines the TYPES of entities and relationships:
- Nodes represent ENTITY TYPES (e.g., 'Person', 'Location', 'Company') — not specific people or places
- Edges represent RELATIONSHIP TYPES between node types (e.g., 'Friend Of', 'Born In') — not specific relationships
- Attributes define the PROPERTIES that instances of a node/edge type can have (e.g., 'Name', 'Eye Color', 'Birth Year') — not the actual values

You will be given a graph schema and a document. Extract all relevant entities and relationships from the document and return them as structured data that conforms to the schema.

Rules:
- Only create node and relationship types that exist in the schema
- Only use properties that are defined in the schema for each node/relationship type
- Assign a unique string ID to every node (e.g., 'n1', 'n2', ...) — IDs must be consistent across all tables
- Every relationship must reference valid node IDs via 'from' and 'to'
- Every relationship must reference different node IDs via 'from' and 'to' (no self-loops)
- Do not invent data that is not present or strongly implied by the document

Formatting and Output Instructions:
- Strictly follow the template below for your response.
- Output only the CSV tables, no extra text, explanations, or comments.
- Use only the property values and options defined in the schema.
- For select-type properties, use only the allowed options.
- Do not output anything except the CSV tables as shown in the template.

Here is the schema of the graph, which defines the node and relationship types, their properties, and the allowed values for those properties.
Use this as a reference to understand how to structure your output:

{}

Here is the template for how to format your response based on the provided schema:

{}

Think of it like inserting rows into a database, not defining the schema."##,
        legend, csv
    )
}

fn build_user_prompt(document: &str) -> String {
    format!(
        r##"Extract all entities and relationships from the following document and populate the graph according to the schema:

{}"##,
        document
    )
}

fn schema_to_template_csv(schema: GraphSchemaDto) -> Result<(String, String), ()> {
    let mut legend = String::new();
    let mut csv = String::new();

    let mut node_map = BTreeMap::new();
    for node in &schema.nodes {
        node_map.insert(node.key.clone(), node);
    }
    let mut edge_map = BTreeMap::new();
    for edge in &schema.edges {
        edge_map.insert(edge.key.clone(), edge);
    }

    for (key, node) in &node_map {
        let mut required_keys: Vec<String> =
            node.properties.iter().map(|p| p.key.to_string()).collect();
        required_keys.sort();
        writeln!(
            legend,
            "## Node-{}\nlabel: \"{}\"\nrequired_properties: [{}]",
            key,
            node.label,
            required_keys.join(", ")
        )
        .map_err(|_| ())?;
    }
    for (key, edge) in &edge_map {
        let mut required_keys: Vec<String> =
            edge.properties.iter().map(|p| p.key.to_string()).collect();
        required_keys.sort();
        writeln!(
            legend,
            "## Edge-{}\nlabel: \"{}\"\nrequired_properties: [{}]",
            key,
            edge.label,
            required_keys.join(", ")
        )
        .map_err(|_| ())?;
    }

    let mut property_map = BTreeMap::new();
    for node in &schema.nodes {
        for prop in &node.properties {
            property_map.insert(
                prop.key.clone(),
                (
                    prop.label.clone(),
                    prop.property_type.clone(),
                    prop.metadata.options.clone(),
                ),
            );
        }
    }
    for edge in &schema.edges {
        for prop in &edge.properties {
            property_map.insert(
                prop.key.clone(),
                (
                    prop.label.clone(),
                    prop.property_type.clone(),
                    prop.metadata.options.clone(),
                ),
            );
        }
    }
    for (key, (label, property_type, options)) in &property_map {
        writeln!(legend, "## Property-{}\nlabel: \"{}\"", key, label).map_err(|_| ())?;
        writeln!(legend, "type: {}", property_type).map_err(|_| ())?;
        if let Some(opts) = options {
            let mut opts_vec = opts.clone();
            opts_vec.sort();
            let opts_str = opts_vec
                .iter()
                .map(|o| format!("\"{}\"", o))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(legend, "options: [{}]", opts_str).map_err(|_| ())?;
        }
    }

    for (key, node) in &node_map {
        let mut headers = vec!["id".to_string()];
        let mut prop_keys: Vec<String> = node.properties.iter().map(|p| p.key.to_string()).collect();
        prop_keys.sort();
        headers.extend(prop_keys);
        writeln!(csv, "## Node-{}\n{}", key, headers.join(",")).map_err(|_| ())?
    }
    for (key, edge) in &edge_map {
        let mut headers = vec!["from".to_string(), "to".to_string()];
        let mut prop_keys: Vec<String> = edge.properties.iter().map(|p| p.key.to_string()).collect();
        prop_keys.sort();
        headers.extend(prop_keys);
        writeln!(csv, "## Edge-{}\n{}", key, headers.join(",")).map_err(|_| ())?
    }

    Ok((legend.trim().to_string(), csv.trim().to_string()))
}

fn coerce_value(raw: &str, property_type: Option<&PropertyTypeDto>) -> serde_json::Value {
    match property_type {
        Some(PropertyTypeDto::Number) => raw
            .parse::<serde_json::Number>()
            .map(serde_json::Value::Number)
            .unwrap_or_else(|_| serde_json::Value::String(raw.to_string())),
        Some(PropertyTypeDto::Boolean) => match raw.to_lowercase().as_str() {
            "true" => serde_json::Value::Bool(true),
            "false" => serde_json::Value::Bool(false),
            _ => serde_json::Value::String(raw.to_string()),
        },
        _ => serde_json::Value::String(raw.to_string()),
    }
}

fn csv_to_graph_data(
    csv: &str,
    property_types: &HashMap<String, PropertyTypeDto>,
) -> Result<CreateGraphDataDto, String> {
    let csv = csv.trim();
    let mut lines = csv.lines().peekable();
    let mut nodes: Vec<CreateNodeDataDto> = vec![];
    let mut edges: Vec<CreateEdgeDataDto> = vec![];
    let mut node_id_map: HashMap<String, NodeDataIdDto> = HashMap::new();

    while let Some(line) = lines.next() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if let Some(key) = line.strip_prefix("## Node-") {
            let key = key.trim().to_string();
            let headers_line = lines.next().ok_or_else(|| format!("Expected headers after '## Node-{}'" , key))?;
            let headers: Vec<&str> = headers_line.trim().split(',').map(str::trim).collect();

            loop {
                match lines.peek() {
                    None => break,
                    Some(next) if next.trim().starts_with("## ") => break,
                    _ => {}
                }
                let row = lines.next().ok_or_else(|| "Unexpected end of input while reading node row".to_string())?;
                let row = row.trim();
                if row.is_empty() {
                    continue;
                }
                let cols: Vec<&str> = row.split(',').map(str::trim).collect();
                if cols.len() != headers.len() {
                    return Err(format!("Node row has {} columns but headers has {}", cols.len(), headers.len()));
                }
                let ai_id = cols[0].to_string();
                let node_id = NodeDataIdDto::new();
                node_id_map.insert(ai_id, node_id);

                let mut values = HashMap::new();
                for i in 1..headers.len() {
                    let value = coerce_value(cols[i], property_types.get(headers[i]));
                    values.insert(headers[i].to_string(), value);
                }
                nodes.push(CreateNodeDataDto {
                    node_data_id: node_id,
                    key: key.clone().into(),
                    properties: PropertiesDataDto { values },
                });
            }
        } else if let Some(key) = line.strip_prefix("## Edge-") {
            let key = key.trim().to_string();
            let headers_line = lines.next().ok_or_else(|| format!("Expected headers after '## Edge-{}'" , key))?;
            let headers: Vec<&str> = headers_line.trim().split(',').map(str::trim).collect();

            loop {
                match lines.peek() {
                    None => break,
                    Some(next) if next.trim().starts_with("## ") => break,
                    _ => {}
                }
                let row = lines.next().ok_or_else(|| "Unexpected end of input while reading edge row".to_string())?;
                let row = row.trim();
                if row.is_empty() {
                    continue;
                }
                let cols: Vec<&str> = row.split(',').map(str::trim).collect();
                if cols.len() != headers.len() {
                    return Err(format!("Edge row has {} columns but headers has {}", cols.len(), headers.len()));
                }
                let from_id = node_id_map.get(cols[0]).copied().ok_or_else(|| format!("Unknown node ID '{}' in edge 'from'", cols[0]))?;
                let to_id = node_id_map.get(cols[1]).copied().ok_or_else(|| format!("Unknown node ID '{}' in edge 'to'", cols[1]))?;

                let mut values = HashMap::new();
                for i in 2..headers.len() {
                    let value = coerce_value(cols[i], property_types.get(headers[i]));
                    values.insert(headers[i].to_string(), value);
                }
                edges.push(CreateEdgeDataDto {
                    key: key.clone().into(),
                    from_node_data_id: from_id,
                    to_node_data_id: to_id,
                    properties: PropertiesDataDto { values },
                });
            }
        } else {
            return Err(format!("Unexpected line: '{}'", line));
        }
    }

    Ok(CreateGraphDataDto { nodes, edges })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_to_template_csv() {
        let schema = serde_json::from_value::<GraphSchemaDto>(json!({
            "nodes": [
                {
                    "node_schema_id": "019c9503-34c7-71a0-aee8-f5025ebb9e27",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Object",
                    "key": "ESVhRs9k",
                    "color": "#C70039",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5e0b2ada8a3",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f5025ebb9e27",
                            "edge_schema_id": null,
                            "label": "Name",
                            "key": "po86zGND",
                            "property_type": "String",
                            "metadata": {
                                "options": null
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5f426c41dd0",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f5025ebb9e27",
                            "edge_schema_id": null,
                            "label": "Type",
                            "key": "JboctBKk",
                            "property_type": "Select",
                            "metadata": {
                                "options": [
                                    "Lighthouse",
                                    "Logbook"
                                ]
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                },
                {
                    "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Person",
                    "key": "dudFcexv",
                    "color": "#28B463",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5a8ed5de18a",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                            "edge_schema_id": null,
                            "label": "Role",
                            "key": "dDlyhiOg",
                            "property_type": "Select",
                            "metadata": {
                                "options": [
                                    "Keeper",
                                    "Witness"
                                ]
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f579dd147c29",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                            "edge_schema_id": null,
                            "label": "Name",
                            "key": "B1ixXXAx",
                            "property_type": "String",
                            "metadata": {
                                "options": null
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f58b646e309d",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                            "edge_schema_id": null,
                            "label": "Location",
                            "key": "m0NrB2sm",
                            "property_type": "String",
                            "metadata": {
                                "options": null
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f591c4ec345c",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                            "edge_schema_id": null,
                            "label": "Years of Experience",
                            "key": "h2GIMoa9",
                            "property_type": "Number",
                            "metadata": {
                                "options": null
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                },
                {
                    "node_schema_id": "019c9503-34c7-71a0-aee8-f4ff2d6290c2",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Element",
                    "key": "nFbOTJ9C",
                    "color": "#FFC300",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5bf12c9eb42",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ff2d6290c2",
                            "edge_schema_id": null,
                            "label": "Type",
                            "key": "K1FOhEqB",
                            "property_type": "Select",
                            "metadata": {
                                "options": [
                                    "Fog",
                                    "Sea",
                                    "Light",
                                    "Gannet"
                                ]
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5cf8d5f85ca",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ff2d6290c2",
                            "edge_schema_id": null,
                            "label": "Color",
                            "key": "XcrvXgOd",
                            "property_type": "String",
                            "metadata": {
                                "options": null
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5dcb48d3a33",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ff2d6290c2",
                            "edge_schema_id": null,
                            "label": "Behavior",
                            "key": "jZ6GspWq",
                            "property_type": "Select",
                            "metadata": {
                                "options": [
                                    "Indifferent",
                                    "Responsive"
                                ]
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                }
            ],
            "edges": [
                {
                    "edge_schema_id": "019c9503-34c7-71a0-aee8-f52911dc6692",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Communicates With",
                    "key": "eELB9Bwe",
                    "color": "#3355FF",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f625c649630c",
                            "node_schema_id": null,
                            "edge_schema_id": "019c9503-34c7-71a0-aee8-f52911dc6692",
                            "label": "Connection Type",
                            "key": "MDHY1uVN",
                            "property_type": "Select",
                            "metadata": {
                                "options": [
                                    "Parent-Child",
                                    "Profession"
                                ]
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                },
                {
                    "edge_schema_id": "019c9503-34c7-71a0-aee8-f51dcfde6050",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Records",
                    "key": "Oq9afK3f",
                    "color": "#FF5733",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f60ac4ae8e56",
                            "node_schema_id": null,
                            "edge_schema_id": "019c9503-34c7-71a0-aee8-f51dcfde6050",
                            "label": "Type",
                            "key": "rZYz1jYr",
                            "property_type": "Select",
                            "metadata": {
                                "options": [
                                    "Routine",
                                    "Observation"
                                ]
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f6178554dfdd",
                            "node_schema_id": null,
                            "edge_schema_id": "019c9503-34c7-71a0-aee8-f51dcfde6050",
                            "label": "Frequency",
                            "key": "QVL3enQS",
                            "property_type": "Select",
                            "metadata": {
                                "options": [
                                    "Daily",
                                    "Weekly",
                                    "Monthly"
                                ]
                            },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                }
            ]
        }))
        .unwrap();

        let (legend, csv) = schema_to_template_csv(schema).unwrap();
        let expected_legend = r#"
## Node-ESVhRs9k
label: "Object"
required_properties: [JboctBKk, po86zGND]
## Node-dudFcexv
label: "Person"
required_properties: [B1ixXXAx, dDlyhiOg, h2GIMoa9, m0NrB2sm]
## Node-nFbOTJ9C
label: "Element"
required_properties: [K1FOhEqB, XcrvXgOd, jZ6GspWq]
## Edge-Oq9afK3f
label: "Records"
required_properties: [QVL3enQS, rZYz1jYr]
## Edge-eELB9Bwe
label: "Communicates With"
required_properties: [MDHY1uVN]
## Property-B1ixXXAx
label: "Name"
type: String
## Property-JboctBKk
label: "Type"
type: Select
options: ["Lighthouse", "Logbook"]
## Property-K1FOhEqB
label: "Type"
type: Select
options: ["Fog", "Gannet", "Light", "Sea"]
## Property-MDHY1uVN
label: "Connection Type"
type: Select
options: ["Parent-Child", "Profession"]
## Property-QVL3enQS
label: "Frequency"
type: Select
options: ["Daily", "Monthly", "Weekly"]
## Property-XcrvXgOd
label: "Color"
type: String
## Property-dDlyhiOg
label: "Role"
type: Select
options: ["Keeper", "Witness"]
## Property-h2GIMoa9
label: "Years of Experience"
type: Number
## Property-jZ6GspWq
label: "Behavior"
type: Select
options: ["Indifferent", "Responsive"]
## Property-m0NrB2sm
label: "Location"
type: String
## Property-po86zGND
label: "Name"
type: String
## Property-rZYz1jYr
label: "Type"
type: Select
options: ["Observation", "Routine"]
"#
        .trim()
        .to_string();

        let expected_csv = r#"
## Node-ESVhRs9k
id,JboctBKk,po86zGND
## Node-dudFcexv
id,B1ixXXAx,dDlyhiOg,h2GIMoa9,m0NrB2sm
## Node-nFbOTJ9C
id,K1FOhEqB,XcrvXgOd,jZ6GspWq
## Edge-Oq9afK3f
from,to,QVL3enQS,rZYz1jYr
## Edge-eELB9Bwe
from,to,MDHY1uVN
"#
        .trim()
        .to_string();

        assert_eq!(legend, expected_legend);
        assert_eq!(csv, expected_csv);
    }

    #[test]
    fn test_csv_to_graph_data() {
        let csv = r#"
## Node-JparY0E3
 id,ft5ybMXL,k7xdQhvx
 n1,Logbook,Lighthouse
 ## Node-di8zqvue
 id,INa4ejnu,VCQ8T4SG,X4u4YW94,fmwi38Kd
 n1,Point,Keeper,30,Marcus
 n2,Edinburgh,Witness,0,Daughter
 ## Node-wZlDcFRT
 id,DeJx3xiU,WAJyLvmA,WPv9aBad
 n1,Responsive,white,Fog
 n2,Responsive,dark green,Sea
 n3,Indifferent,white,Gannet
 ## Edge-EPLXEEpm
 from,to,I3HCHgGr,gWp6NrlG
 n1,n1,Daily,Observation
 n2,n2,Weekly,Routine
 ## Edge-z41HW1jw
 from,to,pgD0HOWK
 n1,n2,Profession
"#
        .trim();

        let graph_data = csv_to_graph_data(
            csv,
            &HashMap::from([("X4u4YW94".to_string(), PropertyTypeDto::Number)]),
        )
        .unwrap();

        let id_to_idx: HashMap<NodeDataIdDto, usize> = graph_data
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.node_data_id, i))
            .collect();

        // All node_data_ids must be unique
        assert_eq!(
            id_to_idx.len(),
            graph_data.nodes.len(),
            "node_data_ids must be unique"
        );

        // All edge references must point to existing nodes
        for edge in &graph_data.edges {
            assert!(id_to_idx.contains_key(&edge.from_node_data_id));
            assert!(id_to_idx.contains_key(&edge.to_node_data_id));
        }

        // Verify nodes: key + properties (order-preserved, ignoring node_data_id)
        let actual_nodes: Vec<(String, serde_json::Value)> = graph_data
            .nodes
            .iter()
            .map(|n| (n.key.to_string(), serde_json::to_value(&n.properties).unwrap()))
            .collect();
        assert_eq!(
            actual_nodes,
            vec![
                (
                    "JparY0E3".to_string(),
                    json!({"ft5ybMXL": "Logbook", "k7xdQhvx": "Lighthouse"})
                ),
                (
                    "di8zqvue".to_string(),
                    json!({"INa4ejnu": "Point", "VCQ8T4SG": "Keeper", "X4u4YW94": 30, "fmwi38Kd": "Marcus"})
                ),
                (
                    "di8zqvue".to_string(),
                    json!({"INa4ejnu": "Edinburgh", "VCQ8T4SG": "Witness", "X4u4YW94": 0, "fmwi38Kd": "Daughter"})
                ),
                (
                    "wZlDcFRT".to_string(),
                    json!({"DeJx3xiU": "Responsive", "WAJyLvmA": "white", "WPv9aBad": "Fog"})
                ),
                (
                    "wZlDcFRT".to_string(),
                    json!({"DeJx3xiU": "Responsive", "WAJyLvmA": "dark green", "WPv9aBad": "Sea"})
                ),
                (
                    "wZlDcFRT".to_string(),
                    json!({"DeJx3xiU": "Indifferent", "WAJyLvmA": "white", "WPv9aBad": "Gannet"})
                ),
            ]
        );

        // Verify edges: key + from/to node indices + properties
        // (n1, n2, n3 in the edge sections resolve to wZlDcFRT nodes at indices 3, 4, 5,
        //  because JparY0E3's n1 and di8zqvue's n1/n2 are overwritten in the id map)
        let actual_edges: Vec<(String, usize, usize, serde_json::Value)> = graph_data
            .edges
            .iter()
            .map(|e| {
                (
                    e.key.to_string(),
                    id_to_idx[&e.from_node_data_id],
                    id_to_idx[&e.to_node_data_id],
                    serde_json::to_value(&e.properties).unwrap(),
                )
            })
            .collect();
        assert_eq!(
            actual_edges,
            vec![
                (
                    "EPLXEEpm".to_string(),
                    3,
                    3,
                    json!({"I3HCHgGr": "Daily", "gWp6NrlG": "Observation"})
                ),
                (
                    "EPLXEEpm".to_string(),
                    4,
                    4,
                    json!({"I3HCHgGr": "Weekly", "gWp6NrlG": "Routine"})
                ),
                (
                    "z41HW1jw".to_string(),
                    3,
                    4,
                    json!({"pgD0HOWK": "Profession"})
                ),
            ]
        );
    }
}
