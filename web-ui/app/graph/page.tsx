"use client";

import GraphSidebar from "@/components/sidebars/graph-sidebar";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { Spinner } from "@/components/ui/spinner";
import { ApiProvider } from "@/lib/api/provider";
import { GraphData, GraphMetadata, GraphSchema, ProcessedGraphData } from "@/types";
import dynamic from "next/dynamic";
import { useEffect, useRef, useState } from "react";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false
});

// todo: move this in backend ?
const processGraphData = (
  { graphData, graphSchema }: { graphData: GraphData; graphSchema: GraphSchema; }
): ProcessedGraphData => {
  const nodes = graphData.nodes.map((node) => {
    const nodeSchema = graphSchema.nodes.find((n) => n.formatted_label === node.label);
    const color = nodeSchema ? nodeSchema.color : "#888888";
    return {
      id: node.node_id,
      label: node.label,
      color
    };
  });

  const links = graphData.edges.map((edge) => {
    const edgeSchema = graphSchema.edges.find((e) => e.formatted_label === edge.label);
    const color = edgeSchema ? edgeSchema.color : "#888888";
    return {
      source: edge.from_id,
      target: edge.to_id,
      label: edge.label,
      color
    };
  });

  return { nodes, links };
};

const GraphPage = () => {
  const { graphService } = ApiProvider;
  const [graphMetadata, setGraphMetadata] = useState<GraphMetadata | null>(null);
  const [graphSchema, setGraphSchema] = useState<GraphSchema | null>(null);
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [graphData, setGraphData] = useState<GraphData | null>(null);
  const [isGraphLoading, setIsGraphLoading] = useState(true);
  const [processedGraphData, setProcessedGraphData] = useState<ProcessedGraphData | null>(null);
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

  const getGraph = async () => {
    try {
      setIsGraphLoading(true);
      const metadata = await graphService.getMetadata("graph-0");
      setGraphMetadata(metadata);
      const schema = await graphService.getSchema("graph-0");
      setGraphSchema(schema);
      const data = await graphService.getData("graph-0");
      setGraphData(data);
      setProcessedGraphData(processGraphData({ graphData: data, graphSchema: schema }));
      setIsGraphLoading(false);
    } catch (error) {
      console.error("Error fetching graph data:", error);
    } finally {
      setIsGraphLoading(false);
    }
  };

  useEffect(() => {
    getGraph();
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
        <SidebarTrigger className="absolute right-2 top-2 z-900" />
        <div ref={containerRef} className="w-full h-full">
          {isGraphLoading && (
            <div className="flex h-full items-center justify-center">
              <Spinner />
            </div>
          )}
          {dimensions.width > 0 && dimensions.height > 0 && !isGraphLoading && processedGraphData
            && (
              <ForceGraph3D
                graphData={processedGraphData}
                backgroundColor="white"
                width={dimensions.width}
                height={dimensions.height}
              />
            )}
        </div>
      </div>
      <GraphSidebar graphMetadata={graphMetadata} graphSchema={graphSchema} />
    </SidebarProvider>
  );
};

export default GraphPage;
