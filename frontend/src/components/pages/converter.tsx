import { Box } from '@mui/material';

import { ConvertForm } from '@/components/form';
import { Toaster } from '@/components/notifications';
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
      <Toaster />
    </>
  );
}
