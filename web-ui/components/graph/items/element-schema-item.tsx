"use client";

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { Item, ItemContent, ItemTitle } from "@/components/ui/item";
import { ChevronDown } from "lucide-react";

type ElementSchemaItemProps = {
  kind: "node" | "edge";
  label: string;
  color: string;
  description: string;
};

const ElementSchemaItem = ({ kind, label, color, description }: ElementSchemaItemProps) => {
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
        {description && (
          <Collapsible defaultOpen={false} className="group/collapsible-desc">
            <CollapsibleTrigger className="flex items-center text-muted-foreground text-xs cursor-pointer w-fit">
              <ChevronDown
                size={12}
                className="transition-transform group-data-[state=open]/collapsible-desc:rotate-180"
              />
              Description
            </CollapsibleTrigger>
            <CollapsibleContent>
              <p className="mt-1 text-xs text-muted-foreground">{description}</p>
            </CollapsibleContent>
          </Collapsible>
        )}
      </ItemContent>
    </Item>
  );
};

export default ElementSchemaItem;
