import * as v from "valibot";
import { PropertiesDataDto } from "./property-data-dto";

export const CreateEdgeDataDto = v.object({
  key: v.string(),
  from_node_data_id: v.string(),
  to_node_data_id: v.string(),
  properties: PropertiesDataDto
});

export const EdgeDataDto = v.object({
  edge_data_id: v.string(),
  key: v.string(),
  from_node_data_id: v.string(),
  to_node_data_id: v.string(),
  properties: PropertiesDataDto
});
