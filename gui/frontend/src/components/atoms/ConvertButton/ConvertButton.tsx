import ConvertIcon from '@mui/icons-material/Transform';
import { Button, ButtonProps } from '@mui/material';
import { useTranslation } from '@/components/hooks/useTranslation';

type Props = ButtonProps & { progress: number };

/**
 *
 * Icon ref
 * - https://mui.com/material-ui/material-icons/
 */
export function ConvertButton({ loading, ...props }: Props) {
  const { t } = useTranslation();

  return (
    <Button
      endIcon={<ConvertIcon />}
      loading={loading}
      loadingPosition='end'
      sx={{
        height: '55px',
        minWidth: '40%',
        position: 'relative',
        overflow: 'hidden',
        transition: 'all 0.8s ease', // Smooth transition for button state change
      }}
      type='submit'
      variant='contained'
      {...props}
    >
      <span>{loading ? t('converting-btn') : t('convert-btn')}</span>
    </Button>
  );
}
