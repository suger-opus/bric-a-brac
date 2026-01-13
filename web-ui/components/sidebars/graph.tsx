"use client";

import { Button } from "@/components/ui/button";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { Item, ItemActions, ItemContent, ItemDescription, ItemTitle } from "@/components/ui/item";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader
} from "@/components/ui/sidebar";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { SchemaEdge, SchemaNode } from "@/types/graph";
import { ChevronDown, PlusIcon } from "lucide-react";
import { useRouter } from "next/navigation";

const nodes: SchemaNode[] = [
  {
    name: "Person",
    color: "rgb(59, 130, 246)",
    nb_properties: 0
  },
  {
    name: "Company",
    color: "rgb(245, 158, 11)",
    nb_properties: 0
  }
];

const edges: SchemaEdge[] = [
  {
    name: "WORKS_AT",
    color: "rgb(16, 185, 129)",
    nb_properties: 0
  }
];

const GraphSidebar = () => {
  const router = useRouter();

  return (
    <Sidebar side="right">
      <SidebarHeader className="pt-2.75">
        <span className="text-base font-semibold">This is the name of the graph</span>
      </SidebarHeader>
      <SidebarContent className="gap-0">
        <Collapsible defaultOpen className="group/collapsible">
          <SidebarGroup>
            <SidebarGroupLabel asChild>
              <CollapsibleTrigger>
                Nodes
                <ChevronDown className="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-180" />
              </CollapsibleTrigger>
            </SidebarGroupLabel>
            <CollapsibleContent>
              <SidebarGroupContent className="space-y-1">
                {nodes.map((node, index) => (
                  <Item key={index} variant="outline" className="py-2 pl-3">
                    <ItemContent>
                      <div className="flex items-center space-x-1">
                        <div
                          className="w-4 h-4 rounded-full"
                          style={{ backgroundColor: node.color }}
                        />
                        <ItemTitle>{node.name}</ItemTitle>
                      </div>
                      <ItemDescription className="text-gray-800 text-xs cursor-pointer">
                        <u>{node.nb_properties} propertie{node.nb_properties !== 1 ? "s" : ""}</u> |
                        {" "}
                        <u>manage properties</u> | <u>delete node</u>
                      </ItemDescription>
                    </ItemContent>
                    <ItemActions>
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <PlusIcon size={14} />
                        </TooltipTrigger>
                        <TooltipContent>
                          Add a node of type {node.name}
                        </TooltipContent>
                      </Tooltip>
                    </ItemActions>
                  </Item>
                ))}
                <Item
                  variant="outline"
                  className="hover:bg-gray-200 cursor-pointer py-2 pl-3 bg-gray-100"
                >
                  <ItemContent>
                    <ItemTitle>New node type</ItemTitle>
                  </ItemContent>
                  <ItemActions>
                    <PlusIcon size={14} />
                  </ItemActions>
                </Item>
              </SidebarGroupContent>
            </CollapsibleContent>
          </SidebarGroup>
        </Collapsible>
        <Collapsible defaultOpen className="group/collapsible">
          <SidebarGroup>
            <SidebarGroupLabel asChild>
              <CollapsibleTrigger>
                Edges
                <ChevronDown className="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-180" />
              </CollapsibleTrigger>
            </SidebarGroupLabel>
            <CollapsibleContent>
              <SidebarGroupContent className="space-y-1">
                {edges.map((edge, index) => (
                  <Item key={index} variant="outline" className="py-2 pl-3">
                    <ItemContent>
                      <div className="flex items-center space-x-1">
                        <div
                          className="w-4 h-4 rounded-full"
                          style={{ backgroundColor: edge.color }}
                        />
                        <ItemTitle>{edge.name}</ItemTitle>
                      </div>
                      <ItemDescription className="text-gray-800 text-xs">
                        <u>{edge.nb_properties} propertie{edge.nb_properties !== 1 ? "s" : ""}</u> |
                        {" "}
                        <u>manage properties</u> | <u>delete edge</u>
                      </ItemDescription>
                    </ItemContent>
                    <ItemActions>
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <PlusIcon size={14} />
                        </TooltipTrigger>
                        <TooltipContent>
                          Add an edge of type {edge.name}
                        </TooltipContent>
                      </Tooltip>
                    </ItemActions>
                  </Item>
                ))}
                <Item
                  variant="outline"
                  className="hover:bg-gray-200 cursor-pointer py-2 pl-3 bg-gray-100"
                >
                  <ItemContent>
                    <ItemTitle>New edge type</ItemTitle>
                  </ItemContent>
                  <ItemActions>
                    <PlusIcon size={14} />
                  </ItemActions>
                </Item>
              </SidebarGroupContent>
            </CollapsibleContent>
          </SidebarGroup>
        </Collapsible>
      </SidebarContent>
      <SidebarFooter className="flex flex-row items-center justify-between">
        <span className="text-sm text-gray-500">Graph ID: 1234567890</span>
        <Button variant="outline" size="sm" onClick={() => router.push("/")}>
          Exit
        </Button>
      </SidebarFooter>
    </Sidebar>
  );
};

export default GraphSidebar;
