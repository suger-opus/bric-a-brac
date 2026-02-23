import * as v from "valibot";
import { SendFormattedLabelDto, SendLabelDto } from "./utils-dto";

export enum PropertyType {
  NUMBER = "Number",
  // INTEGER = "integer",
  // FLOAT = "float",
  STRING = "String",
  BOOLEAN = "Boolean",
  // DATE = "date",
  // TIME = "time",
  // RANGE = "range",
  SELECT = "Select"
  // MULTISELECT = "multiselect"
}
const PropertyTypeDto = v.enum(PropertyType);

const PropertyMetadataDto = v.object({
  // min: v.nullable(v.number()),
  // max: v.nullable(v.number()),
  options: v.nullable(v.array(v.string()))
  // required: v.boolean()
});

export const PropertySchemaDto = v.object({
  property_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  property_type: PropertyTypeDto,
  metadata: PropertyMetadataDto
});

const CreatePropertySchemaMetadataOptionDto = v.pipe(
  v.string(),
  v.minLength(1, "Property option must be at least 1 characters long."),
  v.maxLength(50, "Property option must be at most 50 characters long.")
);

export const CreatePropertySchemaMetadataDto = v.pipe(
  v.object({
    options: v.nullable(
      v.pipe(
        v.array(CreatePropertySchemaMetadataOptionDto),
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

export const CreatePropertySchemaDto = v.pipe(
  v.object({
    label: SendLabelDto,
    formatted_label: SendFormattedLabelDto,
    metadata: CreatePropertySchemaMetadataDto,
    property_type: PropertyTypeDto
  }),
  v.check(
    (data) => {
      if (data.property_type === "Select") {
        return data.metadata.options !== null;
      }
      return true;
    },
    "Select type requires at least one option."
  ),
  v.check((data) => {
    if (data.property_type !== "Select") {
      return data.metadata.options === null;
    }
    return true;
  })
);
