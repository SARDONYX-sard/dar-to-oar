import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button } from '@mui/material';

import { notify } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { openPath } from '@/tauri_cmd';

type Props = Readonly<{
  path: string;
  setPath: (value: string) => void;
  isDir?: boolean;
}>;

export function SelectPathButton({ path, isDir = false, setPath }: Props) {
  const { t } = useTranslation();
  const handleClick = async () => {
    openPath(path, { setPath, directory: isDir }).catch((e) => notify.error(`${e}`));
  };

  return (
    <Button
      sx={{
        marginTop: '9px',
        width: '100%',
        height: '55px',
      }}
      onClick={handleClick}
      startIcon={<FolderOpenIcon />}
      type="button"
      variant="outlined"
    >
      {t('select-btn')}
    </Button>
  );
}
