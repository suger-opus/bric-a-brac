"use client";

import AccessesCard from "@/components/dashboard/cards/accesses-card";

const HomePage = () => {
  return (
    <div className="max-w-5xl mx-auto px-4 py-6">
      <div className="flex flex-col gap-4">
        <AccessesCard />
      </div>
    </div>
  );
};

export default HomePage;
