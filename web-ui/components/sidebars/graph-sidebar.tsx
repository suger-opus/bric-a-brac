"use client";

import NewNodeDialogContent from "@/components/dialog-contents/new-node-dialog-content";
import { Button } from "@/components/ui/button";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { Dialog, DialogTrigger } from "@/components/ui/dialog";
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
import { Spinner } from "@/components/ui/spinner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { graphMetadata } from "@/lib/api/data";
import { pluralize } from "@/lib/utils";
import { EdgeSchema, GraphMetadata, GraphSchema, NodeSchema } from "@/types";
import { ChevronDown, PlusIcon } from "lucide-react";
import { useRouter } from "next/navigation";
import { useState } from "react";

const NewNodeItem = () => {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
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
      </DialogTrigger>
      <NewNodeDialogContent isOpen={isDialogOpen} onClose={() => setIsDialogOpen(false)} />
    </Dialog>
  );
};

const NodeItem = ({ node }: { node: NodeSchema; }) => {
  return (
    <Item variant="outline" className="py-2 pl-3">
      <ItemContent>
        <div className="flex items-center space-x-1">
          <div
            className="w-4 h-4 rounded-full"
            style={{ backgroundColor: node.color }}
          />
          <ItemTitle>{node.label}</ItemTitle>
        </div>
        <ItemDescription className="text-gray-800 text-xs cursor-pointer w-fit">
          <u>
            {node.properties.length} {pluralize(node.properties.length, "property", "properties")}
          </u>{" "}
          | <u>manage properties</u> | <u>delete node</u>
        </ItemDescription>
      </ItemContent>
      <ItemActions>
        <Tooltip>
          <TooltipTrigger asChild>
            <PlusIcon size={14} />
          </TooltipTrigger>
          <TooltipContent>
            Add a node of type {node.label}
          </TooltipContent>
        </Tooltip>
      </ItemActions>
    </Item>
  );
};

const NewEdgeItem = () => {
  return (
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
  );
};

const EdgeItem = ({ edge }: { edge: EdgeSchema; }) => {
  return (
    <Item variant="outline" className="py-2 pl-3">
      <ItemContent>
        <div className="flex items-center space-x-1">
          <div
            className="w-4 h-4 rounded-full"
            style={{ backgroundColor: edge.color }}
          />
          <ItemTitle>{edge.label}</ItemTitle>
        </div>
        <ItemDescription className="text-gray-800 text-xs cursor-pointer w-fit">
          <u>
            {edge.properties.length} {pluralize(edge.properties.length, "property", "properties")}
          </u>{" "}
          | <u>manage properties</u> | <u>delete edge</u>
        </ItemDescription>
      </ItemContent>
      <ItemActions>
        <Tooltip>
          <TooltipTrigger asChild>
            <PlusIcon size={14} />
          </TooltipTrigger>
          <TooltipContent>
            Add an edge of type {edge.label}
          </TooltipContent>
        </Tooltip>
      </ItemActions>
    </Item>
  );
};

type GraphSidebarProps = {
  graphMetadata: GraphMetadata | null;
  graphSchema: GraphSchema | null;
};

const GraphSidebar = ({ graphSchema }: GraphSidebarProps) => {
  const router = useRouter();

  return (
    <Sidebar side="right">
      <SidebarHeader className="pt-2.75">
        <span className="text-base font-semibold">
          {graphMetadata ? graphMetadata.name : "Loading..."}
        </span>
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
                {!graphSchema
                  ? (
                    <div className="flex justify-center">
                      <Spinner />
                    </div>
                  )
                  : (
                    <>
                      {graphSchema.nodes.map((node, index) => <NodeItem key={index} node={node} />)}
                      <NewNodeItem />
                    </>
                  )}
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
                {!graphSchema
                  ? (
                    <div className="flex justify-center">
                      <Spinner />
                    </div>
                  )
                  : (
                    <>
                      {graphSchema.edges.map((edge, index) => <EdgeItem key={index} edge={edge} />)}
                      <NewEdgeItem />
                    </>
                  )}
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
