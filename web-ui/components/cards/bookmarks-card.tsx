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
import { Toggle } from "@/components/ui/toggle";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { pluralize, scrollToElement } from "@/lib/utils";
import { GraphMetadata } from "@/types";
import { ArrowUpRightIcon, BookmarkIcon, ExpandIcon, ShrinkIcon } from "lucide-react";
import Link from "next/link";
import { useEffect, useState } from "react";

type BookmarksProps = {
  is_expanded: boolean;
  expand: () => void;
  un_expand: () => void;
};

const BookmarksPage = ({ is_expanded, expand, un_expand }: BookmarksProps) => {
  const [bookmarkedGraphs, setBookmarkedGraphs] = useState<GraphMetadata[]>([]);
  const [isBookmarksLoading, setIsBookmarksLoading] = useState(true);

  // Results pagination
  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 1;
  const totalPages = Math.ceil(bookmarkedGraphs.length / itemsPerPage);
  const paginatedData = bookmarkedGraphs.slice(
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
      scrollToElement("bookmarks-card");
    }, 300);
  };

  const getBookmarks = async () => {
    try {
      setIsBookmarksLoading(true);
      const results = [] as GraphMetadata[];
      setBookmarkedGraphs(results);
      setCurrentPage(0);
    } catch (error) {
      console.error(error);
    } finally {
      setIsBookmarksLoading(false);
    }
  };

  useEffect(() => {
    getBookmarks();
  }, []);

  return (
    <Card id="bookmarks-card" className="h-full">
      <CardHeader>
        <CardTitle>Bookmarks ({bookmarkedGraphs.length})</CardTitle>
        <CardDescription>The list of the public graphs you have bookmarked</CardDescription>
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
              {is_expanded ? "Shrink this view" : "Expand this view"}
            </TooltipContent>
          </Tooltip>
        </CardAction>
      </CardHeader>
      <CardContent className="grow">
        {isBookmarksLoading
          ? (
            <div className="h-full flex items-center justify-center">
              <Spinner />
            </div>
          )
          : bookmarkedGraphs.length === 0
          ? (
            <Empty className="h-full">
              <EmptyHeader>
                <EmptyMedia variant="icon">
                  <BookmarkIcon />
                </EmptyMedia>
                <EmptyTitle>No Bookmarked Graphs Yet</EmptyTitle>
                <EmptyDescription>
                  You haven&apos;t bookmarked any graphs yet. Get started by bookmarking your first
                  graph.
                </EmptyDescription>
              </EmptyHeader>
            </Empty>
          )
          : (
            <div className="w-full space-y-3 overflow-y-auto">
              {paginatedData.map((graph) => (
                <Item key={graph.graph_id} variant="outline" className="relative h-full">
                  <ItemContent className="h-full">
                    <ItemTitle className="line-clamp-1 max-w-50">
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
                    <ItemDescription className="grow mt-1 line-clamp-3">
                      {graph.description}
                    </ItemDescription>
                  </ItemContent>
                  <ItemActions className="absolute top-2 right-2 gap-0">
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <div>
                          <Toggle
                            aria-label="Toggle bookmark"
                            size="sm"
                            variant="default"
                            pressed
                            className="cursor-pointer data-[state=on]:bg-transparent data-[state=on]:*:[svg]:fill-black data-[state=on]:*:[svg]:stroke-black"
                          >
                            <BookmarkIcon />
                          </Toggle>
                        </div>
                      </TooltipTrigger>
                      <TooltipContent>
                        Un-bookmarked this graph
                      </TooltipContent>
                    </Tooltip>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button variant="ghost" size="icon-sm" asChild>
                          <Link href={`/graph?graph_id=${graph.graph_id}`}>
                            <ArrowUpRightIcon />
                          </Link>
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent>
                        Open this graph
                      </TooltipContent>
                    </Tooltip>
                  </ItemActions>
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

export default BookmarksPage;
