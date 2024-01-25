import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button, Tooltip } from '@mui/material';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { openLogDir } from '@/tauri_cmd';

export const LogDirButton = () => {
  const { t } = useTranslation();
  const handleClick = async () => {
    try {
      await openLogDir();
    } catch (error) {
      if (error instanceof Error) {
        notify.error(error.message);
      }
    }
  };

  return (
    <Tooltip title={t('open-log-dir-tooltip')}>
      <Button
        sx={{
          marginTop: '9px',
          width: '100%',
          height: '60%',
        }}
        onClick={handleClick}
        startIcon={<FolderOpenIcon />}
        type="button"
        variant="outlined"
      >
        {t('open-log-dir-btn')}
      </Button>
    </Tooltip>
  );
};
