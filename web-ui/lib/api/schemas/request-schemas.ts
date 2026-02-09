import * as v from "valibot";
import { propertyType, propertyValue } from "./response-schemas";

export const requestSearchKeyword = v.pipe(
  v.string(),
  v.minLength(3, "Keyword must be at least 3 characters long."),
  v.maxLength(30, "Keyword must be at most 30 characters long.")
  // No character restrictions - allow users to search freely
);

export const requestGraphName = v.pipe(
  v.string(),
  v.minLength(5, "Name must be at least 5 characters long."),
  v.maxLength(50, "Name must be at most 50 characters long.")
  // No character restrictions - allow any printable characters
);

export const requestGraphDescription = v.pipe(
  v.string(),
  v.minLength(25, "Description must be at least 25 characters long."),
  v.maxLength(250, "Description must be at most 250 characters long.")
  // No character restrictions - allow rich descriptions
);

export const requestLabel = v.pipe(
  v.string(),
  v.minLength(3, "Label must be at least 3 characters long."),
  v.maxLength(30, "Label must be at most 30 characters long."),
  v.regex(/^[a-zA-ZÀ-ÿ\s]+$/, "Label must contain only letters and spaces.")
);

export const requestFormattedLabel = v.pipe(
  v.string(),
  v.minLength(3, "Formatted label must be at least 3 characters long."),
  v.maxLength(30, "Formatted label must be at most 30 characters long."),
  v.regex(
    /^([A-Z][a-z]*_)*[A-Z][a-z]*$/,
    "Formatted label must be in TitleCase separated by underscores (e.g., 'Battle_Name')."
  )
);

export const requestColor = v.pipe(
  v.string(),
  v.hexColor("Color must be a valid hex color code.")
);

export const requestPropertyMetadataDetailsOption = v.pipe(
  v.string(),
  v.minLength(3, "Property option must be at least 3 characters long."),
  v.maxLength(30, "Property option must be at most 30 characters long.")
);

export const requestPropertyMetadata = v.pipe(
  v.object({
    options: v.nullable(
      v.pipe(
        v.array(requestPropertyMetadataDetailsOption),
        v.minLength(1, "At least one option is required.")
      )
    )
  })
  // v.check(
  //   (data) => {
  //     if (data.property_type === "range") {
  //       return data.details.min !== null && data.details.max !== null;
  //     }
  //     return true;
  //   },
  //   "Range type requires min and max values."
  // ),
  // v.check(
  //   (data) => {
  //     if (data.property_type === "range") {
  //       return data.details.min !== null && data.details.max !== null
  //         && data.details.min >= data.details.max;
  //     }
  //     return true;
  //   },
  //   "Range type requires max to be strictly superior to min."
  // ),
  // v.check(
  //   (data) => {
  //     if (data.property_type === "multiselect") {
  //       return data.details.options !== null && data.details.options.length > 0;
  //     }
  //     return true;
  //   },
  //   "Multiselect type requires at least one option."
  // )
);

export const requestProperty = v.pipe(
  v.object({
    label: requestLabel,
    formatted_label: requestFormattedLabel,
    metadata: requestPropertyMetadata,
    property_type: propertyType
  }),
  v.check(
    (data) => {
      if (data.property_type === "select") {
        return data.metadata.options !== null;
      }
      return true;
    },
    "Select type requires at least one option."
  ),
  v.check((data) => {
    if (data.property_type !== "select") {
      return data.metadata.options === null;
    }
    return true;
  })
);

export const requestPropertyData = v.object({
  property_id: v.string(),
  value: propertyValue
});

export const requestNodeData = v.object({
  properties: v.array(requestPropertyData)
});

export const requestEdgeData = v.object({
  from_id: v.string(),
  to_id: v.string(),
  properties: v.array(requestPropertyData)
});

export const requestNodeSchema = v.object({
  label: requestLabel,
  formatted_label: requestFormattedLabel,
  color: requestColor,
  properties: v.array(requestProperty)
});

export const requestEdgeSchema = v.object({
  label: requestLabel,
  formatted_label: requestFormattedLabel,
  color: requestColor,
  properties: v.array(requestProperty)
});

export const requestGraph = v.object({
  name: requestGraphName,
  description: requestGraphDescription
});

export const requestSearch = v.object({
  keyword: requestSearchKeyword
});
