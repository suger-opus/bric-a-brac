"use client";

import { EdgeForm } from "@/components/EdgeForm";
import { ForceGraph3DComponent } from "@/components/graph/ForceGraph3D";
import { NodeForm } from "@/components/NodeForm";
import { searchGraph } from "@/lib/api";
import { GraphData } from "@/types/graph";
import { useEffect, useState } from "react";

export default function Home() {
  const [graphId] = useState("00000000-0000-0000-0000-000000000000");
  const [graphData, setGraphData] = useState<GraphData>({
    nodes: [],
    edges: []
  });
  const [isLoading, setIsLoading] = useState(false);
  const [showNodeForm, setShowNodeForm] = useState(false);
  const [showEdgeForm, setShowEdgeForm] = useState(false);

  const loadGraph = async () => {
    try {
      setIsLoading(true);
      const data = await searchGraph(graphId);
      setGraphData(data);
    } catch (error) {
      console.error("Failed to load graph:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadGraph();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleNodeCreated = () => {
    setShowNodeForm(false);
    loadGraph();
  };

  const handleEdgeCreated = () => {
    setShowEdgeForm(false);
    loadGraph();
  };

  return (
    <div className="flex h-screen w-full bg-black">
      <aside className="w-80 border-r border-zinc-800 flex flex-col">
        <div className="p-4 border-b border-zinc-800">
          <h1 className="text-lg font-semibold text-zinc-50">
            Knowledge Graph
          </h1>
          <p className="text-sm text-zinc-400 mt-1">
            {graphData.nodes.length} nodes, {graphData.edges.length} edges
          </p>
        </div>

        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          <div className="space-y-2">
            <button
              onClick={() => {
                setShowNodeForm(!showNodeForm);
                setShowEdgeForm(false);
              }}
              className="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
            >
              {showNodeForm ? "Hide" : "Create Node"}
            </button>

            {showNodeForm && (
              <div className="p-4 rounded-md border border-zinc-800 bg-zinc-900/50">
                <NodeForm graphId={graphId} onNodeCreated={handleNodeCreated} />
              </div>
            )}
          </div>

          <div className="space-y-2">
            <button
              onClick={() => {
                setShowEdgeForm(!showEdgeForm);
                setShowNodeForm(false);
              }}
              className="w-full rounded-md bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
            >
              {showEdgeForm ? "Hide" : "Create Edge"}
            </button>

            {showEdgeForm && (
              <div className="p-4 rounded-md border border-zinc-800 bg-zinc-900/50">
                <EdgeForm graphId={graphId} onEdgeCreated={handleEdgeCreated} />
              </div>
            )}
          </div>

          <button
            onClick={loadGraph}
            className="w-full rounded-md border border-zinc-700 px-4 py-2 text-sm font-medium text-zinc-300 hover:bg-zinc-800"
          >
            Refresh Graph
          </button>

          {graphData.nodes.length > 0 && (
            <div className="pt-4 border-t border-zinc-800">
              <h3 className="text-sm font-medium text-zinc-300 mb-2">Nodes</h3>
              <div className="space-y-1 max-h-64 overflow-y-auto">
                {graphData.nodes.map((node) => (
                  <div
                    key={node.id}
                    className="p-2 rounded bg-zinc-900 border border-zinc-800 text-xs"
                  >
                    <div className="font-mono text-zinc-500 truncate">
                      {node.id}
                    </div>
                    <div className="text-zinc-300">{node.label}</div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </aside>

      <main className="flex-1 overflow-hidden">
        {isLoading
          ? (
            <div className="flex h-full items-center justify-center text-zinc-400">
              Loading graph...
            </div>
          )
          : <ForceGraph3DComponent data={graphData} />}
      </main>
    </div>
  );
}
