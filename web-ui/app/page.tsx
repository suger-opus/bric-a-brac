"use client";

import AccessesCard from "@/components/dashboard/cards/accesses-card";
import BookmarksCard from "@/components/dashboard/cards/bookmarks-card";
import CheersCard from "@/components/dashboard/cards/cheers-card";
import SearchCard from "@/components/dashboard/cards/search-card";
import SettingsCard from "@/components/dashboard/cards/settings-card";
import { useState } from "react";

type Expandable = {
  is_expanded: boolean;
};

const HomePage = () => {
  const [searchCard, setSearchCard] = useState<Expandable>({
    is_expanded: false
  });
  const [bookmarksCard, setBookmarksCard] = useState<Expandable>({
    is_expanded: false
  });
  const [accessesCard, setAccessesCard] = useState<Expandable>({
    is_expanded: false
  });
  const [cheersCard, setCheersCard] = useState<Expandable>({
    is_expanded: false
  });
  const [settingsCard, setSettingsCard] = useState<Expandable>({
    is_expanded: false
  });

  const resetView = () => {
    setSearchCard({ is_expanded: false });
    setBookmarksCard({ is_expanded: false });
    setAccessesCard({ is_expanded: false });
    setCheersCard({ is_expanded: false });
    setSettingsCard({ is_expanded: false });
  };

  return (
    <div className="mx-40 my-4">
      <div className="flex flex-wrap gap-4">
        <div
          className="transition-all duration-300 ease-in-out overflow-hidden"
          style={{
            width: searchCard.is_expanded
              ? "100%"
              : bookmarksCard.is_expanded
              ? "100%"
              : "calc(50% - 0.5rem)",
            height: searchCard.is_expanded
              ? "calc(100vh - 2rem)"
              : bookmarksCard.is_expanded
              ? "22rem"
              : "auto"
          }}
        >
          <SearchCard
            is_expanded={searchCard.is_expanded}
            expand={() => {
              resetView();
              setSearchCard({ is_expanded: true });
            }}
            un_expand={resetView}
          />
        </div>
        <div
          className="transition-all duration-300 ease-in-out overflow-hidden"
          style={{
            width: bookmarksCard.is_expanded
              ? "100%"
              : searchCard.is_expanded
              ? "100%"
              : "calc(50% - 0.5rem)",
            height: bookmarksCard.is_expanded
              ? "calc(100vh - 2rem)"
              : searchCard.is_expanded
              ? "22rem"
              : "auto"
          }}
        >
          <BookmarksCard
            is_expanded={bookmarksCard.is_expanded}
            expand={() => {
              resetView();
              setBookmarksCard({ is_expanded: true });
            }}
            un_expand={resetView}
          />
        </div>
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
            width: cheersCard.is_expanded
              ? "100%"
              : settingsCard.is_expanded
              ? "100%"
              : "calc(50% - 0.5rem)",
            height: cheersCard.is_expanded
              ? "calc(100vh - 2rem)"
              : settingsCard.is_expanded
              ? "22rem"
              : "auto"
          }}
        >
          <CheersCard
            is_expanded={cheersCard.is_expanded}
            expand={() => {
              resetView();
              setCheersCard({ is_expanded: true });
            }}
            un_expand={resetView}
          />
        </div>
        <div
          className="transition-all duration-300 ease-in-out overflow-hidden"
          style={{
            width: settingsCard.is_expanded
              ? "100%"
              : cheersCard.is_expanded
              ? "100%"
              : "calc(50% - 0.5rem)",
            height: settingsCard.is_expanded
              ? "calc(100vh - 2rem)"
              : cheersCard.is_expanded
              ? "22rem"
              : "auto"
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
