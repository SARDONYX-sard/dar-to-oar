import { FileOpen } from '@mui/icons-material';
import { Button, Tooltip } from '@mui/material';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { openLogFile } from '@/tauri_cmd';

export const LogFileButton = () => {
  const { t } = useTranslation();

  const handleClick = async () => {
    try {
      await openLogFile();
    } catch (error) {
      if (error instanceof Error) {
        notify.error(error.message);
      }
    }
  };

  return (
    <Tooltip title={t('open-log-tooltip')}>
      <Button
        sx={{
          marginTop: '9px',
          width: '100%',
          height: '60%',
        }}
        onClick={handleClick}
        startIcon={<FileOpen />}
        type="button"
        variant="outlined"
      >
        {t('open-log-btn')}
      </Button>
    </Tooltip>
  );
};
