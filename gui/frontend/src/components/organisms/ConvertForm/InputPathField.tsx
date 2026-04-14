import { Box, TextField } from '@mui/material';
import { Controller, useFormContext } from 'react-hook-form';
import { type FormProps, type PathFormKeys, setPathToStorage } from './ConvertForm';
import { OpenIcon } from './OpenIcon';
import { SelectPathButton } from '@/components/molecules/SelectPathButton';
import { getParent } from '@/lib/path';

import type { SelectVariants, TextFieldProps } from '@mui/material';
import type { ReactNode } from 'react';

type Props = {
  name: PathFormKeys;
  label: string;
  placeholder: string;
  helperText: string | ReactNode;
  onChange?: TextFieldProps['onChange'];
  isDir: boolean;
  setPathHook?: (path: string) => void;
  textFieldVariant?: SelectVariants;
};

export const InputPathField = ({
  name,
  label,
  placeholder,
  helperText,
  onChange: onChangeOuter,
  setPathHook,
  isDir,
  textFieldVariant,
}: Props) => {
  const { control, getValues, setValue } = useFormContext<FormProps>();

  const path = (() => {
    const path = getParent(getValues(name));
    if (path === '') {
      return localStorage.getItem(`cached-${name}`) ?? path;
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
          <Box sx={{ '& > :not(style)': { marginBottom: 2 } }}>
            <Box sx={{ display: 'flex', alignItems: 'flex-start', gap: 1 }}>
              <OpenIcon path={path} />

              <TextField
                error={Boolean(error)}
                helperText={helperText}
                label={label}
                onBlur={onBlur}
                onChange={handleChange}
                placeholder={placeholder}
                sx={{ width: '100%', paddingRight: '10px' }}
                value={value}
                variant={textFieldVariant ?? 'outlined'}
              />
              <SelectPathButton
                isDir={isDir}
                path={path}
                setPath={handleSetPath}
                sx={{ height: '50px', width: '125px' }}
              />
            </Box>
          </Box>
        );
      }}
    />
  );
};
