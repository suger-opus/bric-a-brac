import PropertyTypeBadge from "@/components/badges/property-type-badge";
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
import { Item, ItemActions, ItemContent, ItemDescription, ItemTitle } from "@/components/ui/item";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import {
  requestFormattedLabel,
  requestLabel,
  requestPropertyMetadata
} from "@/lib/api/schemas/request-schemas";
import { filterLabel, formatLabel, pluralize } from "@/lib/utils";
import { PropertyType, RequestProperty } from "@/types";
import { PenIcon, Trash2Icon, XIcon } from "lucide-react";
import { useState } from "react";
import * as v from "valibot";

type PropertyFieldGroupProps = {
  baseProperty: RequestProperty;
  saveProperty: (property: RequestProperty) => void;
  deleteProperty: () => void;
};

const PropertyFieldGroup = (
  { baseProperty, saveProperty, deleteProperty }: PropertyFieldGroupProps
) => {
  const [label, setLabel] = useState(baseProperty.label);
  const [labelValidationError, setLabelValidationError] = useState<string | null>(null);
  const formattedLabel = formatLabel(label);
  const [formattedLabelValidationError, setFormattedLabelValidationError] = useState<string | null>(
    null
  );
  const [propertyType, setPropertyType] = useState(baseProperty.metadata.property_type);
  const [isPropertyRequired, setIsPropertyRequired] = useState(
    baseProperty.metadata.details.required
  );
  const [currentOption, setCurrentOption] = useState("");
  const [options, setOptions] = useState(baseProperty.metadata.details.options);
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

  const handlePropertyOptionDelete = (optionToDelete: string) => {
    if (options) {
      const filteredOptions = options.filter((option) => option !== optionToDelete);
      setOptions(filteredOptions);
    }
  };

  const handleSaveProperty = () => {
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
    } else {
      setLabelValidationError(null);
    }
    if (!validFormattedLabel.success) {
      setFormattedLabelValidationError(validFormattedLabel.issues[0].message);
    } else {
      setFormattedLabelValidationError(null);
    }
    if (!validMetadata.success) {
      setMetadataValidationError(validMetadata.issues[0].message);
    } else {
      setMetadataValidationError(null);
    }
    if (validLabel.success && validFormattedLabel.success && validMetadata.success) {
      saveProperty({
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

  const handleDeleteProperty = () => {
    deleteProperty();
  };

  return (
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
                      onClick={() => handlePropertyOptionDelete(option)}
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
            <FieldError>{metadataValidationError}</FieldError>
          </Field>
        )}
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
          <Button size="sm" variant="secondary" onClick={handleSaveProperty}>
            Save Property
          </Button>
          <Button size="sm" variant="ghost" onClick={handleDeleteProperty}>
            Delete
          </Button>
        </Field>
      </FieldGroup>
    </ItemContent>
  );
};

type PropertyItemProps = {
  property: RequestProperty;
  updateProperty: () => void;
  deleteProperty: () => void;
};

const PropertyItem = ({ property, updateProperty, deleteProperty }: PropertyItemProps) => {
  const [hoveredSection, setHoveredSection] = useState<"edit" | "delete" | null>(null);

  return (
    <>
      <ItemContent>
        <ItemTitle>{property.label}</ItemTitle>
        <ItemDescription className="space-x-1">
          <PropertyTypeBadge property={property} />
          <Badge variant="secondary" className="font-bold text-[9px]">
            {property.metadata.details.required ? "REQUIRED" : "OPTIONAL"}
          </Badge>
        </ItemDescription>
      </ItemContent>
      <ItemActions className="h-12 w-24 rounded-sm gap-0 overflow-hidden opacity-0 group-hover/item:opacity-100 transition-opacity duration-200">
        <Tooltip>
          <TooltipTrigger asChild>
            <div
              className="h-full flex items-center justify-center bg-gray-100 hover:bg-gray-200 cursor-pointer transition-all duration-300"
              style={{
                width: hoveredSection === "edit"
                  ? "100%"
                  : hoveredSection === "delete"
                  ? "0%"
                  : "50%"
              }}
              onMouseEnter={() => setHoveredSection("edit")}
              onMouseLeave={() => setHoveredSection(null)}
              onClick={updateProperty}
            >
              <PenIcon className="h-4 w-4 text-gray-700" />
            </div>
          </TooltipTrigger>
          <TooltipContent>
            Edit this property
          </TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger asChild>
            <div
              className="h-full flex items-center justify-center bg-red-100 hover:bg-red-200 cursor-pointer transition-all duration-300"
              style={{
                width: hoveredSection === "delete"
                  ? "100%"
                  : hoveredSection === "edit"
                  ? "0%"
                  : "50%"
              }}
              onMouseEnter={() => setHoveredSection("delete")}
              onMouseLeave={() => setHoveredSection(null)}
              onClick={deleteProperty}
            >
              <Trash2Icon className="h-4 w-4 text-red-700" />
            </div>
          </TooltipTrigger>
          <TooltipContent>
            Delete this property
          </TooltipContent>
        </Tooltip>
      </ItemActions>
    </>
  );
};

type PropertyFormProps = {
  property: RequestProperty;
  isSaved: boolean;
  saveProperty: (property: RequestProperty) => void;
  unSaveProperty: () => void;
  deleteProperty: () => void;
};

const PropertyForm = (
  { property, isSaved, saveProperty, unSaveProperty, deleteProperty }: PropertyFormProps
) => {
  return (
    <Item variant="outline" className={`group/item ${isSaved ? "p-2" : "p-3"}`}>
      {isSaved
        ? (
          <PropertyItem
            property={property}
            updateProperty={unSaveProperty}
            deleteProperty={deleteProperty}
          />
        )
        : (
          <PropertyFieldGroup
            baseProperty={property}
            saveProperty={saveProperty}
            deleteProperty={deleteProperty}
          />
        )}
    </Item>
  );
};

export default PropertyForm;
