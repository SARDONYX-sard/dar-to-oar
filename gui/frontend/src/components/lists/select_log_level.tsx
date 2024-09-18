import { FormControl, InputLabel, MenuItem, Select, type SelectChangeEvent, Tooltip } from '@mui/material';
import { forwardRef, useCallback } from 'react';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { type LogLevel, changeLogLevel } from '@/tauri_cmd';
import { selectLogLevel } from '@/utils/selector';

import type { UseFormRegister } from 'react-hook-form';

interface FormValues {
  logLevel: LogLevel;
}

export const SelectLogLevel = forwardRef<
  HTMLSelectElement,
  { value: LogLevel } & ReturnType<UseFormRegister<FormValues>>
>(function SelectLogLevel({ onChange, onBlur, name, value }, ref) {
  const { t } = useTranslation();

  const handleChange = useCallback(
    async (event: SelectChangeEvent<LogLevel>) => {
      localStorage.setItem(name, event.target.value);
      onChange(event);
      try {
        await changeLogLevel(selectLogLevel(event.target.value));
      } catch (err) {
        notify.error(`${err}`);
      }
    },
    [name, onChange],
  );

  return (
    <Tooltip
      placement='top'
      title={
        <>
          <p>{t('log-level-list-tooltip')}</p>
          <p>{t('log-level-list-tooltip2')}</p>
          <p>{t('log-level-list-tooltip3')}</p>
          <p>{t('log-level-list-tooltip4')}</p>
        </>
      }
    >
      <FormControl sx={{ m: 1, minWidth: 110 }} variant='filled'>
        <InputLabel id='log-level-select-label'>{t('log-level-list-label')}</InputLabel>
        <Select
          MenuProps={{ disableScrollLock: true }}
          id='log-level-select'
          label='log level'
          labelId='log-level-select-label'
          name={name}
          onBlur={onBlur}
          onChange={handleChange}
          ref={ref}
          value={value}
        >
          <MenuItem value={'trace'}>Trace</MenuItem>
          <MenuItem value={'debug'}>Debug</MenuItem>
          <MenuItem value={'info'}>Info</MenuItem>
          <MenuItem value={'warn'}>Warning</MenuItem>
          <MenuItem value={'error'}>Error</MenuItem>
        </Select>
      </FormControl>
    </Tooltip>
  );
});
