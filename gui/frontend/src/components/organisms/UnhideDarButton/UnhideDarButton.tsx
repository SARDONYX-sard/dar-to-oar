import VisibilityIcon from '@mui/icons-material/Visibility';
import { Tooltip } from '@mui/material';
import Button from '@mui/material/Button';
import { useCallback, useState } from 'react';

import { CircularProgressWithLabel } from '@/components/atoms/CircularProgressWithLabel';
import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { unhideDarDir } from '@/services/api/convert';
import { progressListener } from '@/services/api/event';

type Props = {
  path: string;
};

export const UnhideDarButton = ({ path }: Props) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleClick = useCallback(async () => {
    if (path === '') {
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
        error: t('unhide-dar-failed'),
      },
    );
  }, [path, t]);

  return (
    <Tooltip title={<p>{t('unhide-dar-btn-tooltip')}</p>}>
      <Button
        onClick={handleClick}
        startIcon={loading ? <CircularProgressWithLabel value={progress} /> : <VisibilityIcon />}
        sx={{
          marginTop: '9px',
          minHeight: '50px',
          width: '100%',
        }}
        type='button'
        variant='outlined'
      >
        {t('unhide-dar-btn')}
      </Button>
    </Tooltip>
  );
};
