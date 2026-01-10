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
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput
} from "@/components/ui/input-group";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import {
  BadgeCheckIcon,
  CopyIcon,
  ExpandIcon,
  InfoIcon,
  LogOutIcon,
  PenIcon,
  UserRoundXIcon
} from "lucide-react";

const Settings = () => {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Settings</CardTitle>
        <CardDescription>Manage your account</CardDescription>
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
      <CardContent className="grow space-y-3">
        <Badge variant="secondary">
          <BadgeCheckIcon />
          Membrer since Jan 2023
        </Badge>
        <InputGroup>
          <InputGroupInput id="user-id" placeholder="xxxx-xxxx-xxxx-xxxx-xxxx" readOnly />
          <InputGroupAddon align="block-start">
            <Label htmlFor="user-id" className="text-foreground">
              Id
            </Label>
            <Tooltip>
              <TooltipTrigger asChild>
                <InputGroupButton
                  variant="ghost"
                  aria-label="Copy"
                  className="ml-auto rounded-full"
                  size="icon-xs"
                >
                  <CopyIcon />
                </InputGroupButton>
              </TooltipTrigger>
              <TooltipContent>
                <p>Copy your user ID</p>
              </TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger asChild>
                <InputGroupButton
                  variant="ghost"
                  aria-label="Help"
                  className="rounded-full"
                  size="icon-xs"
                >
                  <InfoIcon />
                </InputGroupButton>
              </TooltipTrigger>
              <TooltipContent>
                <p>Your user ID</p>
              </TooltipContent>
            </Tooltip>
          </InputGroupAddon>
        </InputGroup>
        <InputGroup>
          <InputGroupInput id="user-email" placeholder="napoleon@example.com" readOnly />
          <InputGroupAddon align="block-start">
            <Label htmlFor="user-email" className="text-foreground">
              Email
            </Label>
            <Tooltip>
              <TooltipTrigger asChild>
                <InputGroupButton
                  variant="ghost"
                  aria-label="Help"
                  className="ml-auto rounded-full"
                  size="icon-xs"
                >
                  <InfoIcon />
                </InputGroupButton>
              </TooltipTrigger>
              <TooltipContent>
                <p>You authenticated using this email</p>
              </TooltipContent>
            </Tooltip>
          </InputGroupAddon>
        </InputGroup>
        <InputGroup>
          <InputGroupInput id="user-username" placeholder="napoleon" />
          <InputGroupAddon align="block-start">
            <Label htmlFor="user-username" className="text-foreground">
              Username
            </Label>
            <Tooltip>
              <TooltipTrigger asChild>
                <InputGroupButton
                  variant="ghost"
                  aria-label="Copy"
                  className="ml-auto rounded-full"
                  size="icon-xs"
                >
                  <PenIcon />
                </InputGroupButton>
              </TooltipTrigger>
              <TooltipContent>
                <p>Change your username</p>
              </TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger asChild>
                <InputGroupButton
                  variant="ghost"
                  aria-label="Help"
                  className="rounded-full"
                  size="icon-xs"
                >
                  <InfoIcon />
                </InputGroupButton>
              </TooltipTrigger>
              <TooltipContent>
                <p>Your username</p>
              </TooltipContent>
            </Tooltip>
          </InputGroupAddon>
        </InputGroup>
      </CardContent>
      <CardFooter className="flex flex-col items-start space-y-4">
        <Separator />
        <div className="flex w-full">
          <Button variant="outline" size="sm">
            <LogOutIcon /> Log Out
          </Button>
          <Button
            variant="destructive"
            size="sm"
            className="ml-auto not-hover:bg-white not-hover:border not-hover:border-red-400 not-hover:text-red-400"
          >
            <UserRoundXIcon /> Delete Account
          </Button>
        </div>
      </CardFooter>
    </Card>
  );
};

export default Settings;
