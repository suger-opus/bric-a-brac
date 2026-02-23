"use client";

import Graph from "@/components/graph/graph";
import GraphCommand from "@/components/graph/graph-command";
import GraphDrawer from "@/components/graph/graph-drawer";
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
  const [openCommand, setOpenCommand] = useState(false);

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

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key.toLowerCase() === "m") {
        e.preventDefault();
        setOpenCommand((prev) => !prev);
      }
    };
    document.addEventListener("keydown", handleKeyDown, true);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
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
          <div className="relative w-full h-full">
            <Graph dimensions={dimensions} setOpenCommand={setOpenCommand} />
            {isLoaded && (
              <>
                <div className="absolute top-2 left-2">
                  <GraphNodeItem />
                  <GraphEdgeItem />
                </div>
                <GraphCommand isOpen={openCommand} onOpenChange={setOpenCommand} />
                <GraphDrawer />
              </>
            )}
          </div>
        </div>
      </div>
      <GraphSidebar openCommand={() => setOpenCommand(true)} />
    </SidebarProvider>
  );
};

export default GraphPage;
