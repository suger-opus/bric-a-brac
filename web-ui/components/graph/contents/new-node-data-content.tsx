"use client";

import { useGraph } from "@/contexts/graph-context";
import { useElementDataForm } from "@/hooks/use-element-data-form";
import { NodeSchema, PropertyType } from "@/types";
import { useEffect, useEffectEvent } from "react";
import NewElementDataContent from "./new-element-data-content";

const NewNodeDataContent = () => {
  const { schema } = useGraph();
  const { nodeSchemaId, properties } = useElementDataForm();

  const handleNodeSchemaChange = useEffectEvent(() => {
    properties.reset();
    if (nodeSchemaId.value !== null) {
      const selectedSchema = schema!.nodes.find((node: NodeSchema) =>
        node.node_schema_id === nodeSchemaId.value
      );
      if (selectedSchema) {
        const initialProperties = selectedSchema.properties.map((prop) => {
          const initialValue = prop.property_type === PropertyType.SELECT
            ? prop.metadata.options![0]
            : prop.property_type === PropertyType.STRING
            ? ""
            : prop.property_type === PropertyType.NUMBER
            ? 0
            : false; // BOOLEAN
          return {
            id: prop.property_id,
            isSaved: true,
            value: {
              property: prop,
              value: initialValue
            }
          };
        });
        properties.setValue(initialProperties);
      }
    }
  });

  useEffect(() => {
    handleNodeSchemaChange();
  }, [nodeSchemaId]);

  return (
    <NewElementDataContent
      kind="node"
      onSubmit={async () => {}}
      schemaOptions={schema!.nodes.map((node: NodeSchema) => ({
        id: node.node_schema_id,
        label: node.label,
        color: node.color,
        properties: node.properties
      }))}
      schemaId={nodeSchemaId}
      properties={properties}
    />
  );
};

export default NewNodeDataContent;
