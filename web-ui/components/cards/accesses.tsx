"use client";

import DataTable from "@/components/data-tables/data-table";
import { columns } from "@/components/data-tables/graph-cols";
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
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from "@/components/ui/empty";
import { Separator } from "@/components/ui/separator";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { ExpandIcon, PlusIcon, VectorSquareIcon } from "lucide-react";

const Accesses = () => {
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
    name: "Very very veeeeeeeeeery long name for Graph Five",
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
      "This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six. This is the description for Graph Six.",
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
        <CardTitle>Your Graphs ({data.length})</CardTitle>
        <CardDescription>List of the graphs you have access to</CardDescription>
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
        {data.length === 0
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
          : <DataTable columns={columns} data={data} />}
      </CardContent>
      {data.length > 0 && (
        <CardFooter className="flex flex-col items-start space-y-4">
          <Separator />
          <Button variant="outline" size="sm">
            <PlusIcon /> Create a new graph
          </Button>
        </CardFooter>
      )}
    </Card>
  );
};

export default Accesses;
