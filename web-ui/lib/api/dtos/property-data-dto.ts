import * as v from "valibot";

export const PropertyValueDto = v.union([
  v.string(),
  v.number(),
  v.boolean()
]);

export const PropertiesDataDto = v.record(v.string(), PropertyValueDto);
