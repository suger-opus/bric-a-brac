"use client";

import LoadingGraphDialogContent from "@/components/dialog-contents/loading-graph-dialog-content";
import { Dialog, DialogOverlay } from "@/components/ui/dialog";
import { GraphProvider } from "@/contexts/graph-context";
import { useSearchParams } from "next/navigation";
import { useState } from "react";

const GraphLayout = ({ children }: Readonly<{ children: React.ReactNode; }>) => {
  const searchParams = useSearchParams();
  const graphId = searchParams.get("graph_id");
  const [isDialogOpen, setIsDialogOpen] = useState(true);

  return (
    <GraphProvider graphId={graphId}>
      <Dialog open={isDialogOpen}>
        <DialogOverlay className="bg-transparent backdrop-blur-xs" />
        <LoadingGraphDialogContent onClose={() => setIsDialogOpen(false)} />
      </Dialog>
      {children}
    </GraphProvider>
  );
};

export default GraphLayout;
