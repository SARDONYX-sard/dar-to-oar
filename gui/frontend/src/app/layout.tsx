import dynamic from 'next/dynamic';

export { metadata } from '@/components/meta/meta';
import Loading from '@/components/templates/Loading';
import '@fontsource/inter'

import type { ReactNode } from 'react';

import '@/app/globals.css';

const ClientLayout = dynamic(() => import('@/components/layout/ClientLayout'), {
  loading: () => <Loading />,
  ssr: false,
});

type Props = Readonly<{
  children: ReactNode;
}>;
export default function RootLayout({ children }: Props) {
  return (
    <html lang='en'>
      <body>
        <ClientLayout>{children}</ClientLayout>
      </body>
    </html>
  );
}
