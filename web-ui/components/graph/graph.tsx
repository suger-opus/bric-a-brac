"use client";

import { useGraph } from "@/contexts/graph-context";
import type { ProcessedEdgeData, ProcessedNodeData } from "@/types";
import { useTheme } from "next-themes";
import dynamic from "next/dynamic";
import { useCallback, useMemo, useRef } from "react";
import SpriteText from "three-spritetext";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false
});

type GraphProps = {
  dimensions: { width: number; height: number; };
};

/** Build a node label from the user-selected display property, or fall back to label. */
function buildNodeLabel(
  node: ProcessedNodeData,
  displayProperty: Record<string, string | null>
): string {
  const selected = displayProperty[node.key] ?? null;
  const mainValue = selected ? node.properties?.[selected] : null;
  return mainValue != null ? String(mainValue) : node.label;
}

const Graph = ({ dimensions }: GraphProps) => {
  const {
    isLoaded,
    processedData,
    setFocusNode,
    setFocusEdge,
    focusNode,
    focusEdge,
    displayProperty
  } = useGraph();
  const { resolvedTheme } = useTheme();

  const isDark = resolvedTheme === "dark";
  const bgColor = isDark ? "#1a1a1a" : "#ffffff";
  const spriteBg = isDark ? "rgba(30,30,30,0.8)" : "rgba(255,255,255,0.6)";

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
      3000
    );
  }, []);

  const nodeThreeObject = useMemo(() => {
    if (!isLoaded) { return undefined; }
    return (node: ProcessedNodeData) => {
      const text = buildNodeLabel(node, displayProperty);
      const sprite = new SpriteText(text);
      sprite.color = node.color;
      sprite.textHeight = 3;
      sprite.backgroundColor = spriteBg;
      sprite.padding = 1;
      sprite.borderRadius = 2;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (sprite as any).position.set(0, -12, 0);
      return sprite;
    };
  }, [isLoaded, displayProperty, spriteBg]);

  const linkThreeObject = useMemo(() => {
    return (link: ProcessedEdgeData) => {
      const sprite = new SpriteText(link.label);
      sprite.color = link.color ?? "#94a3b8";
      sprite.textHeight = 2;
      sprite.backgroundColor = spriteBg;
      sprite.padding = 0.5;
      sprite.borderRadius = 1;
      return sprite;
    };
  }, [spriteBg]);

  return (
    <ForceGraph3D
      ref={graphRef}
      graphData={isLoaded ? processedData! : { nodes: [], links: [] }}
      backgroundColor={bgColor}
      width={dimensions.width}
      height={dimensions.height}
      onNodeClick={isLoaded
        ? (e) => {
          handleNodeClick(e.id as string);
          zoomNode(e);
        }
        : undefined}
      onLinkClick={isLoaded ? (e) => handleEdgeClick(e.id as string) : undefined}
      nodeThreeObjectExtend={isLoaded}
      nodeThreeObject={nodeThreeObject}
      linkThreeObjectExtend
      linkThreeObject={linkThreeObject}
      linkPositionUpdate={(sprite, { start, end }) => {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const obj = sprite as any;
        obj.position.set(
          (start.x + end.x) / 2,
          (start.y + end.y) / 2,
          (start.z + end.z) / 2
        );
      }}
      linkWidth={1}
    />
  );
};

export default Graph;
