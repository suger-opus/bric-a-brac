import { filterLabel } from "@/lib/utils";
import { CreatePropertySchema, PropertyType } from ".";

export const defaultNewProperty: CreatePropertySchema = {
  label: filterLabel(""),
  property_type: PropertyType.STRING,
  metadata: {
    options: null
  }
};
