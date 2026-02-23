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
import { Spinner } from "@/components/ui/spinner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { SearchGraphDto } from "@/lib/api/dtos";
import { pluralize, scrollToElement } from "@/lib/utils";
import { GraphMetadata } from "@/types";
import { ArrowUpRightIcon, ExpandIcon, PlusIcon, ShrinkIcon } from "lucide-react";
import { useState } from "react";
import * as v from "valibot";

type SearchProps = {
  is_expanded: boolean;
  expand: () => void;
  un_expand: () => void;
};

const SearchCard = ({ is_expanded, expand, un_expand }: SearchProps) => {
  const [searchGraphs, setSearchGraphs] = useState<GraphMetadata[]>([]);

  // Search parameters
  const [searchKeyword, setSearchKeyword] = useState("");
  const [validationError, setValidationError] = useState<string | null>(null);
  const [showResults, setShowResults] = useState(false);
  const [isSearchLoading, setIsSearchLoading] = useState(false);

  // Results pagination
  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 1;
  const totalPages = Math.ceil(searchGraphs.length / itemsPerPage);
  const paginatedData = searchGraphs.slice(
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
      scrollToElement("quick-search-card");
    }, 300);
  };

  const executeSearch = async () => {
    try {
      setIsSearchLoading(true);
      setValidationError(null);
      const validation = v.safeParse(SearchGraphDto, { keyword: searchKeyword });
      if (validation.success) {
        setShowResults(false);
        const results = [] as GraphMetadata[];
        setSearchGraphs(results);
        setCurrentPage(0);
        setShowResults(true);
      } else {
        setShowResults(false);
        setValidationError(validation.issues[0].message);
      }
    } catch (error) {
      setShowResults(false);
      console.error(error);
    } finally {
      setIsSearchLoading(false);
    }
  };

  return (
    <Card id="quick-search-card" className="h-full">
      <CardHeader>
        <CardTitle>Quick Search</CardTitle>
        <CardDescription>Search for a public graph</CardDescription>
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
        <div className="space-y-2">
          <InputGroup>
            <InputGroupInput
              id="search-input"
              value={searchKeyword}
              placeholder="Type to search..."
              onChange={(e) => setSearchKeyword(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  executeSearch();
                }
              }}
            />
            <InputGroupAddon align="inline-end">
              {isSearchLoading
                ? <Spinner />
                : (
                  <InputGroupButton
                    variant="secondary"
                    onClick={executeSearch}
                  >
                    Search
                  </InputGroupButton>
                )}
            </InputGroupAddon>
          </InputGroup>
          <Field>
            {validationError && <FieldError>{validationError}</FieldError>}
            {showResults && (
              <FieldDescription>
                {">"} {searchGraphs.length} matching{" "}
                {pluralize(searchGraphs.length, "graph", "graphs")} found.
              </FieldDescription>
            )}
          </Field>
        </div>
        {showResults && (
          <div className="w-full mt-4 space-y-2">
            {paginatedData.map((graph) => (
              <Item key={graph.graph_id} variant="outline" className="relative">
                <ItemContent>
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
          <Button variant="outline" size="sm" className="mr-auto" disabled>
            <PlusIcon /> Advanced Search
          </Button>
          {showResults && (
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
          )}
        </div>
      </CardFooter>
    </Card>
  );
};

export default SearchCard;
