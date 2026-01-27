"use client";

import GraphCommand from "@/components/commands/graph-command";
import GraphEdgeItem from "@/components/items/graph-edge-item";
import GraphNodeItem from "@/components/items/graph-node-item";
import GraphSidebar from "@/components/sidebars/graph-sidebar";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { useGraph } from "@/contexts/graph-context";
import { sampleProcessedGraphData } from "@/lib/api/data";
import { EdgeSchema, NodeSchema, ProcessedEdgeData, ProcessedNodeData } from "@/types";
import dynamic from "next/dynamic";
import { useEffect, useRef, useState } from "react";
import SpriteText from "three-spritetext";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false
});

const GraphPage = () => {
  const {
    isLoaded,
    schema,
    processedData,
    displayedNodeProperties,
    displayedEdgeProperties
  } = useGraph();
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });
  const [focusNode, setFocusNode] = useState<
    { schema: NodeSchema; processedData: ProcessedNodeData; } | null
  >(null);
  const [focusEdge, setFocusEdge] = useState<
    { schema: EdgeSchema; processedData: ProcessedEdgeData; } | null
  >(null);
  const [openCommand, setOpenCommand] = useState(false);

  const handleNodeClick = (id: string) => {
    setOpenCommand(false);
    const foundData = processedData?.nodes.find((n) => n.id === id);
    const foundSchema = schema?.nodes.find((n) => n.formatted_label === foundData?.formatted_label);
    if (foundData && foundSchema && focusNode?.processedData.id !== id) {
      setFocusEdge(null);
      setFocusNode({ schema: foundSchema, processedData: foundData });
    } else {
      setFocusNode(null);
    }
  };

  const handleEdgeClick = (id: string) => {
    setOpenCommand(false);
    const foundData = processedData?.links.find((l) => l.id === id);
    const foundSchema = schema?.edges.find((n) => n.formatted_label === foundData?.formatted_label);
    if (foundData && foundSchema && focusEdge?.processedData.id !== id) {
      setFocusNode(null);
      setFocusEdge({ schema: foundSchema, processedData: foundData });
    } else {
      setFocusEdge(null);
    }
  };

  const handleBackgroundClick = () => {
    setOpenCommand(true);
  };

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

  return (
    <SidebarProvider
      className="w-screen h-screen"
      style={{
        "--sidebar-width": "30rem",
        "--sidebar-width-mobile": "16rem"
      } as React.CSSProperties}
    >
      <div className="w-full h-full relative overflow-hidden">
        <SidebarTrigger className="absolute right-2 top-2 z-20" />
        <div ref={containerRef} className="w-full h-full">
          {isLoaded
            ? (
              <div className="relative w-full h-full">
                <ForceGraph3D
                  graphData={processedData!}
                  backgroundColor="white"
                  width={dimensions.width}
                  height={dimensions.height}
                  onNodeClick={(e) => handleNodeClick(e.id as string)}
                  onLinkClick={(e) => handleEdgeClick(e.id as string)}
                  onBackgroundClick={() => handleBackgroundClick()}
                  nodeThreeObjectExtend={true}
                  nodeThreeObject={(node: ProcessedNodeData) => {
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
                  }}
                  linkThreeObjectExtend={true}
                  linkThreeObject={(link: ProcessedEdgeData) => {
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
                  }}
                  linkPositionUpdate={(sprite, { start, end }) => {
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
                  }}
                />
                {focusNode && (
                  <GraphNodeItem
                    schema={focusNode.schema}
                    processedData={focusNode.processedData}
                  />
                )}
                {focusEdge && (
                  <GraphEdgeItem
                    schema={focusEdge.schema}
                    processedData={focusEdge.processedData}
                  />
                )}
                <GraphCommand isOpen={openCommand} onOpenChange={setOpenCommand} />
              </div>
            )
            : (
              <div className="relative w-full h-full">
                <ForceGraph3D
                  graphData={sampleProcessedGraphData}
                  backgroundColor="white"
                  width={dimensions.width}
                  height={dimensions.height}
                />
              </div>
            )}
        </div>
      </div>
      <GraphSidebar />
    </SidebarProvider>
  );
};

export default GraphPage;
