use bric_a_brac_protos::common::GraphSchemaProto;

pub fn build_system_prompt(schema: &GraphSchemaProto) -> String {
    let mut prompt = String::new();

    // Identity
    prompt.push_str(
        "You are a knowledge graph assistant. Your role is to help users build, query, and \
         explore a knowledge graph. You extract entities and relationships from user-provided \
         information and store them as nodes and edges in the graph. You also answer questions \
         by searching and traversing the graph.\n\n",
    );

    // Schemas
    prompt.push_str("## Current Graph Schema\n\n");

    if schema.nodes.is_empty() && schema.edges.is_empty() {
        prompt.push_str(
            "The graph has no schemas defined yet. When the user provides information, create \
             appropriate node and edge schemas first using `create_schema` and `create_edge_schema`, \
             then create nodes and edges.\n\n",
        );
    } else {
        if !schema.nodes.is_empty() {
            prompt.push_str("Node schemas:\n");
            for node in &schema.nodes {
                prompt.push_str(&format!(
                    "- {} (key: {}): \"{}\"\n",
                    node.label, node.key, node.description
                ));
            }
            prompt.push('\n');
        }

        if !schema.edges.is_empty() {
            prompt.push_str("Edge schemas:\n");
            for edge in &schema.edges {
                prompt.push_str(&format!(
                    "- {} (key: {}): \"{}\"\n",
                    edge.label, edge.key, edge.description
                ));
            }
            prompt.push('\n');
        }
    }

    // Capabilities
    prompt.push_str("## Capabilities\n\n");
    prompt.push_str(
        "You can read from the graph (search, get nodes, explore neighbors, find paths) \
         and write to it (create schemas, create nodes, create edges, update nodes).\n\n",
    );

    // Behavioral rules
    prompt.push_str("## Rules\n\n");
    prompt.push_str(
        "1. **Reuse existing schemas.** Before creating a new schema, check if an existing one \
         fits. Only create schemas for genuinely new concepts.\n\
         2. **Entity resolution.** When creating a node, the system automatically searches for \
         similar existing nodes. If duplicates are found, review them carefully. Update the \
         existing node with merged information rather than creating duplicates.\n\
         3. **Normalize data.** Use consistent naming conventions and capitalization for schema \
         labels and property values.\n\
         4. **Extract thoroughly.** When processing a document, extract all relevant entities and \
         relationships. Don't skip information.\n\
         5. **Use the done tool.** When you have completed processing the user's request, call \
         the `done` tool with a summary of what was accomplished.\n\
         6. **Node properties are free-form.** You decide what properties to store on each node. \
         Include all relevant information as key-value pairs.\n\
         7. **Always use schema keys** (not labels) when creating nodes and edges.\n",
    );

    prompt
}
