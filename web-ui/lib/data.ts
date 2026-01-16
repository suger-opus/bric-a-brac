import { GraphData, GraphMetadata, GraphSchema, PropertyType, Role } from "@/types/graph";
import { User } from "@/types/user";

export const users: User[] = [
  {
    user_id: "xxxx-xxxx-xxxx-xxxx-xxxx",
    username: "napoleon",
    email: "napoleon@example.com",
    created_at: new Date()
  }
];

export const graphs: GraphMetadata[] = [{
  graph_id: "graph-1",
  owner_username: "username",
  user_role: Role.ADMIN,
  is_bookmarked_by_user: false,
  is_cheered_by_user: true,
  created_at: new Date(),
  updated_at: new Date(),
  name: "Graph One",
  description: "This is the description for Graph One.",
  is_public: true,
  nb_data_nodes: 150,
  nb_data_edges: 300,
  nb_cheers: 25,
  nb_bookmarks: 10
}, {
  graph_id: "graph-2",
  owner_username: "username",
  user_role: Role.EDITOR,
  is_bookmarked_by_user: true,
  is_cheered_by_user: false,
  created_at: new Date(),
  updated_at: new Date(),
  name: "Graph Two",
  description: "This is the description for Graph Two.",
  is_public: false,
  nb_data_nodes: 80,
  nb_data_edges: 120,
  nb_cheers: 0,
  nb_bookmarks: 0
}, {
  graph_id: "graph-3",
  owner_username: "username",
  user_role: Role.VIEWER,
  is_bookmarked_by_user: false,
  is_cheered_by_user: false,
  created_at: new Date(),
  updated_at: new Date(),
  name: "Graph Three",
  description: "This is the description for Graph Three.",
  is_public: true,
  nb_data_nodes: 200,
  nb_data_edges: 450,
  nb_cheers: 40,
  nb_bookmarks: 20
}, {
  graph_id: "graph-4",
  owner_username: "username",
  user_role: Role.OWNER,
  is_bookmarked_by_user: false,
  is_cheered_by_user: false,
  created_at: new Date(),
  updated_at: new Date(),
  name: "Graph Four",
  description: "This is the description for Graph Four.",
  is_public: false,
  nb_data_nodes: 60,
  nb_data_edges: 90,
  nb_cheers: 0,
  nb_bookmarks: 0
}, {
  graph_id: "graph-5",
  owner_username: "username",
  user_role: Role.VIEWER,
  is_bookmarked_by_user: true,
  is_cheered_by_user: false,
  created_at: new Date(),
  updated_at: new Date(),
  name:
    "Very very veeeeeeeeeery long name for Graph Five Very very veeeeeeeeeery long name for Graph Five",
  description: "This is the description for Graph Five.",
  is_public: true,
  nb_data_nodes: 120,
  nb_data_edges: 240,
  nb_cheers: 15,
  nb_bookmarks: 5
}, {
  graph_id: "graph-6",
  owner_username: "username",
  user_role: Role.ADMIN,
  is_bookmarked_by_user: false,
  is_cheered_by_user: false,
  created_at: new Date(),
  updated_at: new Date(),
  name: "Graph Six",
  description: "This is the description for Graph Six.",
  is_public: false,
  nb_data_nodes: 90,
  nb_data_edges: 150,
  nb_cheers: 0,
  nb_bookmarks: 0
}];

export const graphMetadata: GraphMetadata = {
  graph_id: "graph-0",
  owner_username: "suger-opus",
  user_role: Role.OWNER,
  is_bookmarked_by_user: false,
  is_cheered_by_user: false,
  created_at: new Date(),
  updated_at: new Date(),
  name: "Acme Corp. Employee Network",
  description:
    "A comprehensive graph representing the employees, departments, and projects within Acme Corporation.",
  is_public: false,
  nb_data_nodes: 3,
  nb_data_edges: 2,
  nb_cheers: 0,
  nb_bookmarks: 0
};

export const graphSchema: GraphSchema = {
  nodes: [
    {
      node_id: "node-schema-1",
      label: "Person",
      formated_label: "Person",
      color: "#3b82f6",
      properties: [{
        property_id: "property_1",
        name: "name",
        formated_name: "name",
        metadata: {
          property_type: PropertyType.STRING,
          details: {
            min: null,
            max: null,
            options: null,
            required: true,
            default_value: null
          }
        }
      }]
    },
    {
      node_id: "node-schema-2",
      label: "Company",
      formated_label: "Company",
      color: "#f59e0b",
      properties: [{
        property_id: "p12",
        name: "name",
        formated_name: "name",
        metadata: {
          property_type: PropertyType.STRING,
          details: {
            min: null,
            max: null,
            options: null,
            required: true,
            default_value: null
          }
        }
      }]
    }
  ],
  edges: [
    {
      edge_id: "edge-schema-1",
      label: "WORKS_AT",
      formated_label: "WORKS_AT",
      color: "#10b981",
      properties: []
    }
  ]
};

export const graphData: GraphData = {
  nodes: [
    {
      graph_id: "graph-0",
      node_id: "node-data-1",
      label: "Person",
      properties: { name: "Alice" }
    },
    {
      graph_id: "graph-0",
      node_id: "node-data-2",
      label: "Company",
      properties: { name: "Acme Corp" }
    },
    {
      graph_id: "graph-0",
      node_id: "node-data-3",
      label: "Person",
      properties: { name: "Bob" }
    }
  ],
  edges: [
    {
      graph_id: "graph-0",
      edge_id: "edge_data_1",
      from_id: "node-data-1",
      to_id: "node-data-2",
      label: "WORKS_AT",
      properties: {}
    },
    {
      graph_id: "graph-0",
      edge_id: "edge_data_2",
      from_id: "node-data-3",
      to_id: "node-data-2",
      label: "WORKS_AT",
      properties: {}
    }
  ]
};
