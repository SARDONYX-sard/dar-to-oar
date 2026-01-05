'use client';

import type { FormControlLabelProps } from '@mui/material';
import { Checkbox, FormControlLabel, Tooltip } from '@mui/material';
import { useStorageState } from '@/components/hooks/useStorageState/useStorageState';
import { useTranslation } from '@/components/hooks/useTranslation';
import { HIDDEN_CACHE_OBJ } from '@/lib/storage/cacheKeys';

type Props = Omit<FormControlLabelProps, 'control' | 'label'>;

export const JsAutoRunButton = ({ ...props }: Props) => {
  const { t } = useTranslation();
  const [runScript, setRunScript] = useStorageState<boolean>(HIDDEN_CACHE_OBJ.runScript, false);

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
        control={<Checkbox checked={runScript} name={`input-${HIDDEN_CACHE_OBJ.runScript}`} onClick={handleClick} />}
        label={label}
        {...props}
      />
    </Tooltip>
  );
};
