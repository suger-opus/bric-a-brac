"use client";

import { useGraph } from "@/contexts/graph-context";
import { useElementDataForm } from "@/hooks/use-element-data-form";
import { EdgeSchema, PropertyType } from "@/types";
import { useEffect, useEffectEvent } from "react";
import NewElementDataContent from "./new-element-data-content";

const NewEdgeDataContent = () => {
  const { schema, processedData } = useGraph();
  const { edgeSchemaId, properties, fromNodeId, toNodeId } = useElementDataForm();

  const handleEdgeSchemaChange = useEffectEvent(() => {
    properties.reset();
    if (edgeSchemaId.value !== null) {
      const selectedSchema = schema!.edges.find((edge: EdgeSchema) =>
        edge.edge_id === edgeSchemaId.value
      );
      if (selectedSchema) {
        const initialProperties = selectedSchema.properties.map((prop) => {
          const { property_type, details } = prop.metadata;
          const initialValue = property_type === PropertyType.SELECT
            ? details.options![0]
            : property_type === PropertyType.STRING
            ? ""
            : property_type === PropertyType.NUMBER
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
    handleEdgeSchemaChange();
  }, [edgeSchemaId]);

  return (
    <NewElementDataContent
      kind="edge"
      onSubmit={async () => {}}
      schemaOptions={schema!.edges.map((edge: EdgeSchema) => ({
        id: edge.edge_id,
        label: edge.label,
        color: edge.color,
        properties: edge.properties
      }))}
      schemaId={edgeSchemaId}
      properties={properties}
      fromNodeId={fromNodeId}
      toNodeId={toNodeId}
      nodeSchemas={schema!.nodes}
      nodeOptions={processedData!.nodes}
    />
  );
};

export default NewEdgeDataContent;
