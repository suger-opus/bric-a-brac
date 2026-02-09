import { propertyValue } from "@/lib/api/schemas/response-schemas";
import { FormInput, FormInputs, Property, PropertyType, PropertyValue } from "@/types";
import { useState } from "react";
import * as v from "valibot";

type UseElementDataFormReturn = {
  nodeSchemaId: FormInput<string | null>;
  edgeSchemaId: FormInput<string | null>;
  properties: FormInputs<{ property: Property; value: PropertyValue; }>;
  fromNodeId: FormInput<string | null>;
  toNodeId: FormInput<string | null>;
  submitNode: () => Promise<void>;
  submitEdge: () => Promise<void>;
};

export const useElementDataForm = (): UseElementDataFormReturn => {
  const [elementSchemaId, setElementSchemaId] = useState<string | null>(null);
  const [elementIdError, setElementSchemaIdError] = useState<string | null>(null);
  const [properties, setProperties] = useState<
    { id: string; isSaved: boolean; value: { property: Property; value: PropertyValue; }; }[]
  >([]);
  const [propertyErrors, setPropertyErrors] = useState<Record<string, string | null>>({});
  const [fromNodeId, setFromNodeId] = useState<string | null>(null);
  const [fromNodeIdError, setFromNodeIdError] = useState<string | null>(null);
  const [toNodeId, setToNodeId] = useState<string | null>(null);
  const [toNodeIdError, setToNodeIdError] = useState<string | null>(null);

  const validateElementSchemaId = () => {
    if (elementSchemaId === null) {
      setElementSchemaIdError("Please select a node type.");
      return false;
    }
    setElementSchemaIdError(null);
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
    const validProperty = v.safeParse(propertyValue, property.value.value);
    if (!validProperty.success) {
      setPropertyErrors((prev) => ({
        ...prev,
        [property.id]: validProperty.issues[0].message
      }));
      return false;
    }
    if (
      property.value.property.property_type === PropertyType.SELECT
      && !property.value.property.metadata.options!.includes(property.value.value as string)
    ) {
      setPropertyErrors((prev) => ({
        ...prev,
        [property.id]: "Invalid option selected."
      }));
      return false;
    }
    setPropertyErrors((prev) => ({ ...prev, [property.id]: null }));
    return true;
  };

  const validateAllProperties = () => {
    return properties.every((property) => validateOneProperty(property.id));
  };

  const validateFromNodeId = () => {
    if (fromNodeId === null) {
      setFromNodeIdError("Please select a starting node.");
      return false;
    }
    setFromNodeIdError(null);
    return true;
  };

  const validateToNodeId = () => {
    if (toNodeId === null) {
      setToNodeIdError("Please select an end node.");
      return false;
    }
    setToNodeIdError(null);
    return true;
  };

  const resetElementSchemaId = () => {
    setElementSchemaId(null);
    setElementSchemaIdError(null);
  };

  const resetProperties = () => {
    setProperties([]);
    setPropertyErrors({});
  };

  const resetFromNodeId = () => {
    setFromNodeId(null);
  };

  const resetToNodeId = () => {
    setToNodeId(null);
  };

  const submitNode = async () => {
    // TODO: Implement node data submission
  };

  const submitEdge = async () => {
    // TODO: Implement edge data submission
  };

  return {
    nodeSchemaId: {
      value: elementSchemaId,
      setValue: setElementSchemaId,
      validate: validateElementSchemaId,
      error: elementIdError,
      reset: resetElementSchemaId
    },
    edgeSchemaId: {
      value: elementSchemaId,
      setValue: setElementSchemaId,
      validate: validateElementSchemaId,
      error: elementIdError,
      reset: resetElementSchemaId
    },
    properties: {
      value: properties,
      setValue: setProperties,
      validateAll: validateAllProperties,
      validateOne: validateOneProperty,
      errors: propertyErrors,
      reset: resetProperties
    },
    fromNodeId: {
      value: fromNodeId,
      setValue: setFromNodeId,
      validate: validateFromNodeId,
      error: fromNodeIdError,
      reset: resetFromNodeId
    },
    toNodeId: {
      value: toNodeId,
      setValue: setToNodeId,
      validate: validateToNodeId,
      error: toNodeIdError,
      reset: resetToNodeId
    },
    submitNode,
    submitEdge
  };
};
