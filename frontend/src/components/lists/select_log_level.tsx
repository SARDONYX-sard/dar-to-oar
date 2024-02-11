import { FormControl, InputLabel, Select, MenuItem, Tooltip, SelectChangeEvent } from '@mui/material';
import { forwardRef, useCallback } from 'react';
import { UseFormRegister } from 'react-hook-form';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { changeLogLevel, type LogLevel } from '@/tauri_cmd';
import { selectLogLevel } from '@/utils/selector';

interface IFormValues {
  logLevel: LogLevel;
}

export const SelectLogLevel = forwardRef<
  HTMLSelectElement,
  { value: LogLevel } & ReturnType<UseFormRegister<IFormValues>>
>(function SelectLogLevel({ name, onChange, ...props }, ref) {
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
      title={
        <>
          <p>{t('log-level-list-tooltip')}</p>
          <p>{t('log-level-list-tooltip2')}</p>
          <p>{t('log-level-list-tooltip3')}</p>
          <p>{t('log-level-list-tooltip4')}</p>
        </>
      }
      placement="top"
    >
      <FormControl variant="filled" sx={{ m: 1, minWidth: 110 }}>
        <InputLabel id="log-level-select-label">{t('log-level-list-label')}</InputLabel>
        <Select
          id="log-level-select"
          inputProps={{ MenuProps: { disableScrollLock: true } }}
          label="log level"
          labelId="log-level-select-label"
          onChange={handleChange}
          {...props}
          ref={ref}
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
