"use client";

import AccessesCard from "@/components/dashboard/cards/accesses-card";
import SettingsCard from "@/components/dashboard/cards/settings-card";
import { useState } from "react";

type Expandable = {
  is_expanded: boolean;
};

const HomePage = () => {
  const [accessesCard, setAccessesCard] = useState<Expandable>({
    is_expanded: false
  });
  const [settingsCard, setSettingsCard] = useState<Expandable>({
    is_expanded: false
  });

  const resetView = () => {
    setAccessesCard({ is_expanded: false });
    setSettingsCard({ is_expanded: false });
  };

  return (
    <div className="max-w-5xl mx-auto px-4 my-4">
      <div className="flex flex-col gap-4">
        <div
          className="transition-all duration-300 ease-in-out overflow-hidden"
          style={{
            width: "100%",
            height: accessesCard.is_expanded ? "calc(100vh - 2rem)" : "auto"
          }}
        >
          <AccessesCard
            is_expanded={accessesCard.is_expanded}
            expand={() => {
              resetView();
              setAccessesCard({ is_expanded: true });
            }}
            un_expand={resetView}
          />
        </div>
        <div
          className="transition-all duration-300 ease-in-out overflow-hidden"
          style={{
            width: "100%",
            height: settingsCard.is_expanded ? "calc(100vh - 2rem)" : "auto"
          }}
        >
          <SettingsCard
            is_expanded={settingsCard.is_expanded}
            expand={() => {
              resetView();
              setSettingsCard({ is_expanded: true });
            }}
            un_expand={resetView}
          />
        </div>
      </div>
    </div>
  );
};

export default HomePage;
