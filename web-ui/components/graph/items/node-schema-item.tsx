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
      displayedProperty={displayedNodeProperties[schema.formatted_label]}
      updateDisplayedProperty={(property_formatted_label: string | undefined) =>
        updateDisplayedNodeProperty(schema.formatted_label, property_formatted_label)}
    />
  );
};

export default NodeSchemaItem;
