import * as v from "valibot";
import { PropertiesDataDto } from "./property-data-dto";

export const CreateNodeDataDto = v.object({
  node_schema_id: v.string(),
  properties: PropertiesDataDto
});

export const NodeDataDto = v.object({
  node_data_id: v.string(),
  formatted_label: v.string(),
  properties: PropertiesDataDto
});
