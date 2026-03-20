"use client";

import { useGraph } from "@/contexts/graph-context";
import { sampleProcessedGraphData } from "@/lib/api/data";
import type { ProcessedNodeData } from "@/types";
import dynamic from "next/dynamic";
import { useCallback, useRef } from "react";
import SpriteText from "three-spritetext";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false
});

type GraphProps = {
  dimensions: { width: number; height: number };
};

const Graph = ({ dimensions }: GraphProps) => {
  const {
    isLoaded,
    processedData,
    setFocusNode,
    setFocusEdge,
    focusNode,
    focusEdge,
  } = useGraph();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const graphRef = useRef<any>(null);

  const handleNodeClick = (id: string) => {
    setFocusEdge(null);
    setFocusNode(focusNode === id ? null : id);
  };

  const handleEdgeClick = (id: string) => {
    setFocusNode(null);
    setFocusEdge(focusEdge === id ? null : id);
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const zoomNode = useCallback((node: any) => {
    const distance = 40;
    const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z);
    graphRef.current?.cameraPosition(
      { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio },
      node,
      3000,
    );
  }, []);

  return (
    <ForceGraph3D
      ref={graphRef}
      graphData={isLoaded ? processedData! : sampleProcessedGraphData}
      backgroundColor="white"
      width={dimensions.width}
      height={dimensions.height}
      onNodeClick={isLoaded
        ? (e) => { handleNodeClick(e.id as string); zoomNode(e); }
        : undefined}
      onLinkClick={isLoaded ? (e) => handleEdgeClick(e.id as string) : undefined}
      nodeThreeObjectExtend={isLoaded}
      nodeThreeObject={isLoaded
        ? (node: ProcessedNodeData) => {
          const sprite = new SpriteText(node.key);
          sprite.color = node.color;
          sprite.textHeight = 4;
          sprite.offsetY = -12;
          return sprite;
        }
        : undefined}
      linkWidth={1}
    />
  );
};

export default Graph;
