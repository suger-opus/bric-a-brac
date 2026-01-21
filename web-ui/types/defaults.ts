import { filterLabel, formatLabel } from "@/lib/utils";
import { PropertyType, RequestProperty } from ".";

export const defaultNewProperty: RequestProperty = {
  label: filterLabel(""),
  formatted_label: formatLabel(""),
  metadata: {
    property_type: PropertyType.STRING,
    details: { options: null, required: true }
  }
};
