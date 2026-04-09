"use client";

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { Item, ItemContent, ItemTitle } from "@/components/ui/item";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Toggle } from "@/components/ui/toggle";
import { useGraph } from "@/contexts/graph-context";
import { ChevronDown, Eye, EyeOff } from "lucide-react";

type ElementSchemaItemProps = {
  kind: "node" | "edge";
  schemaKey: string;
  label: string;
  color: string;
  description: string;
};

const ElementSchemaItem = (
  { kind, schemaKey, label, color, description }: ElementSchemaItemProps
) => {
  const { availableProperties, displayProperty, setDisplayProperty } = useGraph();
  const properties = availableProperties[schemaKey] ?? [];
  const selected = displayProperty[schemaKey] ?? null;

  const toggleProperty = (prop: string) => {
    setDisplayProperty(schemaKey, selected === prop ? null : prop);
  };

  return (
    <Item variant="outline" className="py-2 px-3">
      <ItemContent>
        <div className="flex items-center space-x-1">
          <div
            className={kind === "node" ? "w-4 h-4 rounded-full" : "w-4 h-2 rounded-xs"}
            style={{ backgroundColor: color }}
          />
          <ItemTitle>{label}</ItemTitle>
        </div>
        {description && <p className="mt-1 text-xs text-muted-foreground">{description}</p>}
        {properties.length > 0 && (
          <Collapsible defaultOpen={false} className="group/collapsible-props">
            <CollapsibleTrigger className="flex items-center text-muted-foreground text-xs cursor-pointer w-fit">
              <ChevronDown
                size={12}
                className="transition-transform group-data-[state=open]/collapsible-props:rotate-180"
              />
              Properties
            </CollapsibleTrigger>
            <CollapsibleContent>
              <Table className="mt-1">
                <TableBody>
                  {properties.map((prop) => (
                    <TableRow key={prop} className="h-7">
                      <TableCell className="py-0.5 text-xs">{prop}</TableCell>
                      <TableCell className="py-0.5 w-8 text-right">
                        <Toggle
                          pressed={selected === prop}
                          onPressedChange={() => toggleProperty(prop)}
                          size="sm"
                          className="h-6 w-6 p-0"
                          aria-label={selected === prop ? "Hide from graph" : "Show on graph"}
                        >
                          {selected === prop
                            ? <Eye size={14} />
                            : <EyeOff size={14} className="opacity-40" />}
                        </Toggle>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </CollapsibleContent>
          </Collapsible>
        )}
      </ItemContent>
    </Item>
  );
};

export default ElementSchemaItem;
