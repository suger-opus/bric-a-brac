"use client";

import NewEdgeSchemaButton from "@/components/buttons/new-edge-schema-button";
import NewNodeSchemaButton from "@/components/buttons/new-node-schema-button";
import EdgeSchemaItem from "@/components/items/edge-schema-item";
import NodeSchemaItem from "@/components/items/node-schema-item";
import { Button } from "@/components/ui/button";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
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
import { useGraph } from "@/contexts/graph-context";
import { ChevronDown, HomeIcon, SettingsIcon } from "lucide-react";
import { useRouter } from "next/navigation";

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
                        <NodeSchemaItem key={index} schema={node} />
                      ))}
                      <NewNodeSchemaButton />
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
                        <EdgeSchemaItem key={index} schema={edge} />
                      ))}
                      <NewEdgeSchemaButton />
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
        <Button
          size="sm"
          onClick={() =>
            router.push("/")}
        >
          <HomeIcon />Exit to Home
        </Button>
      </SidebarFooter>
    </Sidebar>
  );
};

export default GraphSidebar;
