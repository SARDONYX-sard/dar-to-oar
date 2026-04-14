import OutputIcon from '@mui/icons-material/Output';
import { Tooltip } from '@mui/material';
import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { openPath as open } from '@/services/api/shell';

export const OpenIcon = ({ path }: { path: string }) => {
  const { t } = useTranslation();

  return (
    <Tooltip
      placement='bottom'
      sx={{ color: 'action.active', mr: 1, my: 2, cursor: 'pointer' }}
      title={t('open-directory-tooltip')}
    >
      <OutputIcon onClick={async () => await NOTIFY.asyncTry(async () => await open(path))} />
    </Tooltip>
  );
};
