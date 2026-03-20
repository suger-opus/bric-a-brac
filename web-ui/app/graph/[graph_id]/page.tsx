"use client";

import Graph from "@/components/graph/graph";
import GraphSidebar from "@/components/graph/graph-sidebar";
import GraphEdgeItem from "@/components/graph/items/edge-data-item";
import GraphNodeItem from "@/components/graph/items/node-data-item";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { useGraph } from "@/contexts/graph-context";
import { useEffect, useRef, useState } from "react";

const GraphPage = () => {
  const { isLoaded } = useGraph();
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });

  useEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        setDimensions({
          width: containerRef.current.offsetWidth,
          height: containerRef.current.offsetHeight,
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
        "--sidebar-width-mobile": "16rem",
      } as React.CSSProperties}
    >
      <div className="w-full h-full relative overflow-hidden">
        <SidebarTrigger className="absolute right-2 top-2 z-20" />
        <div ref={containerRef} className="w-full h-full">
          <div className="relative w-full h-full">
            <Graph dimensions={dimensions} />
            {isLoaded && (
              <div className="absolute top-2 left-2">
                <GraphNodeItem />
                <GraphEdgeItem />
              </div>
            )}
          </div>
        </div>
      </div>
      <GraphSidebar />
    </SidebarProvider>
  );
};

export default GraphPage;
