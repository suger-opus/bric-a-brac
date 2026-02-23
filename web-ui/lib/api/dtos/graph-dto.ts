import * as v from "valibot";
import { RoleDto } from "./access-dto";
import { EdgeDataDto } from "./edge-data-dto";
import { CreateEdgeSchemaDto, EdgeSchemaDto } from "./edge-schema-dto";
import { NodeDataDto } from "./node-data-dto";
import { CreateNodeSchemaDto, NodeSchemaDto } from "./node-schema-dto";

export const GraphSchemaDto = v.object({
  nodes: v.array(NodeSchemaDto),
  edges: v.array(EdgeSchemaDto)
});

export const GraphDataDto = v.object({
  nodes: v.array(NodeDataDto),
  edges: v.array(EdgeDataDto)
});

export const GraphMetadataDto = v.object({
  graph_id: v.string(),
  owner_username: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp()),
  name: v.string(),
  description: v.string(),
  user_role: RoleDto,
  is_public: v.boolean(),
  reddit: v.object({}),
  is_bookmarked_by_user: v.boolean(),
  is_cheered_by_user: v.boolean(),
  nb_data_nodes: v.number(),
  nb_data_edges: v.number(),
  nb_cheers: v.number(),
  nb_bookmarks: v.number()
});

const CreateGraphNameDto = v.pipe(
  v.string(),
  v.minLength(3, "Name must be at least 3 characters long."),
  v.maxLength(100, "Name must be at most 100 characters long.")
  // No character restrictions - allow any printable characters
);

const CreateGraphDescriptionDto = v.pipe(
  v.string(),
  // v.minLength(0, "Description must be at least 0 characters long."),
  v.maxLength(10000, "Description must be at most 10000 characters long.")
  // No character restrictions - allow rich descriptions
);

export const CreateGraphDto = v.object({
  name: CreateGraphNameDto,
  description: CreateGraphDescriptionDto,
  is_public: v.boolean()
});

export const CreateGraphSchemaDto = v.object({
  nodes: v.array(CreateNodeSchemaDto),
  edges: v.array(CreateEdgeSchemaDto)
});

const SearchGraphKeywordDto = v.pipe(
  v.string(),
  v.minLength(3, "Keyword must be at least 3 characters long."),
  v.maxLength(30, "Keyword must be at most 30 characters long.")
  // No character restrictions - allow users to search freely
);

export const SearchGraphDto = v.object({
  keyword: SearchGraphKeywordDto
});
