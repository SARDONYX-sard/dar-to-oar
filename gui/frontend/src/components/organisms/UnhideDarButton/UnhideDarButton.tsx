import VisibilityIcon from '@mui/icons-material/Visibility';
import { useCallback, useState } from 'react';
import { useWatch } from 'react-hook-form';

import { ButtonWithToolTip } from '@/components/atoms/ButtonWithToolTip';
import { CircularProgressWithLabel } from '@/components/atoms/CircularProgressWithLabel';
import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { unhideDarDir } from '@/services/api/convert';
import { progressListener } from '@/services/api/event';

import type { FormProps } from '../ConvertForm/ConvertForm';

export const UnhideDarButton = () => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);
  const { src: path } = useWatch<FormProps>();

  const handleClick = useCallback(async () => {
    if (path === undefined || path === '') {
      NOTIFY.error(t('unhide-dar-specify-error'));
      return;
    }

    await progressListener(
      '/dar2oar/progress/unhide-dar',
      async () => {
        await unhideDarDir(path);
      },
      {
        setLoading,
        setProgress,
        success: t('unhide-dar-success'),
        error: `${path}:\n${t('unhide-dar-failed')}`,
      },
    );
  }, [path, t]);

  return (
    <ButtonWithToolTip
      buttonName={t('unhide-dar-btn')}
      icon={loading ? <CircularProgressWithLabel value={progress} /> : <VisibilityIcon />}
      onClick={handleClick}
      tooltipTitle={<p>{t('unhide-dar-btn-tooltip')}</p>}
      variant='contained'
    />
  );
};
