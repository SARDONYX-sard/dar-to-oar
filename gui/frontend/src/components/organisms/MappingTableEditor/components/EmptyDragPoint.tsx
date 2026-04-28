import { Box } from '@mui/material';
import { useTranslation } from '@/components/hooks/useTranslation';

export const EmptyDragPoint = () => {
  const { t } = useTranslation();
  return (
    <Box
      sx={{
        flexGrow: 1,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: '#777',
        textAlign: 'center',
        px: 2,
      }}
    >
      {t('empty-drag-point-no-data')}
      <br />
      {t('empty-drag-point-drop-files')}
    </Box>
  );
};
