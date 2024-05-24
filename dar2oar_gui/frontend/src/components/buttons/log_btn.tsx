import { FileOpen } from '@mui/icons-material';
import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button, type ButtonProps, Tooltip } from '@mui/material';
import { type ReactNode } from 'react';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { openLogDir, openLogFile } from '@/tauri_cmd';

type Props = {
  buttonName: ReactNode;
  tooltipTitle: ReactNode;
} & ButtonProps;

export const LogButton = ({ buttonName, tooltipTitle, ...props }: Props) => (
  <Tooltip title={tooltipTitle}>
    <Button
      sx={{
        marginTop: '9px',
        width: '100%',
        height: '60%',
      }}
      startIcon={<FileOpen />}
      type="button"
      variant="outlined"
      {...props}
    >
      {buttonName}
    </Button>
  </Tooltip>
);

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

  return <LogButton buttonName={t('open-log-btn')} tooltipTitle={t('open-log-tooltip')} onClick={handleClick} />;
};

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
    <LogButton
      buttonName={t('open-log-dir-btn')}
      onClick={handleClick}
      startIcon={<FolderOpenIcon />}
      tooltipTitle={t('open-log-dir-tooltip')}
    />
  );
};
