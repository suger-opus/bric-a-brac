"use client";

import PropertyTypeBadge from "@/components/graph/badges/property-type-badge";
import { Button } from "@/components/ui/button";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { Item, ItemContent, ItemTitle } from "@/components/ui/item";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { pluralize } from "@/lib/utils";
import { Property } from "@/types";
import { ChevronDown, EyeClosedIcon, EyeIcon } from "lucide-react";

type ElementSchemaItemProps = {
  kind: "node" | "edge";
  label: string;
  color: string;
  properties: Property[];
  displayedProperty: string | undefined;
  updateDisplayedProperty: (property_formatted_label: string | undefined) => void;
};

const ElementSchemaItem = ({
  kind,
  label,
  color,
  properties,
  displayedProperty,
  updateDisplayedProperty
}: ElementSchemaItemProps) => {
  return (
    <Item variant="outline" className="py-2 px-3">
      <ItemContent>
        <div className="flex justify-between items-center">
          <div className="flex items-center space-x-1">
            <div
              className={kind === "node" ? "w-4 h-4 rounded-full" : "w-4 h-2 rounded-xs"}
              style={{ backgroundColor: color }}
            />
            <ItemTitle>{label}</ItemTitle>
          </div>
        </div>
        <Collapsible defaultOpen={false} className="group/collapsible-properties">
          <CollapsibleTrigger className="flex items-center text-gray-800 text-xs cursor-pointer w-fit">
            <ChevronDown
              size={12}
              className="transition-transform group-data-[state=open]/collapsible-properties:rotate-180"
            />
            {properties.length} {pluralize(properties.length, "property", "properties")}
          </CollapsibleTrigger>
          <CollapsibleContent>
            <Table className="mt-1 text-xs">
              <TableBody>
                {properties.map((property, index) => (
                  <TableRow key={index}>
                    <TableCell className="text-left w-48">
                      <Tooltip>
                        <TooltipTrigger className="font-medium text-ellipsis overflow-hidden whitespace-nowrap max-w-48">
                          {property.label}
                        </TooltipTrigger>
                        <TooltipContent>
                          {property.label}
                        </TooltipContent>
                      </Tooltip>
                    </TableCell>
                    <TableCell className="grow">
                      <PropertyTypeBadge property={property} />
                    </TableCell>
                    <TableCell className="text-right w-12">
                      {displayedProperty === property.formatted_label
                        ? (
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Button
                                variant="ghost"
                                size="icon-sm"
                                onClick={() => updateDisplayedProperty(undefined)}
                              >
                                <EyeIcon />
                              </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                              Hide this property
                            </TooltipContent>
                          </Tooltip>
                        )
                        : (
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Button
                                variant="ghost"
                                size="icon-sm"
                                onClick={() => updateDisplayedProperty(property.formatted_label)}
                              >
                                <EyeClosedIcon />
                              </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                              Show this property
                            </TooltipContent>
                          </Tooltip>
                        )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CollapsibleContent>
        </Collapsible>
      </ItemContent>
    </Item>
  );
};

export default ElementSchemaItem;
