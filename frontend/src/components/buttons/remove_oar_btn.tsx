import DeleteIcon from '@mui/icons-material/Delete';
import Button from '@mui/material/Button';
import Tooltip from '@mui/material/Tooltip';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { removeOarDir } from '@/tauri_cmd';

type Props = {
  darPath: string;
  oarPath: string;
};

export const RemoveOarBtn = ({ darPath, oarPath }: Props) => {
  const { t } = useTranslation();

  return (
    <Tooltip title={<p>{t('remove-oar-tooltip')}</p>}>
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
            if (oarPath === '' && darPath === '') {
              toast.error(t('remove-oar-specify-error'));
              return;
            }

            if (oarPath === '') {
              await removeOarDir(darPath);
            } else {
              await removeOarDir(oarPath);
            }
            toast.success(t('remove-oar-success'));
          } catch (_e) {
            toast.error(t('remove-oar-failed'));
          }
        }}
        startIcon={<DeleteIcon />}
      >
        {t('remove-oar-btn')}
      </Button>
    </Tooltip>
  );
};
