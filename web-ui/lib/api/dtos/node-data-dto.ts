import * as v from "valibot";
import { PropertiesDataDto } from "./property-data-dto";

export const CreateNodeDataDto = v.object({
  node_data_id: v.string(),
  key: v.string(),
  properties: PropertiesDataDto
});

export const NodeDataDto = v.object({
  node_data_id: v.string(),
  key: v.string(),
  properties: PropertiesDataDto
});
