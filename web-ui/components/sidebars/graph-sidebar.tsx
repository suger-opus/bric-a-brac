"use client";

import NewNodeDialogContent from "@/components/dialog-contents/new-node-dialog-content";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { Dialog, DialogTrigger } from "@/components/ui/dialog";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@/components/ui/hover-card";
import { Item, ItemActions, ItemContent, ItemTitle } from "@/components/ui/item";
import { Separator } from "@/components/ui/separator";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader
} from "@/components/ui/sidebar";
import { Skeleton } from "@/components/ui/skeleton";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useGraph } from "@/contexts/graph-context";
import { pluralize } from "@/lib/utils";
import { EdgeSchema, NodeSchema, PropertyType, Role } from "@/types";
import {
  ChevronDown,
  EyeClosedIcon,
  EyeIcon,
  HomeIcon,
  PlusIcon,
  SettingsIcon
} from "lucide-react";
import { useRouter } from "next/navigation";
import { Fragment, useState } from "react";

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

type ElementSchemaItemProps = {
  role: Role;
  nodeSchema?: NodeSchema;
  edgeSchema?: EdgeSchema;
};

const ElementSchemaItem = ({ role, nodeSchema, edgeSchema }: ElementSchemaItemProps) => {
  const {
    displayedNodeProperties,
    displayedEdgeProperties,
    updateDisplayedNodeProperty,
    updateDisplayedEdgeProperty
  } = useGraph();
  const kind = nodeSchema ? "node" : "edge";
  const color = nodeSchema?.color || edgeSchema?.color || "";
  const label = nodeSchema?.label || edgeSchema?.label || "";
  const formatted_label = nodeSchema?.formatted_label || edgeSchema?.formatted_label || "";
  const properties = nodeSchema?.properties || edgeSchema?.properties || [];
  const displayedProperty = nodeSchema
    ? displayedNodeProperties[formatted_label]
    : displayedEdgeProperties[formatted_label];
  const updateDisplayedProperty = nodeSchema
    ? updateDisplayedNodeProperty
    : updateDisplayedEdgeProperty;

  if (!nodeSchema && !edgeSchema) {
    return <Skeleton className="h-16" />;
  }

  return (
    <Item variant="outline" className="py-2 pl-3">
      <ItemContent>
        <div className="flex justify-between items-center">
          <div className="flex items-center space-x-1">
            <div
              className={kind === "node" ? "w-4 h-4 rounded-full" : "w-4 h-2 rounded-xs"}
              style={{ backgroundColor: color }}
            />
            <ItemTitle>{label}</ItemTitle>
          </div>
          {[Role.OWNER, Role.ADMIN, Role.EDITOR].includes(role) && (
            <Tooltip>
              <TooltipTrigger asChild>
                <Button variant="ghost" size="icon-sm">
                  <PlusIcon className="h-3.5 w-3.5" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                Add a{kind === "edge" ? "n" : ""} {kind} of type {label}
              </TooltipContent>
            </Tooltip>
          )}
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
                    <TableCell>
                      <Tooltip>
                        <TooltipTrigger className="font-medium text-ellipsis overflow-hidden whitespace-nowrap max-w-32">
                          {property.label}
                        </TooltipTrigger>
                        <TooltipContent>
                          {property.label}
                        </TooltipContent>
                      </Tooltip>
                    </TableCell>
                    <TableCell>
                      {property.metadata.property_type === PropertyType.SELECT
                        ? (
                          <HoverCard>
                            <HoverCardTrigger className="cursor-pointer">
                              <Badge variant="outline" className="font-bold text-[9px]">
                                {property.metadata.property_type.toUpperCase()}{" "}
                                ({property.metadata.details.options!.length})
                              </Badge>
                            </HoverCardTrigger>
                            <HoverCardContent className="w-fit">
                              <span className="text-sm font-semibold">
                                Select Options ({property.metadata.details.options!.length})
                              </span>
                              <div className="mt-3 max-h-80 overflow-y-auto no-scrollbar">
                                {property.metadata.details.options!.map((p, index) => (
                                  <Fragment key={index}>
                                    <div className="text-sm">{p}</div>
                                    {index < property.metadata.details.options!.length - 1 && (
                                      <Separator className="my-2" />
                                    )}
                                  </Fragment>
                                ))}
                              </div>
                            </HoverCardContent>
                          </HoverCard>
                        )
                        : (
                          <Badge variant="outline" className="font-bold text-[9px]">
                            {property.metadata.property_type.toUpperCase()}
                          </Badge>
                        )}
                    </TableCell>
                    <TableCell>
                      {property.metadata.details.required ? "Required" : "Optional"}
                    </TableCell>
                    <TableCell>
                      {displayedProperty === property.formatted_label
                        ? (
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Button
                                variant="ghost"
                                size="icon-sm"
                                onClick={() => updateDisplayedProperty(formatted_label, undefined)}
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
                                onClick={() =>
                                  updateDisplayedProperty(
                                    formatted_label,
                                    property.formatted_label
                                  )}
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

const GraphSidebar = () => {
  const router = useRouter();
  const { metadata, isLoaded, schema } = useGraph();

  return (
    <Sidebar side="right">
      <SidebarHeader className="pt-2.75">
        {!isLoaded
          ? <Skeleton className="h-8" />
          : (
            <span className="text-base font-semibold">
              {metadata!.name}
            </span>
          )}
      </SidebarHeader>
      <SidebarContent className="gap-0">
        <Collapsible defaultOpen className="group/collapsible-nodes">
          <SidebarGroup>
            <SidebarGroupLabel asChild>
              <CollapsibleTrigger>
                Nodes
                <ChevronDown className="ml-auto transition-transform group-data-[state=open]/collapsible-nodes:rotate-180" />
              </CollapsibleTrigger>
            </SidebarGroupLabel>
            <CollapsibleContent>
              <SidebarGroupContent className="space-y-1">
                {!isLoaded
                  ? (
                    <>
                      <Skeleton className="h-8" />
                      <Skeleton className="h-8" />
                      <Skeleton className="h-8" />
                    </>
                  )
                  : (
                    <>
                      {schema!.nodes.map((node, index) => (
                        <ElementSchemaItem
                          key={index}
                          nodeSchema={node}
                          role={metadata!.user_role}
                        />
                      ))}
                      <NewNodeItem />
                    </>
                  )}
              </SidebarGroupContent>
            </CollapsibleContent>
          </SidebarGroup>
        </Collapsible>
        <Collapsible defaultOpen className="group/collapsible-edges">
          <SidebarGroup>
            <SidebarGroupLabel asChild>
              <CollapsibleTrigger>
                Edges
                <ChevronDown className="ml-auto transition-transform group-data-[state=open]/collapsible-edges:rotate-180" />
              </CollapsibleTrigger>
            </SidebarGroupLabel>
            <CollapsibleContent>
              <SidebarGroupContent className="space-y-1">
                {!isLoaded
                  ? (
                    <>
                      <Skeleton className="h-8" />
                      <Skeleton className="h-8" />
                      <Skeleton className="h-8" />
                    </>
                  )
                  : (
                    <>
                      {schema!.edges.map((edge, index) => (
                        <ElementSchemaItem
                          key={index}
                          edgeSchema={edge}
                          role={metadata!.user_role}
                        />
                      ))}
                      <NewEdgeItem />
                    </>
                  )}
              </SidebarGroupContent>
            </CollapsibleContent>
          </SidebarGroup>
        </Collapsible>
      </SidebarContent>
      <SidebarFooter className="flex flex-row items-center justify-between">
        <Button variant="outline" size="sm">
          <SettingsIcon /> Settings
        </Button>
        <Button size="sm" onClick={() => router.push("/")}>
          <HomeIcon />Exit to Home
        </Button>
      </SidebarFooter>
    </Sidebar>
  );
};

export default GraphSidebar;
