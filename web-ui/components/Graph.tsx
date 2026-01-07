"use client";

import { useMemo } from "react";
import dynamic from "next/dynamic";
import { GraphData } from "@/types/graph";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false,
});

type GraphProps = {
  data: GraphData;
};

const NODE_COLORS: Record<string, string> = {
  Person: "#3b82f6",
  Company: "#f59e0b",
  default: "#6b7280",
};

const LINK_COLORS: Record<string, string> = {
  WORKS_AT: "#10b981",
  default: "#9ca3af",
};

const Graph = ({ data }: GraphProps) => {
  const graphData = useMemo(() => {
    const nodes = data.nodes.map((node) => {
      const name =
        node.properties.name && typeof node.properties.name === "string"
          ? node.properties.name
          : node.id;

      return {
        id: node.id,
        name,
        label: node.label,
        color: NODE_COLORS[node.label] || NODE_COLORS.default,
        val: 1,
      };
    });

    const links = data.edges.map((edge) => ({
      source: edge.from_id,
      target: edge.to_id,
      label: edge.label,
      color: LINK_COLORS[edge.label] || LINK_COLORS.default,
    }));

    return { nodes, links };
  }, [data]);

  return (
    <div className="w-full h-full">
      <ForceGraph3D graphData={graphData} />
    </div>
  );
};

export default Graph;
