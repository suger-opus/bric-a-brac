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
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { Label } from "@/components/ui/label";
import { Spinner } from "@/components/ui/spinner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { userService } from "@/lib/api/services/user-service";
import { scrollToElement } from "@/lib/utils";
import { User } from "@/types";
import { BadgeCheckIcon, ExpandIcon, InfoIcon, ShrinkIcon } from "lucide-react";
import { useEffect, useState } from "react";

type SettingsProps = {
  is_expanded: boolean;
  expand: () => void;
  un_expand: () => void;
};

const SettingsCard = ({ is_expanded, expand, un_expand }: SettingsProps) => {
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
      const result = await userService.getCurrent();
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
                Member since {new Date(user.created_at).toDateString()}
              </Badge>
              <InputGroup>
                <InputGroupInput id="user-id" placeholder={user.user_id} readOnly />
                <InputGroupAddon align="block-start">
                  <Label htmlFor="user-id" className="text-foreground">
                    Id
                  </Label>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button
                        variant="ghost"
                        aria-label="Help"
                        className="ml-auto rounded-full"
                        size="icon-sm"
                      >
                        <InfoIcon />
                      </Button>
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
                      <Button
                        variant="ghost"
                        aria-label="Help"
                        className="ml-auto rounded-full"
                        size="icon-sm"
                      >
                        <InfoIcon />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>You authenticated using this email</p>
                    </TooltipContent>
                  </Tooltip>
                </InputGroupAddon>
              </InputGroup>
              <InputGroup>
                <InputGroupInput id="user-username" placeholder={user.username} readOnly />
                <InputGroupAddon align="block-start">
                  <Label htmlFor="user-username" className="text-foreground">
                    Username
                  </Label>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button
                        variant="ghost"
                        aria-label="Help"
                        className="ml-auto rounded-full"
                        size="icon-sm"
                      >
                        <InfoIcon />
                      </Button>
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
      <CardFooter />
    </Card>
  );
};

export default SettingsCard;
