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
  const errMsg = t('remove-oar-failed');

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
            if (oarPath === '') {
              await removeOarDir(darPath, errMsg);
            } else {
              await removeOarDir(oarPath, errMsg);
            }
            toast.success(t('remove-oar-success'));
          } catch (e) {
            toast.error(`${e}`);
          }
        }}
        startIcon={<DeleteIcon />}
      >
        {t('remove-oar-btn')}
      </Button>
    </Tooltip>
  );
};
