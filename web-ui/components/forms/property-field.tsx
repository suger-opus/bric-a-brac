import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldSeparator
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { Item, ItemContent, ItemHeader } from "@/components/ui/item";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";
import {
  requestFormattedLabel,
  requestLabel,
  requestPropertyMetadata
} from "@/lib/api/schemas/request-schemas";
import { filterLabel, formatLabel, pluralize } from "@/lib/utils";
import { PropertyType, RequestProperty } from "@/types";
import { CheckIcon, TrashIcon, XIcon } from "lucide-react";
import { useState } from "react";
import * as v from "valibot";

type PropertyFieldProps = {
  index: number;
  property: RequestProperty;
  addProperty: (property: RequestProperty) => void;
  removeProperty: () => void;
};

const PropertyField = ({ index, property, addProperty, removeProperty }: PropertyFieldProps) => {
  const [label, setLabel] = useState(property.label);
  const [labelValidationError, setLabelValidationError] = useState<string | null>(null);
  const formattedLabel = formatLabel(label);
  const [formattedLabelValidationError, setFormattedLabelValidationError] = useState<string | null>(
    null
  );
  const [propertyType, setPropertyType] = useState(property.metadata.property_type);
  const [isPropertyRequired, setIsPropertyRequired] = useState(property.metadata.details.required);
  const [currentOption, setCurrentOption] = useState("");
  const [options, setOptions] = useState(property.metadata.details.options);
  const [metadataValidationError, setMetadataValidationError] = useState<string | null>(null);

  const handleLabelChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(filterLabel(e.target.value));
  };

  const handlePropertyTypeChange = (value: PropertyType) => {
    setPropertyType(value);
    if (value !== PropertyType.SELECT) {
      setOptions(null);
    } else {
      setOptions([]);
    }
  };

  const handleOptionInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCurrentOption(e.target.value);
  };

  const handleOptionInputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && currentOption.trim() !== "") {
      e.preventDefault();
      if (options) {
        setOptions([...options, currentOption.trim()]);
      } else {
        setOptions([currentOption.trim()]);
      }
      setCurrentOption("");
    }
  };

  const handlePropertyOptionRemoved = (optionToRemove: string) => {
    if (options) {
      const filteredOptions = options.filter((option) => option !== optionToRemove);
      setOptions(filteredOptions);
    }
  };

  const handleAddProperty = () => {
    const validLabel = v.safeParse(requestLabel, label);
    const validFormattedLabel = v.safeParse(requestFormattedLabel, formattedLabel);
    const validMetadata = v.safeParse(requestPropertyMetadata, {
      property_type: propertyType,
      details: {
        options: options,
        required: isPropertyRequired
      }
    });
    if (!validLabel.success) {
      setLabelValidationError(validLabel.issues[0].message);
    }
    if (!validFormattedLabel.success) {
      setFormattedLabelValidationError(validFormattedLabel.issues[0].message);
    }
    if (!validMetadata.success) {
      setMetadataValidationError(validMetadata.issues[0].message);
    }
    if (validLabel.success && validFormattedLabel.success && validMetadata.success) {
      setLabelValidationError(null);
      setFormattedLabelValidationError(null);
      setMetadataValidationError(null);
      addProperty({
        label,
        formatted_label: formattedLabel,
        metadata: {
          property_type: propertyType,
          details: {
            options: options,
            required: isPropertyRequired
          }
        }
      });
    }
  };

  const handleRemoveProperty = () => {
    removeProperty();
  };

  return (
    <Item variant="outline">
      <ItemHeader>Property #{index + 1}</ItemHeader>
      <ItemContent>
        <FieldGroup>
          <Field>
            <FieldLabel htmlFor="new-property-label">Label</FieldLabel>
            <FieldDescription className="text-xs">
              The label is the name of the property.{" "}
              <b>
                Only letters and spaces are allowed.
              </b>
            </FieldDescription>
            <Input
              id="new-property-label"
              value={label}
              onChange={handleLabelChange}
              placeholder="Character"
            />
            <FieldError>{labelValidationError}</FieldError>
          </Field>
          <Field>
            <FieldLabel htmlFor="new-property-formatted-label">Formatted Label</FieldLabel>
            <FieldDescription className="text-xs">
              The formatted label is generated automatically.{" "}
              <b>
                It should be unique among properties of this node/edge type.
              </b>
            </FieldDescription>
            <Input id="new-property-formatted-label" value={formattedLabel} readOnly />
            <FieldError>{formattedLabelValidationError}</FieldError>
          </Field>
          <Field>
            <FieldLabel htmlFor="new-property-type">Type</FieldLabel>
            <FieldDescription className="text-xs">
              The type defines the kind of data this property can hold.
            </FieldDescription>
            <Select value={propertyType} onValueChange={handlePropertyTypeChange}>
              <SelectTrigger className="w-full" id="new-property-type">
                <SelectValue placeholder="Select a type" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  {Object.values(PropertyType).map((type) => (
                    <SelectItem key={type} value={type}>
                      {type}
                    </SelectItem>
                  ))}
                </SelectGroup>
              </SelectContent>
            </Select>
          </Field>
          {propertyType === PropertyType.SELECT && (
            <Field>
              <FieldLabel htmlFor="new-property-options">Options</FieldLabel>
              <FieldDescription className="text-xs">
                Define the selectable options for this property. Type and press Enter to add an
                option.
              </FieldDescription>
              <InputGroup>
                <InputGroupInput
                  placeholder="Yellow"
                  value={currentOption}
                  onChange={handleOptionInputChange}
                  onKeyDown={handleOptionInputKeyDown}
                />
                <InputGroupAddon align="block-end" className="flex flex-wrap">
                  {options?.map((option, idx) => (
                    <Badge key={idx} variant="secondary">
                      {option}
                      <Button
                        variant="ghost"
                        size="icon-sm"
                        className="cursor-pointer p-0 h-3 w-3"
                        onClick={() => handlePropertyOptionRemoved(option)}
                      >
                        <XIcon />
                      </Button>
                    </Badge>
                  ))}
                </InputGroupAddon>
                <InputGroupAddon align="block-end">
                  <FieldDescription className="text-xs">
                    {options?.length} {pluralize(options?.length || 0, "option", "options")}
                  </FieldDescription>
                </InputGroupAddon>
              </InputGroup>
            </Field>
          )}
          <FieldError>{metadataValidationError}</FieldError>
          <Field orientation="horizontal">
            <Checkbox
              id="new-property-required"
              checked={isPropertyRequired}
              onCheckedChange={(e) => setIsPropertyRequired(e as boolean)}
            />
            <FieldLabel
              htmlFor="new-property-required"
              className="font-normal"
            >
              I want this property to be required
            </FieldLabel>
          </Field>
          <FieldSeparator />
          <Field orientation="horizontal">
            <Button size="sm" variant="secondary" onClick={handleAddProperty}>
              <CheckIcon />
              Add Property
            </Button>
            <Button size="sm" variant="ghost" onClick={handleRemoveProperty}>
              <TrashIcon /> Remove
            </Button>
          </Field>
        </FieldGroup>
      </ItemContent>
    </Item>
  );
};

export default PropertyField;
