import { Edge, GraphData, Node } from "@/types/graph";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export interface CreateNodeRequest {
  graph_id: string;
  label: string;
  properties: Record<string, string | number | boolean>;
}

export interface CreateEdgeRequest {
  graph_id: string;
  from_id: string;
  to_id: string;
  label: string;
  properties: Record<string, string | number | boolean>;
}

export async function createNode(request: CreateNodeRequest): Promise<Node> {
  const response = await fetch(`${API_BASE_URL}/nodes`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(request)
  });

  if (!response.ok) {
    throw new Error(`Failed to create node: ${response.statusText}`);
  }

  return response.json();
}

export async function createEdge(request: CreateEdgeRequest): Promise<Edge> {
  const response = await fetch(`${API_BASE_URL}/edges`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(request)
  });

  if (!response.ok) {
    throw new Error(`Failed to create edge: ${response.statusText}`);
  }

  return response.json();
}

export async function searchGraph(graphId: string): Promise<GraphData> {
  const response = await fetch(`${API_BASE_URL}/search`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      graph_id: graphId,
      include_edges: true
    })
  });

  if (!response.ok) {
    throw new Error(`Failed to search graph: ${response.statusText}`);
  }

  return response.json();
}
