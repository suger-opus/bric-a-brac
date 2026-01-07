"use client";

import Graph from "@/components/Graph";
import Sidebar from "@/components/Sidebar";
import { searchGraph } from "@/lib/api";
import { GraphData } from "@/types/graph";
import { useEffect, useState } from "react";

export default function Home() {
  const [graphs, setGraphs] = useState<{ id: string; name: string }[]>([]);
  const [currentGraphId, setCurrentGraphId] = useState<string | undefined>(
    undefined
  );
  const [graphData, setGraphData] = useState<GraphData>({
    nodes: [],
    edges: [],
  });
  const [isLoading, setIsLoading] = useState(false);

  const loadGraph = async () => {
    try {
      if (!currentGraphId) return;
      setIsLoading(true);
      const data = await searchGraph(currentGraphId);
      setGraphData(data);
    } catch (error) {
      console.error("Failed to load graph:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadGraph();
  }, [currentGraphId]);

  const handleGraphSelect = (graphId: string) => {
    setCurrentGraphId(graphId);
  };

  const handleGraphCreate = (name: string) => {
    const newGraphId = crypto.randomUUID();
    setGraphs([...graphs, { id: newGraphId, name }]);
    setCurrentGraphId(newGraphId);
  };

  return (
    <div className="flex h-screen w-full bg-black">
      <Sidebar
        graphs={graphs}
        currentGraphId={currentGraphId}
        graphData={graphData}
        onGraphSelect={handleGraphSelect}
        onGraphCreate={handleGraphCreate}
        onRefresh={loadGraph}
        onNodeCreated={loadGraph}
        onEdgeCreated={loadGraph}
      />

      <main className="flex-1 overflow-hidden">
        {isLoading ? (
          <div className="flex h-full items-center justify-center text-zinc-400">
            Loading graph...
          </div>
        ) : (
          <Graph data={graphData} />
        )}
      </main>
    </div>
  );
}
