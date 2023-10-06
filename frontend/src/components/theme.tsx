"use client";

import { CssBaseline } from "@mui/material";
import {
  createTheme,
  ThemeProvider as ThemeProvider_,
} from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";
import type { ReactNode } from "react";
import { useMemo } from "react";

export default function ThemeProvider({ children }: { children: ReactNode }) {
  const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: prefersDarkMode ? "dark" : "light",
        },
      }),
    [prefersDarkMode]
  );

  return (
    <ThemeProvider_ theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProvider_>
  );
}
