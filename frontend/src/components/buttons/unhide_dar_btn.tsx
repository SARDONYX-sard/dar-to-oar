import VisibilityIcon from '@mui/icons-material/Visibility';
import { Tooltip } from '@mui/material';
import Button from '@mui/material/Button';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { unhideDarDir } from '@/tauri_cmd';

type Props = {
  path: string;
};

export const UnhideDarBtn = ({ path }: Props) => {
  const { t } = useTranslation();
  const handleClick = async () => {
    try {
      if (path === '') {
        toast.error(t('unhide-dar-specify-error'));
        return;
      }

      await unhideDarDir(path);
      toast.success(t('unhide-dar-success'));
    } catch (_e) {
      toast.error(t('unhide-dar-failed'));
    }
  };

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
        onClick={handleClick}
        startIcon={<VisibilityIcon />}
      >
        {t('unhide-dar-btn')}
      </Button>
    </Tooltip>
  );
};
