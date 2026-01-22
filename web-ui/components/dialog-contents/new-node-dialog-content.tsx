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
import {
  requestColor,
  requestFormattedLabel,
  requestLabel,
  requestProperty
} from "@/lib/api/schemas/request-schemas";
import { filterLabel, formatLabel } from "@/lib/utils";
import { RequestProperty } from "@/types";
import { defaultNewProperty } from "@/types/defaults";
import { CheckIcon, ChevronRightIcon, PlusIcon } from "lucide-react";
import { useEffect, useEffectEvent, useState } from "react";
import * as v from "valibot";

type NewNodeDialogContentProps = {
  isOpen: boolean;
  onClose: () => void;
};

const steps = ["1. General", "2. Display", "3. Custom Properties"];

const NewNodeDialogContent = ({ isOpen, onClose }: NewNodeDialogContentProps) => {
  const [currentStep, setCurrentStep] = useState(0);
  const [label, setLabel] = useState("");
  const [labelValidationError, setLabelValidationError] = useState<string | null>(null);
  const formattedLabel = formatLabel(label);
  const [formattedLabelValidationError, setFormattedLabelValidationError] = useState<string | null>(
    null
  );
  const [color, setColor] = useState("#3b82f6");
  const [colorValidationError, setColorValidationError] = useState<string | null>(null);
  const [properties, setProperties] = useState<
    { id: string; isSaved: boolean; property: RequestProperty; }[]
  >([]);
  const [propertyErrors, setPropertyErrors] = useState<Record<string, string | null>>({});
  const unresolvedError = labelValidationError !== null
    || formattedLabelValidationError !== null
    || colorValidationError !== null
    || Object.values(propertyErrors).some((error) => error !== null);

  const handleLabelChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(filterLabel(e.target.value));
  };

  const handleAddProperty = () => {
    setProperties([...properties, {
      id: crypto.randomUUID(),
      isSaved: false,
      property: defaultNewProperty
    }]);
  };

  const handleDeleteProperty = (id: string) => {
    setProperties(properties.filter((property) => property.id !== id));
  };

  const handleSaveProperty = (id: string, newProperty: RequestProperty) => {
    setProperties(properties.map((property) => {
      if (property.id === id) {
        return { id, isSaved: true, property: newProperty };
      }
      return property;
    }));
    setPropertyErrors((prev) => ({ ...prev, [id]: null }));
  };

  const handleUnsaveProperty = (id: string) => {
    setProperties(properties.map((property) => {
      if (property.id === id) {
        return { ...property, isSaved: false };
      }
      return property;
    }));
  };

  const handlePreviousPage = () => {
    setCurrentStep((prev) => Math.max(0, prev - 1));
  };

  const handleNextPage = () => {
    if (currentStep === 0) {
      const validLabel = v.safeParse(requestLabel, label);
      const validFormattedLabel = v.safeParse(requestFormattedLabel, formattedLabel);
      if (!validLabel.success) {
        setLabelValidationError(validLabel.issues[0].message);
      } else {
        setLabelValidationError(null);
      }
      if (!validFormattedLabel.success) {
        setFormattedLabelValidationError(validFormattedLabel.issues[0].message);
      } else {
        setFormattedLabelValidationError(null);
      }
      if (validLabel.success && validFormattedLabel.success) {
        setCurrentStep(1);
      }
    } else if (currentStep === 1) {
      const validColor = v.safeParse(requestColor, color);
      if (!validColor.success) {
        setColorValidationError(validColor.issues[0].message);
      }
      if (validColor.success) {
        setColorValidationError(null);
        setCurrentStep(2);
      }
    } else if (currentStep === 2) {
      properties.forEach((property) => {
        if (!property.isSaved) {
          setPropertyErrors((prev) => ({
            ...prev,
            [property.id]: "Save this property to continue."
          }));
        } else {
          const validProperty = v.safeParse(requestProperty, property.property);
          if (!validProperty.success) {
            setPropertyErrors((prev) => ({
              ...prev,
              [property.id]: validProperty.issues[0].message
            }));
          } else {
            setPropertyErrors((prev) => ({ ...prev, [property.id]: null }));
          }
        }
      });
    }
  };

  // todo: explain why useEffectEvent is needed here
  const resetState = useEffectEvent(() => {
    setCurrentStep(0);
    setLabel("");
    setLabelValidationError(null);
    setFormattedLabelValidationError(null);
    setColor("#3b82f6");
    setColorValidationError(null);
    setProperties([]);
    setPropertyErrors({});
  });

  useEffect(() => {
    if (isOpen) {
      resetState();
    }
  }, [isOpen]);

  return (
    <DialogContent className="flex flex-col justify-between">
      <DialogHeader className="h-fit">
        <DialogTitle>New Node Type</DialogTitle>
        <DialogDescription>Define the schema of a new node type.</DialogDescription>
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
                <FieldLabel htmlFor="new-node-label">Label</FieldLabel>
                <FieldDescription className="text-xs">
                  The label is the name of the node type.{" "}
                  <b>
                    Only letters and spaces are allowed.
                  </b>
                </FieldDescription>
                <Input
                  id="new-node-label"
                  value={label}
                  onChange={handleLabelChange}
                  placeholder="Character"
                />
                <FieldError>{labelValidationError}</FieldError>
              </Field>
              <Field>
                <FieldLabel htmlFor="new-node-formatted-label">Formatted Label</FieldLabel>
                <FieldDescription className="text-xs">
                  The formatted label is generated automatically.{" "}
                  <b>
                    It should be unique among nodes and edges types of this graph.
                  </b>
                </FieldDescription>
                <Input id="new-node-formatted-label" value={formattedLabel} readOnly />
                <FieldError>{formattedLabelValidationError}</FieldError>
              </Field>
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[1]}>
            <FieldGroup>
              <ColorPickerField
                color={color}
                setColor={setColor}
                validationError={colorValidationError}
              />
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[2]}>
            <FieldGroup>
              <div className="space-y-4">
                {properties.map((property, index) => (
                  <div key={index} className="space-y-1">
                    <PropertyForm
                      isSaved={property.isSaved}
                      property={property.property}
                      saveProperty={(p) => handleSaveProperty(property.id, p)}
                      unSaveProperty={() => handleUnsaveProperty(property.id)}
                      deleteProperty={() => handleDeleteProperty(property.id)}
                    />
                    {propertyErrors[property.id] && (
                      <FieldError key={index}>{propertyErrors[property.id]}</FieldError>
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
        <div className="flex flex-col w-full items-end space-y-2">
          <div className="flex space-x-2">
            {currentStep === 0
              ? (
                <Button variant="outline" onClick={onClose}>
                  Cancel
                </Button>
              )
              : (
                <Button
                  variant="outline"
                  onClick={handlePreviousPage}
                >
                  Back
                </Button>
              )}
            <Button
              onClick={handleNextPage}
            >
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

export default NewNodeDialogContent;
