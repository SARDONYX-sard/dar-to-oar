import ConvertIcon from '@mui/icons-material/Transform';
import LoadingButton, { type LoadingButtonProps } from '@mui/lab/LoadingButton';

import { useTranslation } from '@/components/hooks/useTranslation';

type Props = LoadingButtonProps;

/**
 *
 * Icon ref
 * - https://mui.com/material-ui/material-icons/
 */
export function ConvertButton({ loading, ...props }: Props) {
  const { t } = useTranslation();

  return (
    <LoadingButton
      endIcon={<ConvertIcon />}
      loading={loading}
      loadingPosition='end'
      sx={{
        height: '40px',
        minWidth: '100%',
      }}
      type='button'
      variant='contained'
      {...props}
    >
      <span>{loading ? t('converting-btn') : t('convert-btn')}</span>
    </LoadingButton>
  );
}
