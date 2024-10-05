import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button } from '@mui/material';

import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { openPath } from '@/services/api/dialog';

type Props = Readonly<{
  path: string;
  setPath: (value: string) => void;
  isDir?: boolean;
}>;

export function PathDialogButton({ path, isDir = false, setPath }: Props) {
  const { t } = useTranslation();

  const handleClick = () => {
    NOTIFY.asyncTry(async () => await openPath(path, { setPath, directory: isDir }));
  };

  return (
    <Button
      onClick={handleClick}
      startIcon={<FolderOpenIcon />}
      sx={{
        marginTop: '9px',
        width: '100%',
        height: '55px',
      }}
      type='button'
      variant='outlined'
    >
      {t('select-btn')}
    </Button>
  );
}
