"use client";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { userService } from "@/lib/api/services/user-service";
import { User } from "@/types";
import { BadgeCheckIcon, MoonIcon, SunIcon } from "lucide-react";
import { useTheme } from "next-themes";
import { useEffect, useState } from "react";
import { toast } from "sonner";

const SettingsCard = () => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const { theme, setTheme } = useTheme();

  useEffect(() => {
    userService.getCurrent()
      .then(setUser)
      .catch(() => toast.error("Could not load your profile"))
      .finally(() => setIsLoading(false));
  }, []);

  return (
    <Card className="h-full">
      <CardHeader>
        <CardTitle>Settings</CardTitle>
        <CardDescription>Your account and preferences</CardDescription>
      </CardHeader>
      <CardContent className="grow flex flex-col justify-between space-y-4">
        <div className="space-y-4">
          {isLoading
            ? (
              <div className="space-y-3">
                <Skeleton className="h-5 w-48" />
                <Skeleton className="h-4 w-32" />
                <Skeleton className="h-4 w-56" />
                <Skeleton className="h-4 w-40" />
              </div>
            )
            : user && (
              <>
                <Badge variant="secondary">
                  <BadgeCheckIcon className="h-3.5 w-3.5" />
                  Member since {new Date(user.created_at).toLocaleDateString(undefined, {
                    month: "long",
                    year: "numeric"
                  })}
                </Badge>
                <div className="grid gap-1.5 text-sm">
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Username</span>
                    <span className="font-medium">{user.username}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Email</span>
                    <span className="font-medium">{user.email}</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-muted-foreground">User ID</span>
                    <span className="font-mono text-[10px] text-muted-foreground">
                      {user.user_id}
                    </span>
                  </div>
                </div>
              </>
            )}
        </div>

        <div>
          <Separator className="mb-4" />
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium">Appearance</p>
              <p className="text-xs text-muted-foreground">Switch between light and dark mode</p>
            </div>
            <div className="flex items-center gap-1">
              <Button
                variant={theme === "light" ? "default" : "outline"}
                size="icon-sm"
                onClick={() => setTheme("light")}
                aria-label="Light mode"
              >
                <SunIcon className="h-4 w-4" />
              </Button>
              <Button
                variant={theme === "dark" ? "default" : "outline"}
                size="icon-sm"
                onClick={() => setTheme("dark")}
                aria-label="Dark mode"
              >
                <MoonIcon className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default SettingsCard;
