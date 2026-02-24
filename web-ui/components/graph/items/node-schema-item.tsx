"use client";

import { useGraph } from "@/contexts/graph-context";
import { NodeSchema } from "@/types";
import ElementSchemaItem from "./element-schema-item";

type NodeSchemaItemProps = {
  schema: NodeSchema;
};

const NodeSchemaItem = ({ schema }: NodeSchemaItemProps) => {
  const { displayedNodeProperties, updateDisplayedNodeProperty } = useGraph();

  return (
    <ElementSchemaItem
      kind="node"
      label={schema.label}
      color={schema.color}
      properties={schema.properties}
      displayedProperty={displayedNodeProperties[schema.key]}
      updateDisplayedProperty={(property_key: string | undefined) =>
        updateDisplayedNodeProperty(schema.key, property_key)}
    />
  );
};

export default NodeSchemaItem;
