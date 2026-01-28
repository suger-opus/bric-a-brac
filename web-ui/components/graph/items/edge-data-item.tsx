"use client";

import { useGraph } from "@/contexts/graph-context";
import { EdgeSchema, ProcessedEdgeData } from "@/types";
import { useEffect, useEffectEvent, useState } from "react";
import GraphElementItem from "./element-data-item";

const GraphEdgeItem = () => {
  const { metadata, schema, processedData, focusEdge } = useGraph();
  const [edgeData, setEdgeData] = useState<ProcessedEdgeData | null>(null);
  const [edgeSchema, setEdgeSchema] = useState<EdgeSchema | null>(null);

  const findEdge = useEffectEvent(() => {
    const foundData = processedData?.links.find((l) => l.id === focusEdge);
    const foundSchema = schema?.edges.find((n) => n.formatted_label === foundData?.formatted_label);
    setEdgeData(foundData || null);
    setEdgeSchema(foundSchema || null);
  });

  useEffect(() => {
    findEdge();
  }, [focusEdge]);

  return (edgeData && edgeSchema)
    ? (
      <GraphElementItem
        kind="edge"
        id={focusEdge!}
        label={edgeSchema.label}
        color={edgeSchema.color}
        schemaProperties={edgeSchema.properties}
        dataProperties={edgeData.properties}
        role={metadata!.user_role}
      />
    )
    : null;
};

export default GraphEdgeItem;
