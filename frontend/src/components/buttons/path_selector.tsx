import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button } from '@mui/material';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { openPath } from '@/tauri_cmd';

type Props = Readonly<{
  path: string;
  isDir?: boolean;
  setValue: (value: string) => void;
}>;

export function SelectPathButton({ path, isDir = false, setValue }: Props) {
  const { t } = useTranslation();
  const handleClick = async () => {
    openPath(path, setValue, isDir).catch((e) => toast.error(`${e}`));
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
