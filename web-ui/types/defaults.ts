import { filterLabel, formatLabel } from "@/lib/utils";
import { CreatePropertySchema, PropertyType } from ".";

export const defaultNewProperty: CreatePropertySchema = {
  label: filterLabel(""),
  formatted_label: formatLabel(""),
  property_type: PropertyType.STRING,
  metadata: {
    options: null
  }
};
