import { Box, Checkbox, FormControlLabel, Tooltip } from '@mui/material';
import type { ReactNode } from 'react';
import { Controller, useFormContext } from 'react-hook-form';
import { STORAGE } from '@/lib/storage';
import type { FormProps } from './ConvertForm';

type PickBooleans<T> = {
  [K in keyof T as T[K] extends boolean ? K : never]: T[K];
};

type BoolFormProps = PickBooleans<FormProps>;

type CheckboxFieldProps = {
  name: Exclude<keyof BoolFormProps, 'loading'>;
  label: string;
  icon?: ReactNode;
  tooltipText: ReactNode;
};

export const CheckboxField = ({ name, label, tooltipText, icon }: CheckboxFieldProps) => {
  const { control } = useFormContext<FormProps>();

  return (
    <Controller
      control={control}
      name={name}
      render={({ field: { value, onChange } }) => (
        <Tooltip title={tooltipText}>
          <FormControlLabel
            control={
              <Checkbox
                aria-label={label}
                checked={value}
                onClick={() => {
                  const newValue = !value;
                  STORAGE.set(name, `${newValue}`);
                  onChange(newValue);
                }}
              />
            }
            label={
              <Box component='div' sx={{ display: 'flex' }}>
                {icon}
                {label}
              </Box>
            }
          />
        </Tooltip>
      )}
    />
  );
};
