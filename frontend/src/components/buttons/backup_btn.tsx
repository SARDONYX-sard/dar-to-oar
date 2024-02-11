import FileDownloadIcon from '@mui/icons-material/FileDownload';
import FileOpen from '@mui/icons-material/FileOpen';
import { Button, type ButtonProps, Tooltip } from '@mui/material';
import { type ReactNode } from 'react';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { backup } from '@/tauri_cmd';

type Props = {
  buttonName: ReactNode;
  tooltipTitle: ReactNode;
} & ButtonProps;

export const BackupButton = ({ buttonName, tooltipTitle, ...props }: Readonly<Props>) => (
  <Tooltip title={tooltipTitle}>
    <Button
      sx={{
        height: '4em',
        marginBottom: '8px',
        marginRight: '8px',
        marginTop: '8px',
        minWidth: '120px',
        width: '120px',
      }}
      type="button"
      variant="outlined"
      {...props}
    >
      {buttonName}
    </Button>
  </Tooltip>
);

export const ImportBackupButton = () => {
  const { t } = useTranslation();

  const handleClick = async () => {
    try {
      await backup.import();
    } catch (e) {
      notify.error(`${e}`);
    }
  };

  return (
    <BackupButton
      buttonName={t('backup-import-btn-name')}
      tooltipTitle={t('backup-import-tooltip')}
      onClick={handleClick}
      startIcon={<FileOpen />}
    />
  );
};

export const ExportBackupButton = () => {
  const { t } = useTranslation();

  const handleClick = async () => {
    try {
      if (await backup.export()) {
        notify.success(t('backup-export-success'));
      }
    } catch (e) {
      notify.error(`${e}`);
    }
  };

  return (
    <BackupButton
      buttonName={t('backup-export-btn-name')}
      tooltipTitle={t('backup-export-tooltip')}
      onClick={handleClick}
      startIcon={<FileDownloadIcon />}
    />
  );
};
