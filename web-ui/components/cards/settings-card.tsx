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
import { Spinner } from "@/components/ui/spinner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { ApiProvider } from "@/lib/api/provider";
import { scrollToElement } from "@/lib/utils";
import { User } from "@/types";
import {
  BadgeCheckIcon,
  CopyIcon,
  ExpandIcon,
  InfoIcon,
  LogOutIcon,
  PenIcon,
  ShrinkIcon,
  UserRoundXIcon
} from "lucide-react";
import { useEffect, useState } from "react";

type SettingsProps = {
  is_expanded: boolean;
  expand: () => void;
  un_expand: () => void;
};

const SettingsCard = ({ is_expanded, expand, un_expand }: SettingsProps) => {
  const { userService } = ApiProvider;
  const [user, setUser] = useState<User | null>(null);
  const [isUserLoading, setIsUserLoading] = useState(true);

  const handleExpand = () => {
    if (is_expanded) {
      un_expand();
    } else {
      expand();
    }
    setTimeout(() => {
      scrollToElement("settings-card");
    }, 300);
  };

  const getUser = async () => {
    try {
      setIsUserLoading(true);
      const result = await userService.get();
      setUser(result);
    } catch (error) {
      console.error("Error during getUser:", error);
    } finally {
      setIsUserLoading(false);
    }
  };

  useEffect(() => {
    getUser();
  }, []);

  return (
    <Card id="settings-card" className="h-full">
      <CardHeader>
        <CardTitle>Settings</CardTitle>
        <CardDescription>Manage your account</CardDescription>
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
      <CardContent className="grow space-y-3">
        {!user || isUserLoading
          ? (
            <div className="h-full flex items-center justify-center">
              <Spinner />
            </div>
          )
          : (
            <>
              <Badge variant="secondary">
                <BadgeCheckIcon />
                Member since {user.created_at.toDateString()}
              </Badge>
              <InputGroup>
                <InputGroupInput id="user-id" placeholder={user.user_id} readOnly />
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
                <InputGroupInput id="user-email" placeholder={user.email} readOnly />
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
                <InputGroupInput id="user-username" placeholder={user.username} />
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
            </>
          )}
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

export default SettingsCard;
