import ConvertIcon from '@mui/icons-material/Transform';
import LoadingButton from '@mui/lab/LoadingButton';

import { useTranslation } from '@/hooks';

type Props = Readonly<{
  loading: boolean;
  setLoading: (loading: boolean) => void;
}>;

/**
 *
 * Icon ref
 * - https://mui.com/material-ui/material-icons/
 */
export function ConvertButton({ loading, setLoading }: Props) {
  const { t } = useTranslation();

  return (
    <LoadingButton
      type="submit"
      sx={{ width: '100%' }}
      endIcon={<ConvertIcon />}
      loading={loading}
      loadingPosition="end"
      variant="contained"
      onChange={() => setLoading(true)}
    >
      <span>{loading ? t('converting-btn') : t('convert-btn')}</span>
    </LoadingButton>
  );
}
