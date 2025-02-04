import { Grid2 as Grid, TextField } from '@mui/material';
import { Controller, useFormContext } from 'react-hook-form';

import { SelectPathButton } from '@/components/molecules/SelectPathButton';
import { getParent } from '@/lib/path';
import { STORAGE } from '@/lib/storage';

import { type FormProps, type PathFormKeys, setPathToStorage } from './ConvertForm';

import type { TextFieldProps } from '@mui/material/TextField';
import type { ReactNode } from 'react';

type Props = {
  name: PathFormKeys;
  label: string;
  placeholder: string;
  helperText: string | ReactNode;
  onChange?: TextFieldProps['onChange'];
  isDir: boolean;
  setPathHook?: (path: string) => void;
};

export const InputPathField = ({
  name,
  label,
  placeholder,
  helperText,
  onChange: onChangeOuter,
  setPathHook,
  isDir,
}: Props) => {
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
    setPathHook?.(path);
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
              <SelectPathButton isDir={isDir} path={path} setPath={handleSetPath} />
            </Grid>
          </Grid>
        );
      }}
    />
  );
};
