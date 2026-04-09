"use client";

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger
} from "@/components/ui/alert-dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { graphService } from "@/lib/api/services/graph-service";
import { GraphMetadata, Role } from "@/types";
import { ColumnDef } from "@tanstack/react-table";
import { Trash2Icon } from "lucide-react";
import { toast } from "sonner";

export const columns = (onRefresh: () => void): ColumnDef<GraphMetadata>[] => [
  {
    accessorKey: "name",
    header: "Name",
    cell: ({ row }) => (
      <span className="truncate max-w-48 block font-medium">
        {row.original.name}
      </span>
    )
  },
  {
    accessorKey: "is_public",
    header: "Visibility",
    cell: ({ row }) =>
      row.original.is_public
        ? <Badge variant="default">Public</Badge>
        : <Badge variant="outline">Private</Badge>
  },
  {
    accessorKey: "user_access",
    header: "Role",
    cell: ({ row }) => <Badge variant="secondary">{row.original.user_role}</Badge>
  },
  {
    id: "actions",
    cell: ({ row }) => {
      if (row.original.user_role !== Role.OWNER) { return null; }
      return (
        <div className="flex justify-end w-full" onClick={(e) => e.stopPropagation()}>
          <AlertDialog>
            <Tooltip>
              <TooltipTrigger asChild>
                <AlertDialogTrigger asChild>
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    className="text-muted-foreground hover:text-destructive"
                  >
                    <Trash2Icon className="h-4 w-4" />
                  </Button>
                </AlertDialogTrigger>
              </TooltipTrigger>
              <TooltipContent>Delete graph</TooltipContent>
            </Tooltip>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Delete &ldquo;{row.original.name}&rdquo;?</AlertDialogTitle>
                <AlertDialogDescription>
                  This will permanently delete the graph and all its data. This action cannot be
                  undone.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                  onClick={async () => {
                    try {
                      await graphService.delete(row.original.graph_id);
                      toast.success("Graph deleted");
                      onRefresh();
                    } catch {
                      toast.error("Could not delete graph");
                    }
                  }}
                >
                  Delete
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      );
    }
  }
];
