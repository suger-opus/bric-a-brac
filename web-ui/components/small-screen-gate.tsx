"use client";

import { MonitorIcon } from "lucide-react";
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
        <MonitorIcon className="w-16 h-16 text-muted-foreground" strokeWidth={1.5} />
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
