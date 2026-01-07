"use client";

import { useState } from "react";
import { GraphData } from "@/types/graph";
import Modal from "./atoms/Modal";
import CreateEdgeForm from "./forms/CreateEdge";
import CreateNodeForm from "./forms/CreateNode";

type SidebarProps = {
  graphs: { id: string; name: string }[];
  currentGraphId: string | undefined;
  graphData: GraphData;
  onGraphSelect: (graphId: string) => void;
  onGraphCreate: (name: string) => void;
  onRefresh: () => void;
  onNodeCreated: () => void;
  onEdgeCreated: () => void;
};

const Sidebar = ({
  graphs,
  currentGraphId,
  graphData,
  onGraphSelect,
  onGraphCreate,
  onRefresh,
  onNodeCreated,
  onEdgeCreated,
}: SidebarProps) => {
  const [showNodeModal, setShowNodeModal] = useState(false);
  const [showEdgeModal, setShowEdgeModal] = useState(false);
  const [showCreateGraph, setShowCreateGraph] = useState(false);
  const [newGraphName, setNewGraphName] = useState("");

  const handleCreateGraph = (e: React.FormEvent) => {
    e.preventDefault();
    if (newGraphName.trim()) {
      onGraphCreate(newGraphName.trim());
      setNewGraphName("");
      setShowCreateGraph(false);
    }
  };

  const handleNodeCreated = () => {
    setShowNodeModal(false);
    onNodeCreated();
  };

  const handleEdgeCreated = () => {
    setShowEdgeModal(false);
    onEdgeCreated();
  };

  return (
    <>
      <aside className="w-80 border-r border-zinc-800 flex flex-col">
        <div className="p-4 border-b border-zinc-800">
          <h1 className="text-lg font-semibold text-zinc-50">
            Knowledge Graph
          </h1>
        </div>

        <div className="flex-1 overflow-y-auto">
          <div className="p-4 border-b border-zinc-800">
            <div className="flex items-center justify-between mb-3">
              <h2 className="text-sm font-medium text-zinc-300">Graphs</h2>
              <button
                onClick={() => setShowCreateGraph(!showCreateGraph)}
                className="text-xs text-blue-400 hover:text-blue-300"
              >
                + New Graph
              </button>
            </div>

            {showCreateGraph && (
              <form onSubmit={handleCreateGraph} className="mb-3">
                <input
                  type="text"
                  value={newGraphName}
                  onChange={(e) => setNewGraphName(e.target.value)}
                  placeholder="Graph name"
                  className="w-full rounded-md border border-zinc-700 bg-zinc-900 px-2 py-1.5 text-sm text-zinc-100 placeholder-zinc-500 focus:border-zinc-500 focus:outline-none mb-2"
                  autoFocus
                />
                <div className="flex gap-2">
                  <button
                    type="submit"
                    className="flex-1 rounded-md bg-blue-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-blue-700"
                  >
                    Create
                  </button>
                  <button
                    type="button"
                    onClick={() => {
                      setShowCreateGraph(false);
                      setNewGraphName("");
                    }}
                    className="flex-1 rounded-md border border-zinc-700 px-3 py-1.5 text-xs font-medium text-zinc-300 hover:bg-zinc-800"
                  >
                    Cancel
                  </button>
                </div>
              </form>
            )}

            <div className="space-y-1">
              {graphs.map((graph) => (
                <button
                  key={graph.id}
                  onClick={() => onGraphSelect(graph.id)}
                  className={`w-full rounded-md px-3 py-2 text-left text-sm transition-colors ${
                    graph.id === currentGraphId
                      ? "bg-blue-600 text-white"
                      : "text-zinc-300 hover:bg-zinc-800"
                  }`}
                >
                  {graph.name}
                </button>
              ))}
            </div>
          </div>

          {currentGraphId && (
            <div className="p-4 border-b border-zinc-800">
              <p className="text-sm text-zinc-400 mb-3">
                {graphData.nodes.length} nodes, {graphData.edges.length} edges
              </p>

              <div className="space-y-2">
                <button
                  onClick={() => setShowNodeModal(true)}
                  className="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
                >
                  Create Node
                </button>

                <button
                  onClick={() => setShowEdgeModal(true)}
                  className="w-full rounded-md bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
                >
                  Create Edge
                </button>

                <button
                  onClick={onRefresh}
                  className="w-full rounded-md border border-zinc-700 px-4 py-2 text-sm font-medium text-zinc-300 hover:bg-zinc-800"
                >
                  Refresh Graph
                </button>
              </div>
              <>
                <Modal
                  isOpen={showNodeModal}
                  onClose={() => setShowNodeModal(false)}
                  title="Create Node"
                >
                  <CreateNodeForm
                    graphId={currentGraphId}
                    onNodeCreated={handleNodeCreated}
                  />
                </Modal>

                <Modal
                  isOpen={showEdgeModal}
                  onClose={() => setShowEdgeModal(false)}
                  title="Create Edge"
                >
                  <CreateEdgeForm
                    graphId={currentGraphId}
                    graphData={graphData}
                    onEdgeCreated={handleEdgeCreated}
                  />
                </Modal>
              </>
            </div>
          )}
        </div>
      </aside>
    </>
  );
};

export default Sidebar;
