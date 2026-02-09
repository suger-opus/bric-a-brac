"use client";

import { Badge } from "@/components/ui/badge";
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
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from "@/components/ui/empty";
import { Item, ItemActions, ItemContent, ItemDescription, ItemTitle } from "@/components/ui/item";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationNext,
  PaginationPrevious
} from "@/components/ui/pagination";
import { Separator } from "@/components/ui/separator";
import { Spinner } from "@/components/ui/spinner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { pluralize, scrollToElement } from "@/lib/utils";
import { GraphMetadata } from "@/types";
import { ArrowUpRightIcon, ExpandIcon, HandHeartIcon, ShrinkIcon } from "lucide-react";
import Link from "next/link";
import { useEffect, useState } from "react";

type CheersProps = {
  is_expanded: boolean;
  expand: () => void;
  un_expand: () => void;
};

const CheersCard = ({ is_expanded, expand, un_expand }: CheersProps) => {
  const [cheeredGraphs, setCheeredGraphs] = useState<GraphMetadata[]>([]);
  const [isCheersLoading, setIsCheersLoading] = useState(true);

  // Results pagination
  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 1;
  const totalPages = Math.ceil(cheeredGraphs.length / itemsPerPage);
  const paginatedData = cheeredGraphs.slice(
    currentPage * itemsPerPage,
    currentPage * itemsPerPage + itemsPerPage
  );

  const handleExpand = () => {
    if (is_expanded) {
      un_expand();
    } else {
      expand();
    }
    setTimeout(() => {
      scrollToElement("cheers-card");
    }, 300);
  };

  const getCheers = async () => {
    try {
      setIsCheersLoading(true);
      const results = [] as GraphMetadata[];
      setCheeredGraphs(results);
      setCurrentPage(0);
    } catch (error) {
      console.error(error);
    } finally {
      setIsCheersLoading(false);
    }
  };

  useEffect(() => {
    getCheers();
  }, []);

  return (
    <Card id="cheers-card" className="h-full">
      <CardHeader>
        <CardTitle>Cheers ({cheeredGraphs.length})</CardTitle>
        <CardDescription>The list of the public graphs you have cheered</CardDescription>
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
        {isCheersLoading
          ? (
            <div className="h-full flex items-center justify-center">
              <Spinner />
            </div>
          )
          : cheeredGraphs.length === 0
          ? (
            <Empty className="h-full">
              <EmptyHeader>
                <EmptyMedia variant="icon">
                  <HandHeartIcon />
                </EmptyMedia>
                <EmptyTitle>No Cheered Graphs Yet</EmptyTitle>
                <EmptyDescription>
                  You haven&apos;t cheered any graphs yet. Get started by cheering your first graph.
                </EmptyDescription>
              </EmptyHeader>
            </Empty>
          )
          : (
            <div className="w-full space-y-3 overflow-y-auto">
              {paginatedData.map((graph) => (
                <Item key={graph.graph_id} variant="outline" asChild>
                  <Link href={`/graph?graph_id=${graph.graph_id}`}>
                    <ItemContent>
                      <ItemTitle className="line-clamp-1 max-w-80">
                        {graph.name}
                      </ItemTitle>
                      <ItemDescription className="space-x-1">
                        <Badge variant="outline">
                          {graph.nb_data_nodes} {pluralize(graph.nb_data_nodes, "node", "nodes")}
                        </Badge>
                        <Badge variant="outline">
                          {graph.nb_data_edges} {pluralize(graph.nb_data_edges, "edge", "edges")}
                        </Badge>
                      </ItemDescription>
                    </ItemContent>
                    <ItemActions>
                      <ArrowUpRightIcon size={16} />
                    </ItemActions>
                  </Link>
                </Item>
              ))}
            </div>
          )}
      </CardContent>
      {totalPages > 1 && (
        <CardFooter className="flex flex-col items-end space-y-4">
          <Separator />
          <div className="w-fit">
            <Pagination>
              <PaginationContent>
                <PaginationItem>
                  <PaginationPrevious
                    onClick={() => setCurrentPage(Math.max(0, currentPage - 1))}
                    aria-disabled={currentPage === 0}
                    size="sm"
                    className={currentPage === 0
                      ? "pointer-events-none opacity-50"
                      : ""}
                  />
                </PaginationItem>
                <PaginationItem>
                  <span className="text-sm text-muted-foreground mx-1">
                    Page {currentPage + 1} of {totalPages}
                  </span>
                </PaginationItem>
                <PaginationItem>
                  <PaginationNext
                    size="sm"
                    onClick={() => setCurrentPage(Math.min(totalPages - 1, currentPage + 1))}
                    aria-disabled={currentPage === totalPages - 1}
                    className={currentPage === totalPages - 1
                      ? "pointer-events-none opacity-50"
                      : ""}
                  />
                </PaginationItem>
              </PaginationContent>
            </Pagination>
          </div>
        </CardFooter>
      )}
    </Card>
  );
};

export default CheersCard;
