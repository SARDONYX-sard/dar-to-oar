import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import type { ButtonProps } from '@mui/material';
import { ButtonWithToolTip } from '@/components/atoms/ButtonWithToolTip';
import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { LOG } from '@/services/api/log';

type Props = ButtonProps;

export const LogDirButton = ({ ...props }: Props) => {
  const { t } = useTranslation();
  const handleClick = () => NOTIFY.asyncTry(async () => await LOG.openDir());

  return (
    <ButtonWithToolTip
      {...props}
      buttonName={t('open-log-dir-btn')}
      icon={<FolderOpenIcon />}
      onClick={handleClick}
      tooltipTitle={t('open-log-dir-tooltip')}
    />
  );
};
