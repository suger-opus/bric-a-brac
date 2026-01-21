"use client";

import ColorPickerField from "@/components/forms/color-picker-field";
import PropertyField from "@/components/forms/property-field";
import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldSet
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  requestColor,
  requestFormattedLabel,
  requestLabel
} from "@/lib/api/schemas/request-schemas";
import { filterLabel, formatLabel } from "@/lib/utils";
import { RequestProperty } from "@/types";
import { defaultNewProperty } from "@/types/defaults";
import { PlusIcon } from "lucide-react";
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
  const [draftProperties, setDraftProperties] = useState<RequestProperty[]>([]);
  const [properties, setProperties] = useState<RequestProperty[]>([]);

  const handleLabelChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(filterLabel(e.target.value));
  };

  const handleAddDraftProperty = () => {
    setDraftProperties([...draftProperties, defaultNewProperty]);
  };

  const handleRemoveDraftProperty = (index: number) => {
    setDraftProperties(draftProperties.filter((_, i) => i !== index));
  };

  const handleAddProperty = (index: number, property: RequestProperty) => {
    setProperties([...properties, property]);
    handleRemoveDraftProperty(index);
  };

  const handleRemoveProperty = (index: number) => {
    setProperties(properties.filter((_, i) => i !== index));
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
      }
      if (!validFormattedLabel.success) {
        setFormattedLabelValidationError(validFormattedLabel.issues[0].message);
      }
      if (validLabel.success && validFormattedLabel.success) {
        setLabelValidationError(null);
        setFormattedLabelValidationError(null);
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
              {steps[0]}
            </TabsTrigger>
            <TabsTrigger value={steps[1]} disabled={currentStep !== 1}>
              {steps[1]}
            </TabsTrigger>
            <TabsTrigger value={steps[2]} disabled={currentStep !== 2}>
              {steps[2]}
            </TabsTrigger>
          </TabsList>
          <TabsContent value={steps[0]}>
            <FieldSet>
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
            </FieldSet>
          </TabsContent>
          <TabsContent value={steps[1]}>
            <FieldSet>
              <FieldGroup>
                <ColorPickerField
                  color={color}
                  setColor={setColor}
                  validationError={colorValidationError}
                />
              </FieldGroup>
            </FieldSet>
          </TabsContent>
          <TabsContent value={steps[2]}>
            <FieldSet>
              <FieldGroup>
                {draftProperties.map((property, index) => (
                  <PropertyField
                    key={index}
                    index={index}
                    property={property}
                    addProperty={(p) => handleAddProperty(index, p)}
                    removeProperty={() => handleRemoveDraftProperty(index)}
                  />
                ))}
                <Field>
                  <Button size="sm" variant="outline" onClick={handleAddDraftProperty}>
                    <PlusIcon />Add Custom Property
                  </Button>
                </Field>
              </FieldGroup>
            </FieldSet>
          </TabsContent>
        </Tabs>
      </div>
      <DialogFooter className="h-fit">
        {currentStep === 0
          ? (
            <Button className="ml-auto" variant="outline" onClick={onClose}>
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
      </DialogFooter>
    </DialogContent>
  );
};

export default NewNodeDialogContent;
