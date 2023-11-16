import VisibilityIcon from '@mui/icons-material/Visibility';
import { Tooltip } from '@mui/material';
import Button from '@mui/material/Button';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { restoreDarDir } from '@/tauri_cmd';

type Props = {
  path: string;
};

export const UnhideDarBtn = ({ path }: Props) => {
  const { t } = useTranslation();

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
        onClick={async () => {
          try {
            toast.success(await restoreDarDir(path, t('unhide-dar-failed')));
          } catch (err) {
            toast.error(`${err}`);
          }
        }}
        startIcon={<VisibilityIcon />}
      >
        {t('unhide-dar-btn')}
      </Button>
    </Tooltip>
  );
};
