import SmallScreenGate from "@/components/small-screen-gate";
import ThemeProvider from "@/components/theme-provider";
import { Toaster } from "@/components/ui/sonner";
import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"]
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"]
});

export const metadata: Metadata = {
  title: {
    default: "Bric-à-brac",
    template: "%s | Bric-à-brac"
  },
  description: "Interactive 3D knowledge graph visualization",
  icons: {
    icon: "/favicon.svg"
  }
};

const RootLayout = ({ children }: Readonly<{ children: React.ReactNode; }>) => {
  return (
    <html lang="en" suppressHydrationWarning data-scroll-behavior="smooth">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
        suppressHydrationWarning
      >
        <ThemeProvider>
          <SmallScreenGate>
            <main>
              {children}
            </main>
          </SmallScreenGate>
          <Toaster position="bottom-right" />
        </ThemeProvider>
      </body>
    </html>
  );
};

export default RootLayout;
