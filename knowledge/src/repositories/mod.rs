use crate::error::ApiError;
use bric_a_brac_protos::knowledge::{property_value, EdgeData, GraphData, NodeData, PropertyValue};
use neo4rs::{query, BoltString, BoltType};
use std::collections::HashMap;

pub struct Repository;

impl Repository {
    pub fn new() -> Self {
        Self
    }

    pub async fn insert_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: String,
        node_data_id: String,
        key: String,
        properties: HashMap<String, PropertyValue>,
    ) -> Result<NodeData, ApiError> {
        let mut properties: HashMap<BoltString, BoltType> =
            PropertiesWrapper(properties).try_into()?;
        properties.insert("graph_id".to_string().into(), graph_id.to_string().into());
        properties.insert(
            "node_data_id".to_string().into(),
            node_data_id.to_string().into(),
        );
        let prop_keys: Vec<String> = properties
            .keys()
            .enumerate()
            .map(|(i, key)| format!("{}: $p{}", key, i))
            .collect();

        let cypher = format!(
            r#"
CREATE (n:{} {{ {} }})
RETURN n
        "#,
            key,
            prop_keys.join(", ")
        );
        let q = properties
            .iter()
            .enumerate()
            .fold(query(&cypher), |q, (i, (_key, value))| {
                q.param(&format!("p{}", i), value.clone())
            });
        let mut result = connection.execute(q).await?;
        let row = result
            .next(connection)
            .await?
            .ok_or_else(|| ApiError::NoneRow())?;
        let neo_node: neo4rs::Node = row.get("n")?;

        Ok(NodeDataWrapper::try_from(neo_node)?.0)
    }

    pub async fn insert_edge(
        &self,
        connection: &mut neo4rs::Txn,
        edge_data_id: String,
        from_node_data_id: String,
        to_node_data_id: String,
        key: String,
        properties: HashMap<String, PropertyValue>,
    ) -> Result<EdgeData, ApiError> {
        let mut properties: HashMap<BoltString, BoltType> =
            PropertiesWrapper(properties).try_into()?;
        properties.insert(
            "edge_data_id".to_string().into(),
            edge_data_id.to_string().into(),
        );
        let prop_keys: Vec<String> = properties
            .keys()
            .enumerate()
            .map(|(i, key)| format!("{}: $p{}", key, i))
            .collect();
        let edge_props = format!(" {{ {} }}", prop_keys.join(", "));
        let cypher = format!(
            r#"
MATCH
    (a {{ node_data_id: $from_node_data_id }}),
    (b {{ node_data_id: $to_node_data_id }})
CREATE (a)-[e:{}{}]->(b)
RETURN
    e,
    a.node_data_id AS from_node_data_id,
    b.node_data_id AS to_node_data_id
        "#,
            key, edge_props
        );

        let q = properties.iter().enumerate().fold(
            query(&cypher)
                .param("from_node_data_id", from_node_data_id.clone())
                .param("to_node_data_id", to_node_data_id.clone()),
            |q, (i, (_key, value))| q.param(&format!("p{}", i), value.clone()),
        );
        let mut result = connection.execute(q).await?;
        let row = result
            .next(connection)
            .await?
            .ok_or_else(|| ApiError::NoneRow())?;
        let neo_edge: neo4rs::Relation = row.get("e")?;
        let from_node_data_id = row.get("from_node_data_id")?;
        let to_node_data_id = row.get("to_node_data_id")?;
        let mut edge_data = EdgeDataWrapper::try_from(neo_edge)?.0;
        edge_data.from_node_data_id = from_node_data_id;
        edge_data.to_node_data_id = to_node_data_id;

        Ok(edge_data)
    }

    pub async fn load_graph(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: String,
    ) -> Result<GraphData, ApiError> {
        let nodes = self.load_graph_nodes(connection, graph_id.clone()).await?;
        let edges = self.load_graph_edges(connection, graph_id).await?;

        Ok(GraphData { nodes, edges })
    }

    async fn load_graph_nodes(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: String,
    ) -> Result<Vec<NodeData>, ApiError> {
        let nodes_query =
            query("MATCH (n { graph_id: $graph_id }) RETURN n").param("graph_id", graph_id.clone());
        let mut nodes_result = connection.execute(nodes_query).await?;

        let mut nodes = Vec::new();
        while let Some(row) = nodes_result.next(&mut *connection).await? {
            let neo_node: neo4rs::Node = row.get("n")?;
            nodes.push(NodeDataWrapper::try_from(neo_node)?.0);
        }

        Ok(nodes)
    }

    async fn load_graph_edges(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: String,
    ) -> Result<Vec<EdgeData>, ApiError> {
        let edges_query = query(
            r#"
MATCH (a { graph_id: $graph_id })-[e]->(b { graph_id: $graph_id })
RETURN
    e,
    a.node_data_id AS from_node_data_id,
    b.node_data_id AS to_node_data_id
        "#,
        )
        .param("graph_id", graph_id);
        let mut edges_result = connection.execute(edges_query).await?;

        let mut edges = Vec::new();
        while let Some(row) = edges_result.next(&mut *connection).await? {
            let neo_edge: neo4rs::Relation = row.get("e")?;
            let from_node_data_id = row.get("from_node_data_id")?;
            let to_node_data_id = row.get("to_node_data_id")?;
            let mut edge_data = EdgeDataWrapper::try_from(neo_edge)?.0;
            edge_data.from_node_data_id = from_node_data_id;
            edge_data.to_node_data_id = to_node_data_id;

            edges.push(edge_data);
        }

        Ok(edges)
    }
}

trait ExtendElement {
    fn get(&self, key: &str) -> Option<BoltType>;

    fn keys(&self) -> Vec<&str>;

    fn extract_id_in_properties(&self, id_key: &str) -> Result<String, ApiError> {
        let bolt_id = self
            .get(id_key)
            .ok_or_else(|| ApiError::MissingId(id_key.to_string()))?;
        let value = PropertyValueWrapper::try_from(bolt_id)?.0.value;

        match value {
            Some(property_value::Value::StringValue(s)) => Ok(s),
            _ => Err(ApiError::WrongId(id_key.to_string())),
        }
    }

    fn collect_properties(&self) -> Result<HashMap<String, PropertyValue>, ApiError> {
        self.keys()
            .into_iter()
            .filter(|key| *key != "node_data_id" && *key != "graph_id" && *key != "edge_data_id")
            .map(|key| {
                let value = self
                    .get(key)
                    .ok_or_else(|| ApiError::UnreachableProperty(key.to_string()))?;
                let prop_value = PropertyValueWrapper::try_from(value)?.0;
                Ok((key.to_string(), prop_value))
            })
            .collect()
    }
}

impl ExtendElement for neo4rs::Node {
    fn get(&self, key: &str) -> Option<BoltType> {
        self.get::<BoltType>(key).ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.keys()
    }
}

impl ExtendElement for neo4rs::Relation {
    fn get(&self, key: &str) -> Option<BoltType> {
        self.get::<BoltType>(key).ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.keys()
    }
}

struct NodeDataWrapper(NodeData);

impl TryFrom<neo4rs::Node> for NodeDataWrapper {
    type Error = ApiError;

    fn try_from(node: neo4rs::Node) -> Result<Self, Self::Error> {
        let node_data_id = node.extract_id_in_properties("node_data_id")?;
        let graph_id = node.extract_id_in_properties("graph_id")?;
        let key = node
            .labels()
            .first()
            .ok_or_else(|| ApiError::UnlabeledNode(node_data_id.clone()))?
            .to_string();
        let properties = node.collect_properties()?;

        Ok(NodeDataWrapper(NodeData {
            graph_id,
            node_data_id,
            key,
            properties,
        }))
    }
}

struct EdgeDataWrapper(EdgeData);

impl TryFrom<neo4rs::Relation> for EdgeDataWrapper {
    type Error = ApiError;

    fn try_from(edge: neo4rs::Relation) -> Result<Self, Self::Error> {
        let edge_data_id = edge.extract_id_in_properties("edge_data_id")?;
        let key = edge.typ().to_string();
        let properties = edge.collect_properties()?;

        Ok(EdgeDataWrapper(EdgeData {
            edge_data_id,
            key,
            from_node_data_id: "".to_string(),
            to_node_data_id: "".to_string(),
            properties,
        }))
    }
}

struct PropertiesWrapper(HashMap<String, PropertyValue>);

impl TryFrom<HashMap<BoltString, BoltType>> for PropertiesWrapper {
    type Error = ApiError;

    fn try_from(props: HashMap<BoltString, BoltType>) -> Result<Self, Self::Error> {
        Ok(PropertiesWrapper(
            props
                .into_iter()
                .map(|(k, v)| Ok((k.to_string(), PropertyValueWrapper::try_from(v)?.0)))
                .collect::<Result<_, Self::Error>>()?,
        ))
    }
}

impl TryFrom<PropertiesWrapper> for HashMap<BoltString, BoltType> {
    type Error = ApiError;

    fn try_from(wrapper: PropertiesWrapper) -> Result<Self, Self::Error> {
        wrapper
            .0
            .into_iter()
            .map(|(k, v)| Ok((k.into(), BoltType::try_from(PropertyValueWrapper(v))?)))
            .collect()
    }
}

struct PropertyValueWrapper(PropertyValue);

impl TryFrom<BoltType> for PropertyValueWrapper {
    type Error = ApiError;

    fn try_from(bolt: BoltType) -> Result<Self, Self::Error> {
        let value = match bolt {
            BoltType::String(s) => property_value::Value::StringValue(s.to_string()),
            BoltType::Float(f) => property_value::Value::NumberValue(f.value),
            BoltType::Boolean(b) => property_value::Value::BoolValue(b.value),
            _ => return Err(ApiError::UnsupportedBoltType(bolt)),
        };
        Ok(PropertyValueWrapper(PropertyValue { value: Some(value) }))
    }
}

impl TryFrom<PropertyValueWrapper> for BoltType {
    type Error = ApiError;

    fn try_from(wrapper: PropertyValueWrapper) -> Result<Self, Self::Error> {
        let value = wrapper.0.value.as_ref().map(|v| match v {
            property_value::Value::StringValue(s) => BoltType::String(s.clone().into()),
            property_value::Value::NumberValue(n) => BoltType::Float(neo4rs::BoltFloat::new(*n)),
            property_value::Value::BoolValue(b) => BoltType::Boolean(neo4rs::BoltBoolean::new(*b)),
        });
        value.ok_or(ApiError::UnsupportedPropertyValue(wrapper.0))
    }
}
