import * as v from "valibot";
import { RoleDto } from "./access-dto";

export const GraphMetadataDto = v.object({
  graph_id: v.string(),
  owner_username: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp()),
  name: v.string(),
  description: v.string(),
  user_role: RoleDto,
  is_public: v.boolean()
});

const CreateGraphNameDto = v.pipe(
  v.string(),
  v.minLength(3, "Name must be at least 3 characters long."),
  v.maxLength(100, "Name must be at most 100 characters long.")
);

const CreateGraphDescriptionDto = v.pipe(
  v.string(),
  v.maxLength(10000, "Description must be at most 10000 characters long.")
);

export const CreateGraphDto = v.object({
  name: CreateGraphNameDto,
  description: CreateGraphDescriptionDto,
  is_public: v.boolean()
});
