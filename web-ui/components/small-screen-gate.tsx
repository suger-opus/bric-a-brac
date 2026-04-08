"use client";

import { useEffect, useState } from "react";

const DESKTOP_BREAKPOINT = 1024;

const SmallScreenGate = ({ children }: { children: React.ReactNode; }) => {
  const [isSmallScreen, setIsSmallScreen] = useState<boolean | null>(null);

  useEffect(() => {
    const check = () => setIsSmallScreen(window.innerWidth < DESKTOP_BREAKPOINT);
    check();
    window.addEventListener("resize", check);
    return () => window.removeEventListener("resize", check);
  }, []);

  if (isSmallScreen === null) { return null; }

  if (isSmallScreen) {
    return (
      <div className="flex flex-col items-center justify-center h-screen px-8 text-center gap-6">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth={1.5}
          strokeLinecap="round"
          strokeLinejoin="round"
          className="w-16 h-16 text-muted-foreground"
        >
          <rect width="20" height="14" x="2" y="3" rx="2" />
          <line x1="8" x2="16" y1="21" y2="21" />
          <line x1="12" x2="12" y1="17" y2="21" />
        </svg>
        <div>
          <h1 className="text-xl font-semibold mb-2">Desktop required</h1>
          <p className="text-muted-foreground text-sm max-w-sm">
            Bric-à-brac is designed for desktop. Please use a larger screen.
          </p>
        </div>
      </div>
    );
  }

  return <>{children}</>;
};

export default SmallScreenGate;
