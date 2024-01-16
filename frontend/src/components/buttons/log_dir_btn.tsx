import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button, Tooltip } from '@mui/material';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { openLogDir } from '@/tauri_cmd';

export const LogDirButton = () => {
  const { t } = useTranslation();

  return (
    <Tooltip title={t('open-log-dir-tooltip')}>
      <Button
        sx={{
          marginTop: '9px',
          width: '100%',
          height: '60%',
        }}
        onClick={async () => openLogDir().catch((e) => toast.error(`${e}`))}
        startIcon={<FolderOpenIcon />}
        type="button"
        variant="outlined"
      >
        {t('open-log-dir-btn')}
      </Button>
    </Tooltip>
  );
};
