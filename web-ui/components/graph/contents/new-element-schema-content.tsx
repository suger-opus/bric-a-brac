"use client";

import ColorPickerField from "@/components/graph/forms/color-picker-field";
import PropertyForm from "@/components/graph/forms/property-form";
import { Button } from "@/components/ui/button";
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { filterLabel } from "@/lib/utils";
import { CreatePropertySchema, FormInput, FormInputs } from "@/types";
import { defaultNewProperty } from "@/types/defaults";
import { CheckIcon, ChevronRightIcon, PlusIcon } from "lucide-react";
import { useEffect, useEffectEvent, useState } from "react";

type NewElementSchemaContentProps = {
  kind: "node" | "edge";
  onSubmit: () => Promise<void>;
  label: FormInput<string>;
  formattedLabel: FormInput<string>;
  color: FormInput<string>;
  properties: FormInputs<CreatePropertySchema>;
};

const steps = ["1. General", "2. Display", "3. Custom Properties"];

const NewElementSchemaContent = ({
  kind,
  onSubmit,
  label,
  formattedLabel,
  color,
  properties
}: NewElementSchemaContentProps) => {
  const [currentStep, setCurrentStep] = useState(0);
  const [lastSavedPropertyId, setLastSavedPropertyId] = useState<string | null>(null);
  const unresolvedError = label.error !== null
    || formattedLabel.error !== null
    || color.error !== null
    || Object.values(properties.errors).some((error) => error !== null);

  const handleLabelChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    label.setValue(filterLabel(e.target.value));
  };

  const handleColorChange = (newColor: string) => {
    color.setValue(newColor);
  };

  const handleAddProperty = () => {
    properties.setValue([...properties.value, {
      id: crypto.randomUUID(),
      isSaved: false,
      value: defaultNewProperty
    }]);
  };

  const handleDeleteProperty = (id: string) => {
    properties.setValue(properties.value.filter((property) => property.id !== id));
    setLastSavedPropertyId(id);
  };

  const handleSaveProperty = (id: string, newProperty: CreatePropertySchema) => {
    properties.setValue(properties.value.map((property) => {
      if (property.id === id) {
        return { id, isSaved: true, value: newProperty };
      }
      return property;
    }));
    setLastSavedPropertyId(id);
  };

  const handleUnsaveProperty = (id: string) => {
    properties.setValue(properties.value.map((property) => {
      if (property.id === id) {
        return { ...property, isSaved: false };
      }
      return property;
    }));
  };

  const handlePreviousPage = () => {
    setCurrentStep((prev) => Math.max(0, prev - 1));
  };

  const handleNextPage = async () => {
    if (currentStep === 0) {
      if (label.validate() && formattedLabel.validate()) {
        setCurrentStep(1);
      }
    } else if (currentStep === 1) {
      if (color.validate()) {
        setCurrentStep(2);
      }
    } else if (currentStep === 2) {
      if (properties.validateAll() && !unresolvedError) {
        try {
          await onSubmit();
        } catch (error) {
          console.error(error);
        }
      }
    }
  };

  const resetState = useEffectEvent(() => {
    setCurrentStep(0);
    label.reset();
    formattedLabel.reset();
    color.reset();
    properties.reset();
  });

  useEffect(() => {
    resetState();
  }, []);

  const validateLastSavedProperty = useEffectEvent(() => {
    if (lastSavedPropertyId) {
      properties.validateOne(lastSavedPropertyId);
      setLastSavedPropertyId(null);
    }
  });

  useEffect(() => {
    validateLastSavedProperty();
  }, [properties.value, lastSavedPropertyId]);

  return (
    <div className="h-full flex flex-col justify-between">
      <Tabs value={steps[currentStep]} className="overflow-hidden">
        <TabsList className="w-full mb-2">
          <TabsTrigger value={steps[0]} disabled={currentStep !== 0}>
            {steps[0]} {currentStep > 0
              ? <CheckIcon className="ml-auto" />
              : <ChevronRightIcon className="ml-auto" />}
          </TabsTrigger>
          <TabsTrigger value={steps[1]} disabled={currentStep !== 1}>
            {steps[1]} {currentStep > 1
              ? <CheckIcon className="ml-auto" />
              : <ChevronRightIcon className="ml-auto" />}
          </TabsTrigger>
          <TabsTrigger value={steps[2]} disabled={currentStep !== 2}>
            {steps[2]} <ChevronRightIcon className="ml-auto" />
          </TabsTrigger>
        </TabsList>
        <div className="no-scrollbar px-1 overflow-y-auto">
          <TabsContent value={steps[0]}>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="new-element-label">Label</FieldLabel>
                <FieldDescription className="text-xs">
                  The label is the name of the {kind} type.{" "}
                  <b>
                    Only letters and spaces are allowed.
                  </b>
                </FieldDescription>
                <Input
                  id="new-element-label"
                  value={label.value}
                  onChange={handleLabelChange}
                  placeholder="Character"
                />
                <FieldError>{label.error}</FieldError>
              </Field>
              <Field>
                <FieldLabel htmlFor="new-element-formatted-label">Formatted Label</FieldLabel>
                <FieldDescription className="text-xs">
                  The formatted label is generated automatically.{" "}
                  <b>
                    It should be unique among nodes and edges types of this graph.
                  </b>
                </FieldDescription>
                <Input id="new-element-formatted-label" value={formattedLabel.value} readOnly />
                <FieldError>{formattedLabel.error}</FieldError>
              </Field>
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[1]}>
            <FieldGroup>
              <ColorPickerField
                color={color.value}
                setColor={handleColorChange}
                validationError={color.error}
              />
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[2]}>
            <FieldGroup>
              <div className="space-y-2">
                {properties.value.map((property, index) => (
                  <div key={index} className="space-y-1">
                    <PropertyForm
                      isSaved={property.isSaved}
                      property={property.value}
                      saveProperty={(p) => handleSaveProperty(property.id, p)}
                      unSaveProperty={() => handleUnsaveProperty(property.id)}
                      deleteProperty={() => handleDeleteProperty(property.id)}
                    />
                    <FieldError>{properties.errors[property.id]}</FieldError>
                  </div>
                ))}
              </div>
              <Field>
                <Button size="sm" variant="outline" onClick={handleAddProperty}>
                  <PlusIcon />Add Custom Property
                </Button>
              </Field>
            </FieldGroup>
          </TabsContent>
        </div>
      </Tabs>
      <div className="mt-4 flex flex-col w-full items-end space-y-3">
        <div className="flex space-x-2">
          {currentStep === 0
            ? (
              <Button variant="outline">
                Cancel
              </Button>
            )
            : (
              <Button variant="outline" onClick={handlePreviousPage}>
                Back
              </Button>
            )}
          <Button onClick={handleNextPage}>
            {currentStep === steps.length - 1 ? "Submit" : "Next"}
          </Button>
        </div>
        {unresolvedError && (
          <FieldError>Unresolved errors. Please fix them before continuing.</FieldError>
        )}
      </div>
    </div>
  );
};

export default NewElementSchemaContent;
