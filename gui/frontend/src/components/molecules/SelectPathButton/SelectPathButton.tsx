import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import { Button } from '@mui/material';
import { ComponentPropsWithRef } from 'react';
import { useTranslation } from '@/components/hooks/useTranslation';
import { NOTIFY } from '@/lib/notify';
import { openPath } from '@/services/api/dialog';

type Props = Readonly<{
  path: string;
  setPath: (value: string) => void;
  isDir?: boolean;
}> &
  ComponentPropsWithRef<typeof Button>;

export function SelectPathButton({ path, isDir = false, setPath, ...props }: Props) {
  const { t } = useTranslation();
  const handleClick = async () => {
    await NOTIFY.asyncTry(async () => await openPath(path, { setPath, directory: isDir }));
  };

  return (
    <Button onClick={handleClick} startIcon={<FolderOpenIcon />} type='button' variant='outlined' {...props}>
      {t('select-btn')}
    </Button>
  );
}
