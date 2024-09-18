import dynamic from 'next/dynamic';
import { Inter } from 'next/font/google';

import Loading from '@/components/pages/loading';
import '@/utils/translation';

import type { Metadata } from 'next';
import type { ReactNode } from 'react';

import '@/app/globals.css';

const inter = Inter({ subsets: ['latin'] });

const ClientLayout = dynamic(() => import('@/app/client_layout'), {
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
      {/* biome-ignore lint/suspicious/noReactSpecificProps: <explanation> */}
      <body className={inter.className}>
        <ClientLayout>
          {children}
          {/* To prevents the conversion button from being hidden because the menu is fixed. */}
          <div style={{ height: '56px' }} />
        </ClientLayout>
      </body>
    </html>
  );
}
