export { metadata } from '@/components/meta/meta';

import type { ReactNode } from 'react';
import ClientLayout from '@/components/layout/ClientLayout';

import '@/app/globals.css';

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
