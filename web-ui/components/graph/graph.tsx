"use client";

import { useGraph } from "@/contexts/graph-context";
import { sampleProcessedGraphData } from "@/lib/api/data";
import { ProcessedEdgeData, ProcessedNodeData } from "@/types";
import dynamic from "next/dynamic";
import { useCallback, useRef } from "react";
import SpriteText from "three-spritetext";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false
});

type GraphProps = {
  dimensions: {
    width: number;
    height: number;
  };
  setOpenCommand: (open: boolean) => void;
};

const Graph = ({ dimensions, setOpenCommand }: GraphProps) => {
  const {
    isLoaded,
    processedData,
    displayedNodeProperties,
    displayedEdgeProperties,
    setFocusNode,
    setFocusEdge,
    focusNode,
    focusEdge
  } = useGraph();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const graphRef = useRef<any>(null);

  const handleNodeClick = (id: string) => {
    setOpenCommand(false);
    setFocusEdge(null);
    if (focusNode === id) {
      setFocusNode(null);
    } else {
      setFocusNode(id);
    }
  };

  const handleEdgeClick = (id: string) => {
    setOpenCommand(false);
    setFocusNode(null);
    if (focusEdge === id) {
      setFocusEdge(null);
    } else {
      setFocusEdge(id);
    }
  };

  const handleBackgroundClick = () => {
    setOpenCommand(true);
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const zoomNode = useCallback((node: any) => {
    const distance = 40;
    const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z);

    if (graphRef.current) {
      graphRef.current.cameraPosition(
        { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio }, // new position
        node, // lookAt ({ x, y, z })
        3000 // ms transition duration
      );
    }
  }, [graphRef]);

  return (
    <ForceGraph3D
      ref={graphRef}
      graphData={isLoaded ? processedData! : sampleProcessedGraphData}
      backgroundColor="white"
      width={dimensions.width}
      height={dimensions.height}
      onNodeClick={isLoaded
        ? (e) => {
          handleNodeClick(e.id as string);
          zoomNode(e);
        }
        : undefined}
      onLinkClick={isLoaded ? (e) => handleEdgeClick(e.id as string) : undefined}
      onBackgroundClick={isLoaded ? () => handleBackgroundClick() : undefined}
      nodeThreeObjectExtend={isLoaded}
      nodeThreeObject={isLoaded
        ? (node: ProcessedNodeData) => {
          const property = node.formatted_label in displayedNodeProperties
            ? displayedNodeProperties[node.formatted_label]
            : undefined;
          const sprite = property && property in node.properties
            ? new SpriteText(node.properties[property] as string)
            : undefined;
          if (sprite) {
            sprite.color = node.color;
            sprite.textHeight = 4;
            sprite.offsetY = -12;
          }
          return sprite;
        }
        : undefined}
      linkThreeObjectExtend={isLoaded}
      linkThreeObject={isLoaded
        ? (link: ProcessedEdgeData) => {
          const property = link.formatted_label in displayedEdgeProperties
            ? displayedEdgeProperties[link.formatted_label]
            : undefined;
          const sprite = property && property in link.properties
            ? new SpriteText(link.properties[property] as string)
            : undefined;
          if (sprite) {
            sprite.color = link.color;
            sprite.textHeight = 1.5;
          }
          return sprite;
        }
        : undefined}
      linkPositionUpdate={isLoaded
        ? (sprite, { start, end }) => {
          if (sprite) {
            const coords: Array<keyof typeof start> = ["x", "y", "z"];
            const middlePos = coords.reduce(
              (acc, c) => {
                acc[c] = start[c] + (end[c] - start[c]) / 2;
                return acc;
              },
              {} as { x: number; y: number; z: number; }
            );
            Object.assign(sprite.position, middlePos);
          }
        }
        : undefined}
    />
  );
};

export default Graph;
