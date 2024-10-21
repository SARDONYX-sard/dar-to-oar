import DeleteIcon from '@mui/icons-material/Delete';
import { useCallback, useState } from 'react';
import { useWatch } from 'react-hook-form';

import { ButtonWithToolTip } from '@/components/atoms/ButtonWithToolTip';
import { CircularProgressWithLabel } from '@/components/atoms/CircularProgressWithLabel';
import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { removeOarDir } from '@/services/api/convert';
import { progressListener } from '@/services/api/event';

import type { FormProps } from '../ConvertForm/ConvertForm';

export const RemoveOarButton = () => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);
  const { src: darPath, dst: oarPath } = useWatch<FormProps>();

  const handleClick = useCallback(async () => {
    if ((oarPath === undefined || oarPath === '') && (darPath === undefined || darPath === '')) {
      NOTIFY.error(t('remove-oar-specify-error'));
      return;
    }
    const path = oarPath === '' ? darPath : oarPath;

    await progressListener(
      '/dar2oar/progress/remove-oar',
      async () => {
        await removeOarDir(path ?? '');
      },
      {
        setLoading,
        setProgress,
        success: t('remove-oar-success'),
        error: `${path}:\n${t('remove-oar-failed')}`,
      },
    );
  }, [darPath, oarPath, t]);

  return (
    <ButtonWithToolTip
      buttonName={t('remove-oar-btn')}
      icon={loading ? <CircularProgressWithLabel value={progress} /> : <DeleteIcon />}
      onClick={handleClick}
      tooltipTitle={<p>{t('remove-oar-tooltip')}</p>}
      variant='contained'
    />
  );
};
