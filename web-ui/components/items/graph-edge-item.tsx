"use client";

import { EdgeSchema, ProcessedEdgeData } from "@/types";
import GraphElementItem from "./graph-element-item";

type GraphEdgeItemProps = {
  schema: EdgeSchema;
  processedData: ProcessedEdgeData;
};

const GraphEdgeItem = ({ schema, processedData }: GraphEdgeItemProps) => {
  return (
    <GraphElementItem
      kind="edge"
      id={processedData.id}
      label={schema.label}
      color={schema.color}
      schemaProperties={schema.properties}
      dataProperties={processedData.properties}
    />
  );
};

export default GraphEdgeItem;
