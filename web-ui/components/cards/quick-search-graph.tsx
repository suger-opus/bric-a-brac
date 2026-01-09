"use client";

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
import { Field, FieldError, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { ExpandIcon, PlusIcon, SearchIcon } from "lucide-react";

const QuickSearchGraph = () => {
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
        <Field>
          <FieldLabel htmlFor="input-id">Keyword</FieldLabel>
          <div className="flex w-full items-center gap-2">
            <Input id="search-keyword" type="text" placeholder="Napoleon" />
            <Tooltip>
              <TooltipTrigger asChild>
                <Button>
                  <SearchIcon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                Search
              </TooltipContent>
            </Tooltip>
          </div>
          <FieldError>Keyword must be at least 5 characters long.</FieldError>
        </Field>
      </CardContent>
      <CardFooter className="flex flex-col items-start space-y-4">
        <Separator />
        <Button variant="outline">
          <PlusIcon /> Advanced Search
        </Button>
      </CardFooter>
    </Card>
  );
};

export default QuickSearchGraph;
