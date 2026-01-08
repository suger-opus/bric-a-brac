"use client";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from "@/components/ui/card";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious
} from "@/components/ui/carousel";
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from "@/components/ui/empty";
import { Toggle } from "@/components/ui/toggle";
import { ArrowUpRightIcon, BookmarkIcon, ExpandIcon } from "lucide-react";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";

const BookmarkedGraphsList = () => {
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

  return (
    <Card>
      <CardHeader>
        <CardTitle>Bookmarked Graphs ({data.length})</CardTitle>
        <CardDescription>The list of graphs you have bookmarked</CardDescription>
        <CardAction>
          <Tooltip>
            <TooltipTrigger>
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
        {data.length === 0
          ? (
            <Empty>
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
            <div className="mx-12">
              <Carousel className="max-w-full">
                <CarouselContent>
                  {data.map((graph) => (
                    <CarouselItem key={graph.id} className="min-h-full">
                      <Card className="min-h-full">
                        <CardHeader>
                          <CardTitle>
                            <div className="line-clamp-2 max-w-42">
                              {graph.name}
                            </div>
                          </CardTitle>
                          <CardDescription className="space-x-1">
                            <Badge variant="outline">
                              {graph.nb_nodes} node{graph.nb_nodes !== 1 ? "s" : ""}
                            </Badge>
                            <Badge variant="outline">
                              {graph.nb_edges} edge{graph.nb_edges !== 1 ? "s" : ""}
                            </Badge>
                          </CardDescription>
                          <CardAction className="space-x-1">
                            <Tooltip>
                              <Toggle
                                aria-label="Toggle bookmark"
                                size="sm"
                                variant="outline"
                                defaultPressed={true}
                                className="data-[state=on]:bg-transparent data-[state=on]:*:[svg]:fill-black data-[state=on]:*:[svg]:stroke-black"
                                asChild
                              >
                                <TooltipTrigger>
                                  <BookmarkIcon />
                                </TooltipTrigger>
                              </Toggle>
                              <TooltipContent>
                                Un-bookmarked this graph
                              </TooltipContent>
                            </Tooltip>
                            <Tooltip>
                              <TooltipTrigger>
                                <Button variant="outline" size="icon-sm">
                                  <ArrowUpRightIcon />
                                </Button>
                              </TooltipTrigger>
                              <TooltipContent>
                                Open this graph
                              </TooltipContent>
                            </Tooltip>
                          </CardAction>
                        </CardHeader>
                        <CardContent>
                          <div className="line-clamp-5">
                            {graph.description}
                          </div>
                        </CardContent>
                      </Card>
                    </CarouselItem>
                  ))}
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
              </Carousel>
            </div>
          )}
      </CardContent>
    </Card>
  );
};

export default BookmarkedGraphsList;
