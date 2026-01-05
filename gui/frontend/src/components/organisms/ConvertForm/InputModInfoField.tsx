import { Grid, TextField } from '@mui/material';
import type { ReactNode } from 'react';
import { Controller, useFormContext } from 'react-hook-form';

import type { FormProps } from './ConvertForm';
import { setPathToStorage } from './ConvertForm';

type Props = {
  name: 'modName' | 'modAuthor';
  label: string;
  placeholder: string;
  helperText: ReactNode;
};

export const InputModInfoField = ({ name, ...props }: Props) => {
  const { control } = useFormContext<FormProps>();

  return (
    <Grid size={'grow'}>
      <Controller
        control={control}
        name={name}
        render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
          <TextField
            error={Boolean(error)}
            margin='dense'
            onBlur={onBlur}
            onChange={(e) => {
              onChange(e);
              setPathToStorage(name, e.target.value);
            }}
            sx={{ width: '100%' }}
            value={value}
            variant='outlined'
            {...props}
          />
        )}
      />
    </Grid>
  );
};
