"use client";

import { NodeSchema, ProcessedNodeData } from "@/types";
import GraphElementItem from "./graph-element-item";

type GraphNodeItemProps = {
  schema: NodeSchema;
  processedData: ProcessedNodeData;
};

const GraphNodeItem = ({ schema, processedData }: GraphNodeItemProps) => {
  return (
    <GraphElementItem
      kind="node"
      id={processedData.id}
      label={schema.label}
      color={schema.color}
      schemaProperties={schema.properties}
      dataProperties={processedData.properties}
    />
  );
};

export default GraphNodeItem;
