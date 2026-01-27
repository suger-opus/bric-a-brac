"use client";

import EdgeSchemaItem from "@/components/items/edge-schema-item";
import NodeSchemaItem from "@/components/items/node-schema-item";
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
import { Toggle } from "@/components/ui/toggle";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useGraph } from "@/contexts/graph-context";
import { Role } from "@/types";
import {
  BookmarkIcon,
  ChevronDown,
  HandHeartIcon,
  HomeIcon,
  PackageIcon,
  SplineIcon
} from "lucide-react";
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
            <div className="space-y-2">
              <div className="space-y-1">
                <p className="text-xl font-bold">
                  {metadata!.name}
                </p>
                <div className="border-l-2 border-black/80 pl-2 text-black/80 text-xs">
                  <p>
                    <i>by</i>{" "}
                    <b>
                      <u>{metadata!.owner_username}</u>
                    </b>{" "}
                    <i>on {metadata!.created_at.toLocaleDateString()}</i>
                  </p>
                  <p>
                    <i>last edited: {metadata!.updated_at.toLocaleDateString()}</i>
                  </p>
                </div>
              </div>
              <div className="flex items-center space-x-1">
                <Badge className="font-bold text-[9px] h-5">
                  {metadata!.is_public ? "PUBLIC" : "PRIVATE"}
                </Badge>
                {metadata!.user_role !== Role.NONE && (
                  <Badge className="font-bold text-[9px] h-5">
                    {metadata!.user_role.toUpperCase()}
                  </Badge>
                )}
                {metadata!.is_public && (
                  <>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Badge variant="outline" className="h-5">
                          <Toggle
                            aria-label="Toggle bookmark"
                            variant="default"
                            pressed={metadata!.is_cheered_by_user}
                            className="h-5 w-fit p-0 font-bold text-[11px] cursor-pointer data-[state=on]:bg-transparent data-[state=on]:*:[svg]:fill-black data-[state=on]:*:[svg]:stroke-black"
                          >
                            <HandHeartIcon />
                            {metadata!.nb_cheers}
                          </Toggle>
                        </Badge>
                      </TooltipTrigger>
                      <TooltipContent>
                        {metadata!.is_cheered_by_user ? "Un-cheer this graph" : "Cheer this graph"}
                      </TooltipContent>
                    </Tooltip>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Badge variant="outline" className="h-5">
                          <Toggle
                            aria-label="Toggle bookmark"
                            variant="default"
                            pressed={metadata!.is_bookmarked_by_user}
                            className="h-5 w-fit p-0 font-bold text-[11px] cursor-pointer data-[state=on]:bg-transparent data-[state=on]:*:[svg]:fill-black data-[state=on]:*:[svg]:stroke-black"
                          >
                            <BookmarkIcon />
                            {metadata!.nb_bookmarks}
                          </Toggle>
                        </Badge>
                      </TooltipTrigger>
                      <TooltipContent>
                        {metadata!.is_bookmarked_by_user
                          ? "Un-bookmark this graph"
                          : "Bookmark this graph"}
                      </TooltipContent>
                    </Tooltip>
                  </>
                )}
                <Badge variant="outline" className="font-bold text-[11px] h-5">
                  <PackageIcon />
                  {metadata!.nb_data_nodes}
                </Badge>
                <Badge variant="outline" className="font-bold text-[11px] h-5">
                  <SplineIcon />
                  {metadata!.nb_data_edges}
                </Badge>
              </div>
              <p className="text-sm">{metadata!.description}</p>
            </div>
          )}
      </SidebarHeader>
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
                    </>
                  )}
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
                    </>
                  )}
              </SidebarGroupContent>
            </CollapsibleContent>
          </SidebarGroup>
        </Collapsible>
      </SidebarContent>
      <SidebarFooter className="flex flex-row items-center">
        <Button
          className="ml-auto"
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
