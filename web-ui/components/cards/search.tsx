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
import { Field, FieldDescription, FieldError } from "@/components/ui/field";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput
} from "@/components/ui/input-group";
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
import { ArrowUpRightIcon, ExpandIcon, PlusIcon } from "lucide-react";
import { useState } from "react";

const Search = () => {
  const [search, setSearch] = useState("");

  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 1;

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

  return (
    <Card>
      <CardHeader>
        <CardTitle>Quick Search</CardTitle>
        <CardDescription>Search for a public graph</CardDescription>
        <CardAction>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button size="icon-sm">
                <ExpandIcon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              Expand this view
            </TooltipContent>
          </Tooltip>
        </CardAction>
      </CardHeader>
      <CardContent className="grow">
        <div className="space-y-2">
          <InputGroup>
            <InputGroupInput
              placeholder="Type to search..."
              onChange={(e) => setSearch(e.target.value)}
            />
            <InputGroupAddon align="inline-end">
              <InputGroupButton variant="secondary">Search</InputGroupButton>
            </InputGroupAddon>
          </InputGroup>
          <Field>
            {search.length > 0 && search.length < 5 && (
              <FieldError>Keyword must be at least 5 characters long.</FieldError>
            )}
            {search.length >= 5 && (
              <FieldDescription>
                {">"} {data.length} matching graph{data.length !== 1 ? "s" : ""} found.
              </FieldDescription>
            )}
          </Field>
        </div>
        {search.length >= 5 && (
          <div className="w-full mt-4 space-y-2">
            {paginatedData.map((graph) => (
              <Item key={graph.id} variant="outline" className="relative">
                <ItemContent>
                  <ItemTitle className="line-clamp-1 max-w-50">
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
                  <ItemDescription className="grow mt-1 line-clamp-5">
                    {graph.description}
                  </ItemDescription>
                </ItemContent>
                <ItemActions className="absolute top-2 right-2">
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button variant="ghost" size="icon-sm">
                        <ArrowUpRightIcon />
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
      <CardFooter className="flex flex-col items-start space-y-4">
        <Separator />
        <div className="flex w-full">
          <Button variant="outline" size="sm" className="mr-auto">
            <PlusIcon /> Advanced Search
          </Button>
          {search.length >= 5 && data.length > 0 && (
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
          )}
        </div>
      </CardFooter>
    </Card>
  );
};

export default Search;
