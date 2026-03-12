"use client";

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { Item, ItemContent, ItemTitle } from "@/components/ui/item";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { pluralize } from "@/lib/utils";
import { PropertiesData, PropertySchema } from "@/types";
import { ChevronDown } from "lucide-react";

type DraftElementDataItemProps = {
  kind: "node" | "edge";
  label: string;
  color: string;
  propertiesSchemas: PropertySchema[];
  propertiesData: PropertiesData;
};

const DraftElementDataItem = ({
  kind,
  label,
  color,
  propertiesSchemas,
  propertiesData
}: DraftElementDataItemProps) => {
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
            {propertiesSchemas.length} {pluralize(propertiesSchemas.length, "property", "properties")}
          </CollapsibleTrigger>
          <CollapsibleContent>
            <Table className="mt-1 text-xs">
              <TableBody>
                {Object.entries(propertiesData).map(([key, value]) => (
                  <TableRow key={key}>
                    <TableCell className="font-medium">
                      {propertiesSchemas.find((p) => p.key === key)?.label || key}
                    </TableCell>
                    <TableCell>
                      {String(value)}
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

export default DraftElementDataItem;
