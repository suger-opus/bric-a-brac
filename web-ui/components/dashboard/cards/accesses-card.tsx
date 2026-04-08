"use client";

import NewGraphDialogContent from "@/components/dashboard/contents/new-graph-content";
import DataTable from "@/components/dashboard/tables/data-table";
import { columns } from "@/components/dashboard/tables/graph-cols";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from "@/components/ui/card";
import { Dialog } from "@/components/ui/dialog";
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from "@/components/ui/empty";
import { Skeleton } from "@/components/ui/skeleton";
import { graphService } from "@/lib/api/services/graph-service";
import { GraphMetadata } from "@/types";
import { PlusIcon, VectorSquareIcon } from "lucide-react";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { toast } from "sonner";

const AccessesCard = () => {
  const router = useRouter();
  const [accessedGraphs, setAccessedGraphs] = useState<GraphMetadata[]>([]);
  const [isAccessesLoading, setIsAccessesLoading] = useState(true);
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const getAccesses = async () => {
    try {
      setIsAccessesLoading(true);
      const results = await graphService.list();
      setAccessedGraphs(results);
    } catch {
      toast.error("Could not load your graphs");
    } finally {
      setIsAccessesLoading(false);
    }
  };

  useEffect(() => {
    getAccesses();
  }, []);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Your Graphs ({accessedGraphs.length})</CardTitle>
        <CardDescription>List of the graphs you have access to</CardDescription>
      </CardHeader>
      <CardContent className="grow">
        {isAccessesLoading
          ? (
            <div className="space-y-3">
              <Skeleton className="h-10 w-full" />
              <Skeleton className="h-10 w-full" />
              <Skeleton className="h-10 w-full" />
              <Skeleton className="h-10 w-3/4" />
            </div>
          )
          : accessedGraphs.length === 0
          ? (
            <Empty className="h-full">
              <EmptyHeader>
                <EmptyMedia variant="icon">
                  <VectorSquareIcon />
                </EmptyMedia>
                <EmptyTitle>No Graphs Yet</EmptyTitle>
                <EmptyDescription>
                  You have no access to any graphs yet. Get started by creating your first graph.
                </EmptyDescription>
              </EmptyHeader>
              <EmptyContent>
                <Button onClick={() => setIsDialogOpen(true)}>Create a new graph</Button>
              </EmptyContent>
            </Empty>
          )
          : (
            <DataTable
              columns={columns(getAccesses)}
              data={accessedGraphs}
              onRowClick={(graph) => router.push(`/graph/${graph.graph_id}`)}
            />
          )}
      </CardContent>
      {accessedGraphs.length > 0 && (
        <CardFooter>
          <Button
            variant="outline"
            size="sm"
            onClick={() => setIsDialogOpen(true)}
          >
            <PlusIcon /> Create a new graph
          </Button>
        </CardFooter>
      )}
      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
        <NewGraphDialogContent isOpen={isDialogOpen} onClose={() => setIsDialogOpen(false)} />
      </Dialog>
    </Card>
  );
};

export default AccessesCard;
