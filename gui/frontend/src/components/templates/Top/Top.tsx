'use client';
import { Box, type SxProps, type Theme } from '@mui/material';

import { useInjectJs } from '@/components/hooks/useInjectJs';
import { ConvertForm } from '@/components/organisms/ConvertForm';

const sx: SxProps<Theme> = {
  display: 'grid',
  placeContent: 'center',
  minHeight: 'calc(100vh - 56px)',
  width: '100%',
};

export const Top = () => {
  useInjectJs();

  return (
    <Box component='main' sx={sx}>
      <ConvertForm />
    </Box>
  );
};
