"use client";

import GraphSidebar from "@/components/sidebars/graph";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";

const GraphLayout = ({ children }: { children: React.ReactNode; }) => {
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
        {children}
      </div>
      <GraphSidebar />
    </SidebarProvider>
  );
};

export default GraphLayout;
