import { FormControl, InputLabel, Select, MenuItem, Tooltip } from '@mui/material';
import { forwardRef } from 'react';
import { UseFormRegister } from 'react-hook-form';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { changeLogLevel, type LogLevel } from '@/tauri_cmd';
import { selectLogLevel } from '@/utils/selector';

interface IFormValues {
  logLevel: LogLevel;
}

export const SelectLogLevel = forwardRef<
  HTMLSelectElement,
  { value: LogLevel } & ReturnType<UseFormRegister<IFormValues>>
>(function SelectLogLevel({ onChange, onBlur, name, value }, ref) {
  const { t } = useTranslation();

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
          name={name}
          ref={ref}
          onChange={async (e) => {
            localStorage.setItem(name, e.target.value);
            onChange(e);
            try {
              await changeLogLevel(selectLogLevel(e.target.value));
            } catch (err) {
              toast.error(`${err}`);
            }
          }}
          onBlur={onBlur}
          labelId="log-level-select-label"
          id="log-level-select"
          value={value}
          label="log level"
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
