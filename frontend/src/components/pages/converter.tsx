'use client';

import { Box } from '@mui/material';
import { Toaster } from 'react-hot-toast';

import { ConvertForm } from '@/components/form';
import { useDynStyle, useInjectScript, useLocale, useToastLimit } from '@/hooks';

export default function Converter() {
  useDynStyle();
  useInjectScript();
  useLocale();
  useToastLimit(1);

  return (
    <>
      <Box
        component="main"
        sx={{
          display: 'grid',
          minHeight: 'calc(100vh - 56px)',
          placeContent: 'center',
          placeItems: 'center',
          width: '100%',
        }}
      >
        <ConvertForm />
      </Box>
      <Toaster
        position="bottom-right"
        reverseOrder={false}
        toastOptions={{
          style: {
            color: '#fff',
            background: '#1a1919e1',
          },
        }}
      />
    </>
  );
}
