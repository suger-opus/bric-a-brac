"use client";

import { Badge } from "@/components/ui/badge";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { GraphMetadata, Role } from "@/types";
import { ColumnDef } from "@tanstack/react-table";
import { ArrowUpRightIcon, BanIcon, BookmarkCheckIcon, HandHeartIcon, Trash2Icon } from "lucide-react";
import Link from "next/link";
import { graphService } from "@/lib/api/services/graph-service";
import { toast } from "sonner";

export const columns = (onRefresh: () => void): ColumnDef<GraphMetadata>[] => [
  {
    accessorKey: "name",
    header: () => {
      return (
        <Tooltip>
          <TooltipTrigger>
            Name
          </TooltipTrigger>
          <TooltipContent>
            Name of the graph
          </TooltipContent>
        </Tooltip>
      );
    },
    cell: ({ row }) => {
      return (
        <Tooltip>
          <TooltipTrigger>
            <div className="text-ellipsis overflow-hidden whitespace-nowrap max-w-40">
              {row.original.name}
            </div>
          </TooltipTrigger>
          <TooltipContent>
            {row.original.name}
          </TooltipContent>
        </Tooltip>
      );
    }
  },
  {
    accessorKey: "is_public",
    header: () => {
      return (
        <Tooltip>
          <TooltipTrigger>
            Status
          </TooltipTrigger>
          <TooltipContent>
            Whether the graph is public or private
          </TooltipContent>
        </Tooltip>
      );
    },
    cell: ({ row }) => {
      return row.original.is_public
        ? <Badge variant="default">public</Badge>
        : <Badge variant="outline">private</Badge>;
    }
  },
  {
    accessorKey: "user_access",
    header: () => {
      return (
        <Tooltip>
          <TooltipTrigger>
            Role
          </TooltipTrigger>
          <TooltipContent>
            Your role on the graph (admin, writer, reader)
          </TooltipContent>
        </Tooltip>
      );
    },
    cell: ({ row }) => {
      if (row.original.user_role === Role.ADMIN || row.original.user_role === Role.OWNER) {
        return (
          <Badge variant="secondary" className="bg-gray-300 text-black">
            {row.original.user_role}
          </Badge>
        );
      }
      if (row.original.user_role === Role.EDITOR) {
        return (
          <Badge variant="secondary" className="bg-gray-200 text-black">
            {row.original.user_role}
          </Badge>
        );
      }
      if (row.original.user_role === Role.VIEWER) {
        return (
          <Badge variant="secondary" className="bg-gray-100 text-black">
            {row.original.user_role}
          </Badge>
        );
      }
    }
  },
  {
    accessorKey: "nb_data_nodes",
    header: () => {
      return (
        <Tooltip>
          <TooltipTrigger>
            Nodes
          </TooltipTrigger>
          <TooltipContent>
            Number of nodes in the graph
          </TooltipContent>
        </Tooltip>
      );
    }
  },
  {
    accessorKey: "nb_data_edges",
    header: () => {
      return (
        <Tooltip>
          <TooltipTrigger>
            Edges
          </TooltipTrigger>
          <TooltipContent>
            Number of edges in the graph
          </TooltipContent>
        </Tooltip>
      );
    }
  },
  {
    accessorKey: "nb_bookmarks",
    header: () => {
      return (
        <Tooltip>
          <TooltipTrigger>
            <BookmarkCheckIcon size={16} />
          </TooltipTrigger>
          <TooltipContent>
            Number of bookmarks of the graph
          </TooltipContent>
        </Tooltip>
      );
    },
    cell: ({ row }) => {
      return row.original.is_public ? row.original.nb_bookmarks : (
        <Tooltip>
          <TooltipTrigger>
            <BanIcon size={12} />
          </TooltipTrigger>
          <TooltipContent>
            A private graph can&apos;t have bookmarks
          </TooltipContent>
        </Tooltip>
      );
    }
  },
  {
    accessorKey: "nb_cheers",
    header: () => {
      return (
        <Tooltip>
          <TooltipTrigger>
            <HandHeartIcon size={16} />
          </TooltipTrigger>
          <TooltipContent>
            Number of cheers of the graph
          </TooltipContent>
        </Tooltip>
      );
    },
    cell: ({ row }) => {
      return row.original.is_public ? row.original.nb_cheers : (
        <Tooltip>
          <TooltipTrigger>
            <BanIcon size={12} />
          </TooltipTrigger>
          <TooltipContent>
            A private graph can&apos;t have cheers
          </TooltipContent>
        </Tooltip>
      );
    }
  },
  {
    id: "actions",
    cell: ({ row }) => {
      return (
        <div className="flex items-center gap-2 justify-center">
          <Link
            href={`/graph/${row.original.graph_id}`}
            className="flex items-center justify-center"
          >
            <Tooltip>
              <TooltipTrigger className="cursor-pointer">
                <ArrowUpRightIcon size={14} />
              </TooltipTrigger>
              <TooltipContent>
                Open graph
              </TooltipContent>
            </Tooltip>
          </Link>
          {row.original.user_role === Role.OWNER && (
            <Tooltip>
              <TooltipTrigger
                className="cursor-pointer text-muted-foreground hover:text-destructive transition-colors"
                onClick={async () => {
                  if (!confirm(`Delete graph "${row.original.name}"? This cannot be undone.`)) return;
                  try {
                    await graphService.deleteGraph(row.original.graph_id);
                    toast.success("Graph deleted");
                    onRefresh();
                  } catch {
                    // toast already shown by client
                  }
                }}
              >
                <Trash2Icon size={14} />
              </TooltipTrigger>
              <TooltipContent>
                Delete graph
              </TooltipContent>
            </Tooltip>
          )}
        </div>
      );
    }
  }
];
