"use client";

import { GraphData } from "@/types/graph";
import dynamic from "next/dynamic";
import { useEffect, useMemo, useRef, useState } from "react";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false
});

const NODE_COLORS: Record<string, string> = {
  Person: "#3b82f6",
  Company: "#f59e0b",
  default: "#6b7280"
};

const LINK_COLORS: Record<string, string> = {
  WORKS_AT: "#10b981",
  default: "#9ca3af"
};

const data: GraphData = {
  nodes: [
    { id: "1", label: "Person", properties: { name: "Alice" } },
    { id: "2", label: "Company", properties: { name: "Acme Corp" } },
    { id: "3", label: "Person", properties: { name: "Bob" } }
  ],
  edges: [
    { id: "e1", from_id: "1", to_id: "2", label: "WORKS_AT", properties: {} },
    { id: "e2", from_id: "3", to_id: "2", label: "WORKS_AT", properties: {} }
  ]
};

const GraphPage = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });

  useEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        setDimensions({
          width: containerRef.current.offsetWidth,
          height: containerRef.current.offsetHeight
        });
      }
    };

    updateDimensions();

    const resizeObserver = new ResizeObserver(updateDimensions);
    if (containerRef.current) {
      resizeObserver.observe(containerRef.current);
    }

    return () => resizeObserver.disconnect();
  }, []);

  const graphData = useMemo(() => {
    const nodes = data.nodes.map((node) => {
      const name = node.properties.name && typeof node.properties.name === "string"
        ? node.properties.name
        : node.id;

      return {
        id: node.id,
        name,
        label: node.label,
        color: NODE_COLORS[node.label] || NODE_COLORS.default,
        val: 1
      };
    });

    const links = data.edges.map((edge) => ({
      source: edge.from_id,
      target: edge.to_id,
      label: edge.label,
      color: LINK_COLORS[edge.label] || LINK_COLORS.default
    }));

    return { nodes, links };
  }, [data]);

  return (
    <div ref={containerRef} className="w-full h-full">
      {dimensions.width > 0 && dimensions.height > 0 && (
        <ForceGraph3D
          graphData={graphData}
          backgroundColor="white"
          width={dimensions.width}
          height={dimensions.height}
        />
      )}
    </div>
  );
};

export default GraphPage;
