"use client";

import ChatPanel from "@/components/graph/chat-panel";
import EdgeSchemaItem from "@/components/graph/items/edge-schema-item";
import NodeSchemaItem from "@/components/graph/items/node-schema-item";
import { Badge } from "@/components/ui/badge";
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
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useGraph } from "@/contexts/graph-context";
import { BotIcon, ChevronDown, HomeIcon, ListTreeIcon } from "lucide-react";
import { useRouter } from "next/navigation";

const GraphSidebar = () => {
  const router = useRouter();
  const { metadata, isLoaded, schema } = useGraph();

  return (
    <Sidebar side="right">
      <SidebarHeader className="pt-2.75">
        {!isLoaded
          ? (
            <div className="space-y-2">
              <Skeleton className="h-8" />
              <Skeleton className="h-4" />
              <Skeleton className="h-24" />
            </div>
          )
          : (
            <div className="space-y-2">
              <div className="space-y-1">
                <p className="text-xl font-bold">{metadata!.name}</p>
                <div className="border-l-2 border-black/80 pl-2 text-black/80 text-xs">
                  <p>
                    <i>by</i>{" "}
                    <b>
                      <u>{metadata!.owner_username}</u>
                    </b>{" "}
                    <i>on {new Date(metadata!.created_at).toLocaleDateString()}</i>
                  </p>
                  <p>
                    <i>last edited: {new Date(metadata!.updated_at).toLocaleDateString()}</i>
                  </p>
                </div>
              </div>
              <div className="flex items-center space-x-1">
                <Badge className="font-bold text-[9px] h-5">
                  {metadata!.is_public ? "PUBLIC" : "PRIVATE"}
                </Badge>
              </div>
              {metadata!.description && (
                <p className="text-sm text-muted-foreground">{metadata!.description}</p>
              )}
            </div>
          )}
      </SidebarHeader>

      <Tabs defaultValue="chat" className="flex flex-col flex-1 min-h-0">
        <div className="px-3">
          <TabsList className="w-full">
            <TabsTrigger value="chat" className="flex-1 gap-1">
              <BotIcon className="h-3.5 w-3.5" />Chat
            </TabsTrigger>
            <TabsTrigger value="schema" className="flex-1 gap-1">
              <ListTreeIcon className="h-3.5 w-3.5" />Schema
            </TabsTrigger>
          </TabsList>
        </div>

        <TabsContent
          value="chat"
          forceMount
          className="flex-1 min-h-0 mt-0 data-[state=inactive]:hidden"
        >
          <ChatPanel />
        </TabsContent>

        <TabsContent value="schema" className="flex-1 min-h-0 mt-0 overflow-y-auto">
          <SidebarContent className="gap-0">
            <Collapsible defaultOpen className="group/collapsible-nodes">
              <SidebarGroup>
                <SidebarGroupLabel asChild>
                  <CollapsibleTrigger className="cursor-pointer">
                    Nodes
                    <ChevronDown className="ml-auto transition-transform group-data-[state=open]/collapsible-nodes:rotate-180" />
                  </CollapsibleTrigger>
                </SidebarGroupLabel>
                <CollapsibleContent>
                  <SidebarGroupContent className="space-y-1">
                    {!isLoaded
                      ? <>{[1, 2, 3].map((i) => <Skeleton key={i} className="h-8" />)}</>
                      : schema!.nodes.map((node) => (
                        <NodeSchemaItem key={node.node_schema_id} schema={node} />
                      ))}
                  </SidebarGroupContent>
                </CollapsibleContent>
              </SidebarGroup>
            </Collapsible>

            <Collapsible defaultOpen className="group/collapsible-edges">
              <SidebarGroup>
                <SidebarGroupLabel asChild>
                  <CollapsibleTrigger className="cursor-pointer">
                    Edges
                    <ChevronDown className="ml-auto transition-transform group-data-[state=open]/collapsible-edges:rotate-180" />
                  </CollapsibleTrigger>
                </SidebarGroupLabel>
                <CollapsibleContent>
                  <SidebarGroupContent className="space-y-1">
                    {!isLoaded
                      ? <>{[1, 2, 3].map((i) => <Skeleton key={i} className="h-8" />)}</>
                      : schema!.edges.map((edge) => (
                        <EdgeSchemaItem key={edge.edge_schema_id} schema={edge} />
                      ))}
                  </SidebarGroupContent>
                </CollapsibleContent>
              </SidebarGroup>
            </Collapsible>
          </SidebarContent>
        </TabsContent>
      </Tabs>

      <SidebarFooter className="flex flex-row items-center">
        <Button className="ml-auto" size="sm" onClick={() => router.push("/")}>
          <HomeIcon />Exit to Home
        </Button>
      </SidebarFooter>
    </Sidebar>
  );
};

export default GraphSidebar;
