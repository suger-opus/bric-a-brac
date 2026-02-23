import * as v from "valibot";
import { CreatePropertySchemaDto, PropertySchemaDto } from "./property-schema-dto";
import { SendColorDto, SendFormattedLabelDto, SendLabelDto } from "./utils-dto";

export const EdgeSchemaDto = v.object({
  edge_schema_id: v.string(),
  graph_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  color: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp()),
  properties: v.array(PropertySchemaDto)
});

export const CreateEdgeSchemaDto = v.object({
  label: SendLabelDto,
  formatted_label: SendFormattedLabelDto,
  color: SendColorDto,
  properties: v.array(CreatePropertySchemaDto)
});
