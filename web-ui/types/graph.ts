export type GraphMetadata = {
  graph_id: string;
  owner_username: string;
  created_at: Date;
  updated_at: Date;
  name: string;
  description: string;
  user_role: Role;
  is_public: boolean;
  is_bookmarked_by_user: boolean;
  is_cheered_by_user: boolean;
  nb_data_nodes: number;
  nb_data_edges: number;
  nb_cheers: number;
  nb_bookmarks: number;
};

export enum Role {
  OWNER = "owner",
  ADMIN = "admin",
  EDITOR = "editor",
  VIEWER = "viewer",
  NONE = "none"
}

export type GraphSchema = {
  nodes: NodeSchema[];
  edges: EdgeSchema[];
};

export type NodeSchema = {
  node_id: string;
  label: string; // Must be unique per graph (including edge labels)
  formated_label: string; // Must be unique per graph (including edge labels)
  color: string;
  properties: Property[];
};

export type EdgeSchema = {
  edge_id: string;
  label: string; // Must be unique per graph (including node labels)
  formated_label: string; // Must be unique per graph (including node labels)
  color: string;
  properties: Property[];
};

export type Property = {
  property_id: string;
  name: string; // ex: "nombre d'enfants" (must be uniques per node/edge)
  formated_name: string; // ex: "nombre_d_enfants" (must be uniques per node/edge)
  metadata: {
    property_type: PropertyType;
    details: {
      min: number | null;
      max: number | null;
      options: string[] | null;
      required: boolean;
      default_value: PropertyValue | null;
    };
  };
};

export enum PropertyType {
  INTEGER = "integer",
  FLOAT = "float",
  STRING = "string",
  BOOLEAN = "boolean",
  DATE = "date",
  TIME = "time",
  RANGE = "range",
  SELECT = "select",
  MULTISELECT = "multiselect"
}

export type PropertyValue = string | number | boolean;

export type GraphData = {
  nodes: NodeData[];
  edges: EdgeData[];
};

export type NodeData = {
  graph_id: string;
  node_id: string;
  label: string;
  properties: Record<string, PropertyValue>;
};

export type EdgeData = {
  graph_id: string;
  edge_id: string;
  from_id: string;
  to_id: string; // Couple (from_id to_id) with label must be unique per graph
  label: string;
  properties: Record<string, PropertyValue>;
};

export type ProcessedGraphData = {
  nodes: ProcessedNodeData[];
  links: ProcessedEdgeData[];
};

export type ProcessedNodeData = {
  id: string;
  label: string;
  color: string;
};

export type ProcessedEdgeData = {
  source: string;
  target: string;
  label: string;
  color: string;
};
