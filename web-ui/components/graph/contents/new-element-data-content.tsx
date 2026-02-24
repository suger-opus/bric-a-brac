"use client";

import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Field, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
  InputGroupText,
  InputGroupTextarea
} from "@/components/ui/input-group";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemMedia,
  ItemTitle
} from "@/components/ui/item";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { pluralize } from "@/lib/utils";
import {
  FormInput,
  FormInputs,
  NodeSchema,
  ProcessedNodeData,
  PropertySchema,
  PropertyType,
  PropertyValue
} from "@/types";
import { CheckIcon, ChevronRightIcon } from "lucide-react";
import { Fragment, useEffect, useEffectEvent, useState } from "react";

type NewElementDataContentProps = {
  kind: "node" | "edge";
  onSubmit: () => Promise<void>;
  schemaOptions: { id: string; label: string; color: string; properties: PropertySchema[]; }[];
  schemaId: FormInput<string | null>;
  properties: FormInputs<{ property: PropertySchema; value: PropertyValue; }>;
  fromNodeId?: FormInput<string | null>;
  toNodeId?: FormInput<string | null>;
  nodeSchemas?: NodeSchema[];
  nodeOptions?: ProcessedNodeData[];
};

const NodeOptionItem = ({
  nodeProcessedData,
  nodeSchema,
  isSelected,
  onClick
}: {
  nodeProcessedData: ProcessedNodeData;
  nodeSchema: NodeSchema;
  isSelected: boolean;
  onClick: (id: string) => void;
}) => {
  return (
    <Item
      variant="outline"
      className="cursor-pointer"
      onClick={() => onClick(nodeProcessedData.id)}
    >
      <ItemContent>
        <ItemTitle className="w-full">
          <div
            className="h-4 w-4 rounded-full"
            style={{ backgroundColor: nodeSchema.color }}
          />
          {nodeSchema.label}
          <Checkbox checked={isSelected} className="ml-auto" />
        </ItemTitle>
        <Table className="mt-1 text-xs">
          <TableBody>
            {Object.entries(nodeProcessedData.properties).map(([key, property]) => (
              <TableRow key={key}>
                <TableCell className="text-left w-1/2">
                  <Tooltip>
                    <TooltipTrigger className="font-medium text-ellipsis overflow-hidden whitespace-nowrap max-w-48">
                      {nodeSchema.properties.find((p) =>
                        p.key === key
                      )!.label}
                    </TooltipTrigger>
                    <TooltipContent>
                      {nodeSchema.properties.find((p) => p.key === key)!.label}
                    </TooltipContent>
                  </Tooltip>
                </TableCell>
                <TableCell className="text-left w-1/2">
                  <Tooltip>
                    <TooltipTrigger className="font-medium text-ellipsis overflow-hidden whitespace-nowrap max-w-48">
                      {typeof property === "boolean" ? property.toString() : property}
                    </TooltipTrigger>
                    <TooltipContent>
                      {typeof property === "boolean" ? property.toString() : property}
                    </TooltipContent>
                  </Tooltip>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </ItemContent>
    </Item>
  );
};

const NewElementDataContent = ({
  kind,
  onSubmit,
  schemaOptions,
  schemaId,
  properties,
  fromNodeId,
  toNodeId,
  nodeSchemas,
  nodeOptions
}: NewElementDataContentProps) => {
  const steps = kind === "node"
    ? [
      "1. Node Type",
      "2. Properties"
    ]
    : [
      "1. Edge Type",
      "2. Properties",
      "3. Start Node",
      "4. End Node"
    ];
  const [currentStep, setCurrentStep] = useState(0);
  const unresolvedError = schemaId.error !== null
    || Object.values(properties.errors).some((error) => error !== null)
    || (kind === "edge" && (fromNodeId!.error !== null || toNodeId!.error !== null));

  const handlePropertyValueChange = (id: string, newValue: PropertyValue) => {
    properties.setValue(properties.value.map((property) => {
      if (property.id === id) {
        return {
          id,
          isSaved: property.isSaved,
          value: {
            property: property.value.property,
            value: newValue
          }
        };
      }
      return property;
    }));
  };

  const handlePropertyStringChange = (id: string, newValue: string) => {
    if (newValue.length <= 250) {
      handlePropertyValueChange(id, newValue);
    }
  };

  const handleElementSelect = (new_id: string) => {
    if (schemaId.value === new_id) {
      schemaId.setValue(null);
    } else {
      schemaId.setValue(new_id);
    }
  };

  const handlePreviousPage = () => {
    setCurrentStep((prev) => Math.max(0, prev - 1));
  };

  const callOnSumbit = async () => {
    try {
      await onSubmit();
    } catch (error) {
      console.error(error);
    }
  };

  const handleNextPage = async () => {
    if (currentStep === 0) {
      if (schemaId.validate()) {
        setCurrentStep(1);
      }
    } else if (currentStep === 1) {
      if (properties.validateAll()) {
        if (kind === "edge") {
          setCurrentStep(2);
        } else {
          await callOnSumbit();
        }
      }
    } else if (currentStep === 2) {
      if (fromNodeId!.validate()) {
        setCurrentStep(3);
      }
    } else if (currentStep === 3) {
      if (toNodeId!.validate()) {
        await callOnSumbit();
      }
    } else {
      await callOnSumbit();
    }
  };

  const resetState = useEffectEvent(() => {
    setCurrentStep(0);
    schemaId.reset();
    properties.reset();
  });

  useEffect(() => {
    resetState();
  }, []);

  useEffect(() => {
    if (schemaId.value !== null) {
      schemaId.validate();
    }
    if (fromNodeId?.value !== null) {
      fromNodeId?.validate();
    }
    if (toNodeId?.value !== null) {
      toNodeId?.validate();
    }
  }, [schemaId.value, fromNodeId?.value, toNodeId?.value]);

  return (
    <div className="h-full flex flex-col justify-between">
      <Tabs value={steps[currentStep]} className="overflow-hidden h-full">
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
          {kind === "edge" && (
            <TabsTrigger value={steps[2]} disabled={currentStep !== 2}>
              {steps[2]} <ChevronRightIcon className="ml-auto" />
            </TabsTrigger>
          )}
          {kind === "edge" && (
            <TabsTrigger value={steps[3]} disabled={currentStep !== 3}>
              {steps[3]} <ChevronRightIcon className="ml-auto" />
            </TabsTrigger>
          )}
        </TabsList>
        <div className="no-scrollbar px-1 overflow-y-auto h-full">
          <TabsContent value={steps[0]}>
            <FieldGroup>
              <Field>
                <FieldError>{schemaId.error}</FieldError>
                <div className="grid grid-cols-2 gap-2">
                  {schemaOptions.map((option) => (
                    <Item
                      key={option.id}
                      variant="outline"
                      className="cursor-pointer"
                      onClick={() => handleElementSelect(option.id)}
                    >
                      <ItemMedia>
                        <div
                          className="h-4 w-4 rounded-full"
                          style={{ backgroundColor: option.color }}
                        />
                      </ItemMedia>
                      <ItemContent>
                        <ItemTitle>{option.label}</ItemTitle>
                        <ItemDescription className="text-xs">
                          {option.properties.length}{" "}
                          {pluralize(option.properties.length, "property", "properties")}
                        </ItemDescription>
                      </ItemContent>
                      <ItemActions>
                        <Checkbox checked={schemaId.value === option.id} />
                      </ItemActions>
                    </Item>
                  ))}
                </div>
              </Field>
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[1]}>
            <FieldGroup className="mt-1">
              {properties.value.map((property) => (
                <Fragment key={property.id}>
                  {property.value.property.property_type === PropertyType.STRING && (
                    <Field>
                      <FieldLabel htmlFor={property.value.property.key}>
                        {property.value.property.label}
                      </FieldLabel>
                      <InputGroup>
                        <InputGroupTextarea
                          id={property.value.property.key}
                          placeholder="Fill this property"
                          value={property.value.value as string}
                          onChange={(e) => handlePropertyStringChange(property.id, e.target.value)}
                        />
                        <InputGroupAddon align="block-end">
                          <InputGroupText className="text-muted-foreground text-xs">
                            {(property.value.value as string).length}/250
                          </InputGroupText>
                        </InputGroupAddon>
                      </InputGroup>
                      <FieldError>{properties.errors[property.id]}</FieldError>
                    </Field>
                  )}
                  {property.value.property.property_type === PropertyType.NUMBER && (
                    <Field>
                      <FieldLabel htmlFor={property.value.property.key}>
                        {property.value.property.label}
                      </FieldLabel>
                      <InputGroup>
                        <InputGroupInput
                          id={property.value.property.key}
                          type="number"
                          placeholder="0"
                          value={property.value.value.toString()}
                          onChange={(e) => handlePropertyValueChange(property.id, +e.target.value)}
                        />
                      </InputGroup>
                      <FieldError>{properties.errors[property.id]}</FieldError>
                    </Field>
                  )}
                  {property.value.property.property_type === PropertyType.BOOLEAN && (
                    <Field orientation="horizontal">
                      <Checkbox
                        id={property.value.property.key}
                        checked={property.value.value as boolean}
                        onCheckedChange={(e) =>
                          handlePropertyValueChange(property.id, e as boolean)}
                      />
                      <FieldLabel htmlFor={property.value.property.key}>
                        {property.value.property.label}
                      </FieldLabel>
                    </Field>
                  )}
                  {property.value.property.property_type === PropertyType.SELECT && (
                    <Field>
                      <FieldLabel htmlFor={property.value.property.key}>
                        {property.value.property.label}
                      </FieldLabel>
                      <Select
                        value={property.value.value as string}
                        onValueChange={(e) => handlePropertyValueChange(property.id, e)}
                      >
                        <SelectTrigger id={property.value.property.key}>
                          <SelectValue placeholder="Select a value" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            {property.value.property.metadata.options!.map((option) => (
                              <SelectItem key={option} value={option}>
                                {option}
                              </SelectItem>
                            ))}
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                      <FieldError>{properties.errors[property.id]}</FieldError>
                    </Field>
                  )}
                </Fragment>
              ))}
            </FieldGroup>
          </TabsContent>
          {kind === "edge" && (
            <TabsContent value={steps[2]}>
              <FieldGroup>
                <Field>
                  <FieldError>{fromNodeId!.error}</FieldError>
                  <div className="grid grid-cols-2 gap-2">
                    {nodeOptions!.map((option) => (
                      <NodeOptionItem
                        key={option.id}
                        nodeProcessedData={option}
                        nodeSchema={nodeSchemas!.find((s) => s.key === option.key)!}
                        isSelected={fromNodeId!.value === option.id}
                        onClick={() =>
                          fromNodeId!.setValue(fromNodeId!.value === option.id ? null : option.id)}
                      />
                    ))}
                  </div>
                </Field>
              </FieldGroup>
            </TabsContent>
          )}
          {kind === "edge" && (
            <TabsContent value={steps[3]}>
              <FieldGroup>
                <Field>
                  <FieldError>{toNodeId!.error}</FieldError>
                  <div className="grid grid-cols-2 gap-2">
                    {nodeOptions!.filter((o) => o.id !== fromNodeId!.value).map((option) => (
                      <NodeOptionItem
                        key={option.id}
                        nodeProcessedData={option}
                        nodeSchema={nodeSchemas!.find((s) => s.key === option.key)!}
                        isSelected={toNodeId!.value === option.id}
                        onClick={() =>
                          toNodeId!.setValue(toNodeId!.value === option.id ? null : option.id)}
                      />
                    ))}
                  </div>
                </Field>
              </FieldGroup>
            </TabsContent>
          )}
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

export default NewElementDataContent;
