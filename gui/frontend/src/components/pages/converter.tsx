'use client';

import { Box } from '@mui/material';

import { ConvertForm } from '@/components/form';
import { useDynStyle, useInjectScript, useLocale } from '@/hooks';

export default function Converter() {
  useDynStyle();
  useInjectScript();
  useLocale();

  return (
    <Box
      component='main'
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
  );
}
