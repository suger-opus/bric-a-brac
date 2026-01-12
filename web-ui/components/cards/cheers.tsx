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
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { ArrowUpRightIcon, ExpandIcon, HandHeartIcon, ShrinkIcon } from "lucide-react";
import { useState } from "react";

type CheersProps = {
  is_expanded: boolean;
  expand: () => void;
  un_expand: () => void;
};

const Cheers = ({ is_expanded, expand, un_expand }: CheersProps) => {
  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 3;

  const data: {
    id: string;
    name: string;
    description: string;
    is_public: boolean;
    user_access: "admin" | "writer" | "reader";
    nb_nodes: number;
    nb_edges: number;
    nb_cheers: number;
    nb_bookmarks: number;
  }[] = [{
    id: "graph-1",
    name: "Graph One",
    description: "This is the description for Graph One.",
    is_public: true,
    user_access: "admin",
    nb_nodes: 150,
    nb_edges: 300,
    nb_cheers: 25,
    nb_bookmarks: 10
  }, {
    id: "graph-2",
    name: "Graph Two",
    description: "This is the description for Graph Two.",
    is_public: false,
    user_access: "writer",
    nb_nodes: 80,
    nb_edges: 120,
    nb_cheers: 0,
    nb_bookmarks: 0
  }, {
    id: "graph-3",
    name: "Graph Three",
    description: "This is the description for Graph Three.",
    is_public: true,
    user_access: "reader",
    nb_nodes: 200,
    nb_edges: 450,
    nb_cheers: 40,
    nb_bookmarks: 20
  }, {
    id: "graph-4",
    name: "Graph Four",
    description: "This is the description for Graph Four.",
    is_public: false,
    user_access: "admin",
    nb_nodes: 60,
    nb_edges: 90,
    nb_cheers: 0,
    nb_bookmarks: 0
  }, {
    id: "graph-5",
    name:
      "Very very veeeeeeeeeery long name for Graph Five Very very veeeeeeeeeery long name for Graph Five",
    description: "This is the description for Graph Five.",
    is_public: true,
    user_access: "writer",
    nb_nodes: 120,
    nb_edges: 250,
    nb_cheers: 30,
    nb_bookmarks: 15
  }, {
    id: "graph-6",
    name: "Graph Six",
    description:
      "This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six.",
    is_public: false,
    user_access: "reader",
    nb_nodes: 90,
    nb_edges: 140,
    nb_cheers: 0,
    nb_bookmarks: 0
  }];
  // const data: {
  //   id: string;
  //   name: string;
  //   description: string;
  //   is_public: boolean;
  //   user_access: "admin" | "writer" | "reader";
  //   nb_nodes: number;
  //   nb_edges: number;
  //   nb_cheers: number;
  //   nb_bookmarks: number;
  // }[] = [];

  const totalPages = Math.ceil(data.length / itemsPerPage);
  const paginatedData = data.slice(
    currentPage * itemsPerPage,
    currentPage * itemsPerPage + itemsPerPage
  );

  const scrollToElement = () => {
    requestAnimationFrame(() => {
      const element = document.getElementById("cheers-card");
      if (element) {
        const elementPosition = element.getBoundingClientRect().top
          + window.scrollY;
        window.scrollTo({
          top: elementPosition - 16,
          behavior: "smooth"
        });
      }
    });
  };

  return (
    <Card id="cheers-card" className="h-full">
      <CardHeader>
        <CardTitle>Cheers ({data.length})</CardTitle>
        <CardDescription>The list of the public graphs you have cheered</CardDescription>
        <CardAction>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                size="icon-sm"
                onClick={(e) => {
                  e.preventDefault();
                  if (is_expanded) {
                    un_expand();
                  } else {
                    expand();
                  }
                  setTimeout(() => {
                    scrollToElement();
                  }, 300);
                }}
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
        {data.length === 0
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
                <Item key={graph.id} variant="outline" asChild>
                  <a href="#">
                    <ItemContent>
                      <ItemTitle className="line-clamp-1 max-w-80">
                        {graph.name}
                      </ItemTitle>
                      <ItemDescription className="space-x-1">
                        <Badge variant="outline">
                          {graph.nb_nodes} node{graph.nb_nodes !== 1 ? "s" : ""}
                        </Badge>
                        <Badge variant="outline">
                          {graph.nb_edges} edge{graph.nb_edges !== 1 ? "s" : ""}
                        </Badge>
                      </ItemDescription>
                    </ItemContent>
                    <ItemActions>
                      <ArrowUpRightIcon size={16} />
                    </ItemActions>
                  </a>
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
                      : "cursor-pointer"}
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
                      : "cursor-pointer"}
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

export default Cheers;
