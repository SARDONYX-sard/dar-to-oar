import { FileOpen } from '@mui/icons-material';
import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button, type ButtonProps, Tooltip } from '@mui/material';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { openLogDir, openLogFile } from '@/tauri_cmd';

import type { ReactNode } from 'react';

type Props = {
  buttonName: ReactNode;
  tooltipTitle: ReactNode;
} & ButtonProps;

export const LogButton = ({ buttonName, tooltipTitle, ...props }: Props) => (
  <Tooltip title={tooltipTitle}>
    <Button
      startIcon={<FileOpen />}
      sx={{
        marginTop: '9px',
        width: '100%',
        height: '60%',
      }}
      type='button'
      variant='outlined'
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
      notify.error(`${error}`);
    }
  };

  return <LogButton buttonName={t('open-log-btn')} onClick={handleClick} tooltipTitle={t('open-log-tooltip')} />;
};

export const LogDirButton = () => {
  const { t } = useTranslation();

  const handleClick = async () => {
    try {
      await openLogDir();
    } catch (error) {
      notify.error(`${error}`);
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
