"use client";

import NewGraphDialogContent from "@/components/dashboard/contents/new-graph-content";
import DataTable from "@/components/dashboard/tables/data-table";
import { columns } from "@/components/dashboard/tables/graph-cols";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardAction,
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
import { Separator } from "@/components/ui/separator";
import { Spinner } from "@/components/ui/spinner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { ApiProvider } from "@/lib/api/provider";
import { scrollToElement } from "@/lib/utils";
import { GraphMetadata } from "@/types";
import { ExpandIcon, PlusIcon, ShrinkIcon, VectorSquareIcon } from "lucide-react";
import { useEffect, useState } from "react";

type AccessesProps = {
  is_expanded: boolean;
  expand: () => void;
  un_expand: () => void;
};

const AccessesCard = ({ is_expanded, expand, un_expand }: AccessesProps) => {
  const { graphService } = ApiProvider;
  const [accessedGraphs, setAccessedGraphs] = useState<GraphMetadata[]>([]);
  const [isAccessesLoading, setIsAccessesLoading] = useState(true);
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleExpand = () => {
    if (is_expanded) {
      un_expand();
    } else {
      expand();
    }
    setTimeout(() => {
      scrollToElement("accesses-card");
    }, 300);
  };

  const getAccesses = async () => {
    try {
      setIsAccessesLoading(true);
      const results = await graphService.getAllMetadata();
      setAccessedGraphs(results);
    } catch (error) {
      console.error(error);
    } finally {
      setIsAccessesLoading(false);
    }
  };

  useEffect(() => {
    getAccesses();
  }, []);

  return (
    <Card id="accesses-card" className="h-full">
      <CardHeader>
        <CardTitle>Your Graphs ({accessedGraphs.length})</CardTitle>
        <CardDescription>List of the graphs you have access to</CardDescription>
        <CardAction>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                size="icon-sm"
                onClick={handleExpand}
              >
                {is_expanded ? <ShrinkIcon /> : <ExpandIcon />}
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              {is_expanded ? "Collapse this view" : "Expand this view"}
            </TooltipContent>
          </Tooltip>
        </CardAction>
      </CardHeader>
      <CardContent className="grow">
        {isAccessesLoading
          ? (
            <div className="h-full flex items-center justify-center">
              <Spinner />
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
          : <DataTable columns={columns} data={accessedGraphs} />}
      </CardContent>
      {accessedGraphs.length > 0 && (
        <CardFooter className="flex flex-col items-start space-y-4">
          <Separator />
          <Button variant="outline" size="sm" onClick={() => setIsDialogOpen(true)}>
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
