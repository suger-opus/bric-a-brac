import { Action } from "@/types";

export const Category = {
  SEARCH: "SEARCH",
  EDIT: "EDIT",
  ADMIN: "ADMIN",
  OWNER: "OWNER"
} as const;
export type Category = typeof Category[keyof typeof Category];

export const actionLabels: Record<Action, string> = {
  [Action.FIND_NODE]: "Find Node",
  [Action.FIND_PATH]: "Find Path",
  [Action.ASK_AI]: "Ask AI",
  [Action.BUILD_WITH_AI]: "Build with AI",
  [Action.NEW_NODE_TYPE]: "New Node Type",
  [Action.MANAGE_NODE_TYPES]: "Manage Node Types",
  [Action.INSERT_NODE]: "Insert Node",
  [Action.MANAGE_NODES]: "Manage Nodes",
  [Action.NEW_EDGE_TYPE]: "New Edge Type",
  [Action.MANAGE_EDGE_TYPES]: "Manage Edge Types",
  [Action.INSERT_EDGE]: "Insert Edge",
  [Action.MANAGE_EDGES]: "Manage Edges",
  [Action.METADATA]: "Metadata",
  [Action.ACCESSES]: "Accesses",
  [Action.VISIBILITY]: "Visibility",
  [Action.ANALYTICS]: "Analytics",
  [Action.DELETE_GRAPH]: "Delete Graph"
};

export const categoryLabels: Record<Category, string> = {
  [Category.SEARCH]: "Search",
  [Category.EDIT]: "Edit",
  [Category.ADMIN]: "Admin",
  [Category.OWNER]: "Owner"
};
