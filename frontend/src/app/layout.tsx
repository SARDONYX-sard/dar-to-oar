import "@/app/globals.css";
import "@/utils/translation";
import Loading from "@/components/pages/loading";
import React from "react";
import dynamic from "next/dynamic";
import type { Metadata } from "next";
import { Inter } from "next/font/google";

const inter = Inter({ subsets: ["latin"] });

const Menu = dynamic(() => import("@/components/navigation"), {
  loading: () => <Loading />,
  ssr: false,
});
const ThemeProvider = dynamic(() => import("@/components/providers/theme"), {
  loading: () => <Loading />,
  ssr: false,
});

export const metadata: Metadata = {
  title: "DAR to OAR converter",
  description: "Convert from DAR to OAR.",
};

type Props = Readonly<{
  children: React.ReactNode;
}>;
export default function RootLayout({ children }: Props) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <ThemeProvider>
          {children}
          {/* To prevents the conversion button from being hidden because the menu is fixed. */}
          <div style={{ height: "56px" }}></div>
          <Menu />
        </ThemeProvider>
      </body>
    </html>
  );
}
