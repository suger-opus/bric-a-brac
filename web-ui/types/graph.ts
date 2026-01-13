export type SchemaNode = {
  name: string;
  color: string;
  nb_properties: number;
};

export type SchemaEdge = {
  name: string;
  color: string;
  nb_properties: number;
};

export type NodeData = {
  id: string;
  label: string;
  properties: Record<string, PropertyValue>;
};

export type EdgeData = {
  id: string;
  from_id: string;
  to_id: string;
  label: string;
  properties: Record<string, PropertyValue>;
};

export type PropertyValue = string | number | boolean;

export type GraphData = {
  nodes: NodeData[];
  edges: EdgeData[];
};
