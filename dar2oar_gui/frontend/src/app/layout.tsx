import dynamic from 'next/dynamic';
import { Inter } from 'next/font/google';

import Loading from '@/components/pages/loading';
import '@/utils/translation';

import type { Metadata } from 'next';
import type { ReactNode } from 'react';

import '@/app/globals.css';

const inter = Inter({ subsets: ['latin'] });

const Menu = dynamic(() => import('@/components/navigation'), {
  loading: () => <Loading />,
  ssr: false,
});

const ThemeProvider = dynamic(() => import('@/components/providers/theme'), {
  loading: () => <Loading />,
  ssr: false,
});
const SnackBarProvider = dynamic(() => import('@/components/providers/snackbar'), {
  loading: () => <Loading />,
  ssr: false,
});

export const metadata: Metadata = {
  title: 'DAR to OAR converter',
  description: 'Convert from DAR to OAR.',
};

type Props = Readonly<{
  children: ReactNode;
}>;
export default function RootLayout({ children }: Props) {
  return (
    <html lang='en'>
      <body className={inter.className}>
        <ThemeProvider>
          <SnackBarProvider />
          {children}
          {/* To prevents the conversion button from being hidden because the menu is fixed. */}
          <div style={{ height: '56px' }} />
          <Menu />
        </ThemeProvider>
      </body>
    </html>
  );
}
