import {
  CreatePropertySchemaDto,
  SendColorDto,
  SendFormattedLabelDto,
  SendLabelDto
} from "@/lib/api/dtos";
import { formatLabel } from "@/lib/utils";
import { CreatePropertySchema, FormInput, FormInputs } from "@/types";
import { useState } from "react";
import * as v from "valibot";

type UseElementSchemaFormReturn = {
  label: FormInput<string>;
  formattedLabel: FormInput<string>;
  color: FormInput<string>;
  properties: FormInputs<CreatePropertySchema>;
  submitNode: () => Promise<void>;
  submitEdge: () => Promise<void>;
};

export const useElementSchemaForm = (): UseElementSchemaFormReturn => {
  const [label, setLabel] = useState("");
  const [labelValidationError, setLabelValidationError] = useState<string | null>(null);
  const formattedLabel = formatLabel(label);
  const [formattedLabelValidationError, setFormattedLabelValidationError] = useState<string | null>(
    null
  );
  const [color, setColor] = useState("#3b82f6");
  const [colorValidationError, setColorValidationError] = useState<string | null>(null);
  const [properties, setProperties] = useState<
    { id: string; isSaved: boolean; value: CreatePropertySchema; }[]
  >([]);
  const [propertyErrors, setPropertyErrors] = useState<Record<string, string | null>>({});

  const validateLabel = () => {
    const validation = v.safeParse(SendLabelDto, label);
    if (!validation.success) {
      setLabelValidationError(validation.issues[0].message);
      return false;
    }
    setLabelValidationError(null);
    return true;
  };

  const validateFormattedLabel = () => {
    const validation = v.safeParse(SendFormattedLabelDto, formattedLabel);
    if (!validation.success) {
      setFormattedLabelValidationError(validation.issues[0].message);
      return false;
    }
    setFormattedLabelValidationError(null);
    return true;
  };

  const validateColor = () => {
    const validation = v.safeParse(SendColorDto, color);
    if (!validation.success) {
      setColorValidationError(validation.issues[0].message);
      return false;
    }
    setColorValidationError(null);
    return true;
  };

  const validateOneProperty = (id: string) => {
    const property = properties.find((prop) => prop.id === id);
    if (!property) {
      setPropertyErrors((prev) => ({
        ...prev,
        [id]: null
      }));
      return true;
    }
    if (!property.isSaved) {
      setPropertyErrors((prev) => ({
        ...prev,
        [property.id]: "Save this property to continue."
      }));
      return false;
    }
    const validProperty = v.safeParse(CreatePropertySchemaDto, property.value);
    if (!validProperty.success) {
      setPropertyErrors((prev) => ({
        ...prev,
        [property.id]: validProperty.issues[0].message
      }));
      return false;
    }
    setPropertyErrors((prev) => ({ ...prev, [property.id]: null }));
    return true;
  };

  const validateAllProperties = () => {
    return properties.every((property) => validateOneProperty(property.id));
  };

  const resetLabel = () => {
    setLabel("");
    setLabelValidationError(null);
  };

  const resetFormattedLabel = () => {
    setFormattedLabelValidationError(null);
  };

  const resetColor = () => {
    setColor("#3b82f6");
    setColorValidationError(null);
  };

  const resetProperties = () => {
    setProperties([]);
    setPropertyErrors({});
  };

  const submitNode = async () => {
    // TODO: Implement node schema submission
  };

  const submitEdge = async () => {
    // TODO: Implement edge schema submission
  };

  return {
    label: {
      value: label,
      setValue: setLabel,
      validate: validateLabel,
      error: labelValidationError,
      reset: resetLabel
    },
    formattedLabel: {
      value: formattedLabel,
      setValue: (_: string) => {},
      validate: validateFormattedLabel,
      error: formattedLabelValidationError,
      reset: resetFormattedLabel
    },
    color: {
      value: color,
      setValue: setColor,
      validate: validateColor,
      error: colorValidationError,
      reset: resetColor
    },
    properties: {
      value: properties,
      setValue: setProperties,
      validateAll: validateAllProperties,
      validateOne: validateOneProperty,
      errors: propertyErrors,
      reset: resetProperties
    },
    submitNode,
    submitEdge
  };
};
