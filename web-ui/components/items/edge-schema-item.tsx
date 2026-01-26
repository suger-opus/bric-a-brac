"use client";

import { useGraph } from "@/contexts/graph-context";
import { EdgeSchema } from "@/types";
import ElementSchemaItem from "./element-schema-item";

type EdgeSchemaItemProps = {
  schema: EdgeSchema;
};

const EdgeSchemaItem = ({ schema }: EdgeSchemaItemProps) => {
  const { displayedEdgeProperties, updateDisplayedEdgeProperty } = useGraph();

  return (
    <ElementSchemaItem
      kind="edge"
      label={schema.label}
      color={schema.color}
      properties={schema.properties}
      displayedProperty={displayedEdgeProperties[schema.formatted_label]}
      updateDisplayedProperty={(property_formatted_label: string | undefined) =>
        updateDisplayedEdgeProperty(schema.formatted_label, property_formatted_label)}
    />
  );
};

export default EdgeSchemaItem;
