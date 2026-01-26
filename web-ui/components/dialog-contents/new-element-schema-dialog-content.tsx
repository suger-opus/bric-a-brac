"use client";

import ColorPickerField from "@/components/forms/color-picker-field";
import PropertyForm from "@/components/forms/property-form";
import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { filterLabel } from "@/lib/utils";
import { FormInput, FormInputs, RequestProperty } from "@/types";
import { defaultNewProperty } from "@/types/defaults";
import { CheckIcon, ChevronRightIcon, PlusIcon } from "lucide-react";
import { useEffect, useEffectEvent, useState } from "react";

type NewElementSchemaDialogContentProps = {
  kind: "node" | "edge";
  isOpen: boolean;
  onClose: () => void;
  onSubmit: () => Promise<void>;
  label: FormInput<string>;
  formattedLabel: FormInput<string>;
  color: FormInput<string>;
  properties: FormInputs<RequestProperty>;
};

const steps = ["1. General", "2. Display", "3. Custom Properties"];

const NewElementSchemaDialogContent = ({
  kind,
  isOpen,
  onClose,
  onSubmit,
  label,
  formattedLabel,
  color,
  properties
}: NewElementSchemaDialogContentProps) => {
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
  };

  const handleSaveProperty = (id: string, newProperty: RequestProperty) => {
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
        onClose();
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
    if (isOpen) {
      resetState();
    }
  }, [isOpen]);

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
    <DialogContent className="flex flex-col justify-between">
      <DialogHeader className="h-fit">
        <DialogTitle>New {kind[0].toUpperCase() + kind.slice(1)} Type</DialogTitle>
        <DialogDescription>Define the schema of a new {kind} type.</DialogDescription>
      </DialogHeader>
      <div className="no-scrollbar h-[calc(100vh-20rem)] px-1 overflow-y-auto">
        <Tabs value={steps[currentStep]}>
          <TabsList className="w-full mb-4">
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
          <TabsContent value={steps[0]}>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="new-element-label">Label</FieldLabel>
                <FieldDescription className="text-xs">
                  The label is the name of the node type.{" "}
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
                    {properties.errors[property.id] && (
                      <FieldError key={index}>{properties.errors[property.id]}</FieldError>
                    )}
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
        </Tabs>
      </div>
      <DialogFooter className="h-fit">
        <div className="flex flex-col w-full items-end space-y-3">
          <div className="flex space-x-2">
            {currentStep === 0
              ? (
                <Button variant="outline" onClick={onClose}>
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
      </DialogFooter>
    </DialogContent>
  );
};

export default NewElementSchemaDialogContent;
