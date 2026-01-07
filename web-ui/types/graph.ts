export interface Node {
  id: string;
  label: string;
  properties: Record<string, PropertyValue>;
}

export interface Edge {
  id: string;
  from_id: string;
  to_id: string;
  label: string;
  properties: Record<string, PropertyValue>;
}

export type PropertyValue = string | number | boolean;

export interface GraphData {
  nodes: Node[];
  edges: Edge[];
}

// For react-force-graph-3d
export interface ForceGraphNode {
  id: string;
  name: string;
  label: string;
  color?: string;
  val?: number;
}

export interface ForceGraphLink {
  source: string;
  target: string;
  label: string;
  color?: string;
}

export interface ForceGraphData {
  nodes: ForceGraphNode[];
  links: ForceGraphLink[];
}
