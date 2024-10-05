import DeleteIcon from '@mui/icons-material/Delete';
import Button from '@mui/material/Button';
import Tooltip from '@mui/material/Tooltip';
import { useCallback, useState } from 'react';

import { CircularProgressWithLabel } from '@/components/atoms/CircularProgressWithLabel';
import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { removeOarDir } from '@/services/api/convert';
import { progressListener } from '@/services/api/event';

type Props = {
  darPath: string;
  oarPath: string;
};

export const RemoveOarButton = ({ darPath, oarPath }: Props) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleClick = useCallback(async () => {
    if (oarPath === '' && darPath === '') {
      NOTIFY.error(t('remove-oar-specify-error'));
      return;
    }

    await progressListener(
      '/dar2oar/progress/remove-oar',
      async () => {
        const path = oarPath === '' ? darPath : oarPath;
        await removeOarDir(path);
      },
      {
        setLoading,
        setProgress,
        success: t('remove-oar-success'),
        error: t('remove-oar-failed'),
      },
    );
  }, [darPath, oarPath, t]);

  return (
    <Tooltip title={<p>{t('remove-oar-tooltip')}</p>}>
      <Button
        onClick={handleClick}
        startIcon={loading ? <CircularProgressWithLabel value={progress} /> : <DeleteIcon />}
        sx={{
          marginTop: '9px',
          width: '100%',
          height: '60%',
        }}
        type='button'
        variant='outlined'
      >
        {t('remove-oar-btn')}
      </Button>
    </Tooltip>
  );
};
