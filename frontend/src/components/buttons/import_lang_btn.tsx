import { FileOpen } from '@mui/icons-material';
import { Button, Tooltip } from '@mui/material';
import { toast } from 'react-hot-toast';

import { useTranslation } from '@/hooks';
import { importLang } from '@/tauri_cmd';

export const ImportLangButton = () => {
  const { t } = useTranslation();

  const handleClick = async () => {
    try {
      const [isCancelled, contents] = await importLang();
      JSON.parse(contents); // Parse test
      if (!isCancelled) {
        localStorage.setItem('custom-translation-dict', contents);
        localStorage.setItem('locale', 'custom');
        window.location.reload(); // To enable
      }
    } catch (e) {
      toast.error(`${e}`);
    }
  };

  return (
    <Tooltip
      title={
        <>
          <p>{t('import-lang-tooltip')}</p>
          <p>{t('import-lang-tooltip2')}</p>
        </>
      }
    >
      <Button
        sx={{
          height: '4em',
          marginTop: '8px',
          minWidth: '120px',
          width: '120px',
        }}
        onClick={handleClick}
        startIcon={<FileOpen />}
        type="button"
        variant="outlined"
      >
        {t('import-lang-btn')}
      </Button>
    </Tooltip>
  );
};
