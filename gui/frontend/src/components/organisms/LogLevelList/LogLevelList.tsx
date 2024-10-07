import { Tooltip, type SelectChangeEvent } from '@mui/material';
import { useCallback } from 'react';

import { useTranslation } from '@/components/hooks/useTranslation';
import { SelectWithLabel } from '@/components/molecules/SelectWithLabel';
import { NOTIFY } from '@/lib/notify';
import { LOG } from '@/services/api/log';

import { useLogLevelContext } from '../../providers/LogLevelProvider';

export const LogLevelList = () => {
  const { logLevel, setLogLevel } = useLogLevelContext();
  const { t } = useTranslation();

  const handleOnChange = useCallback(
    async ({ target }: SelectChangeEvent) => {
      const newLogLevel = LOG.normalize(target.value);
      setLogLevel(newLogLevel);
      await NOTIFY.asyncTry(async () => await LOG.changeLevel(newLogLevel));
    },
    [setLogLevel],
  );

  const menuItems = [
    { value: 'trace', label: 'Trace' },
    { value: 'debug', label: 'Debug' },
    { value: 'info', label: 'Info' },
    { value: 'warn', label: 'Warning' },
    { value: 'error', label: 'Error' },
  ] as const;

  const tooltip = (
    <>
      <p> {t('log-level-list-tooltip')}</p>
      <p> {t('log-level-list-tooltip2')}</p>
      <p> {t('log-level-list-tooltip3')}</p>
      <p> {t('log-level-list-tooltip4')}</p>
    </>
  );

  return (
    <Tooltip placement='top' title={tooltip}>
      <SelectWithLabel
        label={t('log-level-list-label')}
        menuItems={menuItems}
        onChange={handleOnChange}
        value={logLevel}
      />
    </Tooltip>
  );
};
