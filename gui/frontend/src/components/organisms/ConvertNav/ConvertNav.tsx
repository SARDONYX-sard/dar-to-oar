'use client';
import { Box, type SxProps, type Theme } from '@mui/material';
import { Controller, useFormContext, useWatch } from 'react-hook-form';

import { ConvertButton } from '@/components/atoms/ConvertButton';
import { LogDirButton } from '@/components/molecules/LogDirButton';
import { LogFileButton } from '@/components/molecules/LogFileButton';
import { LogLevelList } from '@/components/organisms/LogLevelList';
import { RemoveOarButton } from '@/components/organisms/RemoveOarButton/RemoveOarButton';
import { UnhideDarButton } from '@/components/organisms/UnhideDarButton';

import type { FormProps } from '../ConvertForm/ConvertForm';

const sx: SxProps<Theme> = {
  position: 'fixed',
  bottom: 50,
  width: '100%',
  display: 'flex',
  alignItems: 'center',
  padding: '10px',
  justifyContent: 'space-between',
  backgroundColor: '#252525d8',
};

/** A transparent element that prevents a component in a fixed position from rising up and hiding other components. */
export const ConvertNavPadding = () => <div style={{ height: '100px' }} />;
export const ConvertNav = () => {
  const { control } = useFormContext<FormProps>();
  const { progress } = useWatch<FormProps>();

  return (
    <Controller
      control={control}
      name='loading'
      render={({ field: { value } }) => (
        <Box gap={1} sx={sx}>
          <LogLevelList />
          <LogDirButton />
          <LogFileButton />
          <UnhideDarButton />
          <RemoveOarButton />
          <ConvertButton loading={value} progress={progress ?? 0} />
        </Box>
      )}
    />
  );
};
