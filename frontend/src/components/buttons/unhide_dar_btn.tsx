import VisibilityIcon from '@mui/icons-material/Visibility';
import { Tooltip } from '@mui/material';
import Button from '@mui/material/Button';
import { useState } from 'react';
import { toast } from 'react-hot-toast';

import { CircularProgressWithLabel } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { progressListener, unhideDarDir } from '@/tauri_cmd';

type Props = {
  path: string;
};

export const UnhideDarBtn = ({ path }: Props) => {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleClick = async () => {
    if (path === '') {
      toast.error(t('unhide-dar-specify-error'));
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
  };

  return (
    <Tooltip title={<p>{t('unhide-dar-btn-tooltip')}</p>}>
      <Button
        type="button"
        sx={{
          marginTop: '9px',
          width: '100%',
          height: '60%',
        }}
        variant="outlined"
        onClick={handleClick}
        startIcon={loading ? <CircularProgressWithLabel value={progress} /> : <VisibilityIcon />}
      >
        {t('unhide-dar-btn')}
      </Button>
    </Tooltip>
  );
};
