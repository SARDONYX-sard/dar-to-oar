import { FileOpen } from '@mui/icons-material';
import { Button } from '@mui/material';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { openLogFile } from '@/tauri_cmd';

export const LogFileButton = () => {
  const { t } = useTranslation();

  return (
    <Button
      sx={{
        marginTop: '9px',
        width: '100%',
        height: '60%',
      }}
      onClick={async () => openLogFile().catch((e) => toast.error(`${e}`))}
      startIcon={<FileOpen />}
      type="button"
      variant="outlined"
    >
      {t('open-log-btn')}
    </Button>
  );
};
