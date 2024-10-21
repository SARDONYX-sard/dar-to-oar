import { Grid2 as Grid, TextField, type TextFieldProps } from '@mui/material';
import { Controller, useFormContext } from 'react-hook-form';

import { SelectPathButton } from '@/components/molecules/SelectPathButton';
import { getParent } from '@/lib/path';
import { STORAGE } from '@/lib/storage';

import { type FormProps, type PathFormKeys, setPathToStorage } from './ConvertForm';

import type { ReactNode } from 'react';

type Props = {
  name: PathFormKeys;
  label: string;
  placeholder: string;
  helperText: string | ReactNode;
  onChange?: TextFieldProps['onChange'];
};

export const InputPathField = ({ name, label, placeholder, helperText, onChange: onChangeOuter }: Props) => {
  const { control, getValues, setValue } = useFormContext<FormProps>();

  const path = (() => {
    const path = getParent(getValues(name));

    if (path === '') {
      return STORAGE.get(`cached-${name}`) ?? path;
    }

    return path;
  })();

  const handleSetPath = (path: string) => {
    setValue(name, path);
    setPathToStorage(name, path);
  };

  return (
    <Controller
      control={control}
      name={name}
      render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => {
        const handleChange: TextFieldProps['onChange'] = (e) => {
          onChange(e);
          onChangeOuter?.(e);
          setPathToStorage(name, e.target.value);
        };

        return (
          <Grid container={true} spacing={2}>
            <Grid size={10}>
              <TextField
                error={Boolean(error)}
                helperText={helperText}
                label={label}
                margin='dense'
                onBlur={onBlur}
                onChange={handleChange}
                placeholder={placeholder}
                sx={{ width: '100%' }}
                value={value}
                variant='outlined'
              />
            </Grid>

            <Grid size={2}>
              <SelectPathButton isDir={true} path={path} setPath={handleSetPath} />
            </Grid>
          </Grid>
        );
      }}
    />
  );
};
