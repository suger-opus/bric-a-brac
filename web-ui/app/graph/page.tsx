"use client";

import GraphSidebar from "@/components/sidebars/graph-sidebar";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { Spinner } from "@/components/ui/spinner";
import { useGraph } from "@/contexts/graph-context";
import dynamic from "next/dynamic";
import { useEffect, useRef, useState } from "react";

const ForceGraph3D = dynamic(() => import("react-force-graph-3d"), {
  ssr: false
});

const GraphPage = () => {
  const { isLoading, processedData } = useGraph();
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
          {isLoading && (
            <div className="flex h-full items-center justify-center">
              <Spinner />
            </div>
          )}
          {dimensions.width > 0 && dimensions.height > 0 && !isLoading
            && processedData
            && (
              <ForceGraph3D
                graphData={processedData}
                backgroundColor="white"
                width={dimensions.width}
                height={dimensions.height}
              />
            )}
        </div>
      </div>
      <GraphSidebar />
    </SidebarProvider>
  );
};

export default GraphPage;
