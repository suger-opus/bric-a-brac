"use client";

import DataTable from "@/components/data-tables/data-table";
import { columns } from "@/components/data-tables/graph-cols";
import NewGraphDialogContent from "@/components/dialog-contents/new-graph-dialog-content";
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
import { Dialog, DialogTrigger } from "@/components/ui/dialog";
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
  const { accessService } = ApiProvider;
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
      const results = await accessService.list();
      setAccessedGraphs(results);
    } catch (error) {
      console.error("Error during getAccesses:", error);
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
                <Button>Create a new graph</Button>
              </EmptyContent>
            </Empty>
          )
          : <DataTable columns={columns} data={accessedGraphs} />}
      </CardContent>
      {accessedGraphs.length > 0 && (
        <CardFooter className="flex flex-col items-start space-y-4">
          <Separator />
          <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
            <DialogTrigger asChild>
              <Button variant="outline" size="sm">
                <PlusIcon /> Create a new graph
              </Button>
            </DialogTrigger>
            <NewGraphDialogContent isOpen={isDialogOpen} onClose={() => setIsDialogOpen(false)} />
          </Dialog>
        </CardFooter>
      )}
    </Card>
  );
};

export default AccessesCard;
