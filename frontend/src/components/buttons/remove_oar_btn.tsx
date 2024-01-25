import DeleteIcon from '@mui/icons-material/Delete';
import Button from '@mui/material/Button';
import Tooltip from '@mui/material/Tooltip';
import { useState } from 'react';

import { CircularProgressWithLabel, notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { progressListener, removeOarDir } from '@/tauri_cmd';

type Props = {
  darPath: string;
  oarPath: string;
};

export const RemoveOarBtn = ({ darPath, oarPath }: Props) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleClick = async () => {
    if (oarPath === '' && darPath === '') {
      notify.error(t('remove-oar-specify-error'));
      return;
    }

    await progressListener(
      '/dar2oar/progress/remove-oar',
      async () => {
        if (oarPath === '') {
          await removeOarDir(darPath);
        } else {
          await removeOarDir(oarPath);
        }
      },
      {
        setLoading,
        setProgress,
        success: t('remove-oar-success'),
        error: t('remove-oar-failed'),
      },
    );
  };

  return (
    <Tooltip title={<p>{t('remove-oar-tooltip')}</p>}>
      <Button
        type="button"
        sx={{
          marginTop: '9px',
          width: '100%',
          height: '60%',
        }}
        variant="outlined"
        onClick={handleClick}
        startIcon={loading ? <CircularProgressWithLabel value={progress} /> : <DeleteIcon />}
      >
        {t('remove-oar-btn')}
      </Button>
    </Tooltip>
  );
};
