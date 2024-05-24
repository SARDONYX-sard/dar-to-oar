'use client';

import { CssBaseline } from '@mui/material';
import { ThemeProvider as ThemeProviderInner, createTheme } from '@mui/material/styles';
import useMediaQuery from '@mui/material/useMediaQuery';
import { useMemo } from 'react';

import type { ReactNode } from 'react';

export default function ThemeProvider({ children }: Readonly<{ children: ReactNode }>) {
  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: prefersDarkMode ? 'dark' : 'light',
        },
      }),
    [prefersDarkMode],
  );

  return (
    <ThemeProviderInner theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProviderInner>
  );
}
