"use client";

import AccessesCard from "@/components/dashboard/cards/accesses-card";
import SettingsCard from "@/components/dashboard/cards/settings-card";
import { Skeleton } from "@/components/ui/skeleton";
import { userService } from "@/lib/api/services/user-service";
import type { User } from "@/types";
import { useEffect, useState } from "react";

const HomePage = () => {
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    userService.getCurrent().then(setUser).catch(() => {});
  }, []);

  return (
    <div className="max-w-5xl mx-auto px-4 py-8">
      <div className="flex flex-col gap-6">
        <div className="space-y-1">
          {user
            ? (
              <h1 className="text-2xl font-bold tracking-tight">
                Welcome back, {user.username}
              </h1>
            )
            : <Skeleton className="h-8 w-64" />}
          <p className="text-muted-foreground text-sm">
            Manage your knowledge graphs and explore your data.
          </p>
        </div>
        <div className="grid gap-6 lg:grid-cols-[1fr_auto]">
          <AccessesCard />
          <div className="lg:w-80">
            <SettingsCard />
          </div>
        </div>
      </div>
    </div>
  );
};

export default HomePage;
