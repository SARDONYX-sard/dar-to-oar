'use client';
import { Checkbox, FormControlLabel, Tooltip } from '@mui/material';

import { useStorageState } from '@/components/hooks/useStorageState/useStorageState';
import { useTranslation } from '@/components/hooks/useTranslation';

import type { FormControlLabelProps } from '@mui/material';

type Props = Omit<FormControlLabelProps, 'control' | 'label'>;
const CACHE_KEY = 'run-script';

export const JsAutoRunButton = ({ ...props }: Props) => {
  const { t } = useTranslation();
  const [runScript, setRunScript] = useStorageState(CACHE_KEY, false);

  const title = (
    <>
      {t('custom-js-auto-run-tooltip')}
      <br />
      {t('custom-js-auto-run-tooltip2')}
    </>
  );

  const handleClick = () => {
    if (runScript) {
      window.location.reload();
    }
    setRunScript(!runScript);
  };

  const label = t('custom-js-auto-run-label');

  return (
    <Tooltip title={title}>
      <FormControlLabel
        control={<Checkbox checked={runScript} name={`input-${CACHE_KEY}`} onClick={handleClick} />}
        label={label}
        {...props}
      />
    </Tooltip>
  );
};
