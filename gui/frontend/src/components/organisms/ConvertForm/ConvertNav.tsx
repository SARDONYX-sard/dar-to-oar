'use client';
import { Box, type SxProps, type Theme } from '@mui/material';
import { Controller, useFormContext, useWatch } from 'react-hook-form';
import { ConvertButton } from '@/components/atoms/ConvertButton';
import { LinearWithValueLabel } from '@/components/atoms/LinearWithValueLabel';
import { LogDirButton } from '@/components/molecules/LogDirButton';
import { LogFileButton } from '@/components/molecules/LogFileButton';
import { LogLevelList } from '@/components/organisms/LogLevelList';
import { RemoveOarButton } from '@/components/organisms/RemoveOarButton/RemoveOarButton';
import { UnhideDarButton } from '@/components/organisms/UnhideDarButton';

import type { FormProps } from './ConvertForm';

/** A transparent element that prevents a component in a fixed position from rising up and hiding other components. */
export const ConvertNavPadding = () => <div style={{ paddingBottom: '100px' }} />;

const sx: SxProps<Theme> = {
  position: 'fixed',
  bottom: 50,
  width: '100%',
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  padding: '5px',
  justifyContent: 'space-between',
  backgroundColor: '#121212a4',
};

export const ConvertNav = () => {
  const { control } = useFormContext<FormProps>();
  const { progress, showProgress } = useWatch<FormProps>();

  return (
    <Controller
      control={control}
      name='loading'
      render={({ field: { value } }) => (
        <Box sx={sx}>
          <Box sx={{ width: '100%', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <LogLevelList />
            <LogDirButton />
            <LogFileButton />
            <UnhideDarButton />
            <RemoveOarButton />
            <ConvertButton loading={value} progress={progress ?? 0} />
          </Box>
          {showProgress ? <LinearWithValueLabel progress={progress ?? 0} /> : null}
        </Box>
      )}
    />
  );
};
