import * as v from "valibot";

export const SendLabelDto = v.pipe(
  v.string(),
  v.minLength(3, "Label must be at least 3 characters long."),
  v.maxLength(25, "Label must be at most 25 characters long.")
  // v.regex(/^[a-zA-ZÀ-ÿ\s]+$/, "Label must contain only letters and spaces.")
);

export const SendColorDto = v.pipe(
  v.string(),
  v.hexColor("Color must be a valid hex color code.")
);
