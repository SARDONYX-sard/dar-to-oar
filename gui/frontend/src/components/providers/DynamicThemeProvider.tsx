import { CssBaseline, createTheme, ThemeProvider } from '@mui/material';
import { type ReactNode, useMemo } from 'react';

type Props = {
  children: ReactNode;
};

export const DynamicThemeProvider = ({ children }: Props) => {
  const theme = useMemo(
    () =>
      createTheme({
        cssVariables: true,
        palette: {
          mode: 'dark',
        },
      }),
    [],
  );

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProvider>
  );
};
