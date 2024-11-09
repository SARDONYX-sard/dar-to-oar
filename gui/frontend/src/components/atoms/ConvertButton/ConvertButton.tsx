import ConvertIcon from '@mui/icons-material/Transform';
import LoadingButton, { type LoadingButtonProps } from '@mui/lab/LoadingButton';
import { useEffect, useState } from 'react';

import { CircularProgressWithLabel } from '@/components/atoms/CircularProgressWithLabel';
import { useTranslation } from '@/components/hooks/useTranslation';

type Props = LoadingButtonProps & { progress: number };

/**
 *
 * Icon ref
 * - https://mui.com/material-ui/material-icons/
 */
export function ConvertButton({ loading, progress, ...props }: Props) {
  const { t } = useTranslation();

  // State to track when loading is complete
  const [isComplete, setIsComplete] = useState(false);

  useEffect(() => {
    if (!loading && progress === 100) {
      setIsComplete(true);

      // Keep the "Converted 100%" text visible for 1 second
      const completeTimer = setTimeout(() => {
        setIsComplete(false); // Reset the background
      }, 500);

      return () => {
        clearTimeout(completeTimer);
      };
    }
  }, [loading, progress]);

  return (
    <LoadingButton
      disabled={loading || isComplete}
      endIcon={loading || isComplete ? undefined : <ConvertIcon />}
      loading={loading}
      loadingPosition='end'
      sx={{
        height: '55px',
        minWidth: '40%',
        position: 'relative',
        overflow: 'hidden',
        transition: 'all 0.8s ease', // Smooth transition for button state change
        ...(loading || isComplete
          ? {
              '&:before': {
                content: '""',
                position: 'absolute',
                top: 0,
                left: 0,
                width: `${progress}%`, // Progress-based width
                height: '100%',
                // backgroundColor: '#1e1f1e57',
                backgroundColor: 'var(--theme-color)',
                zIndex: 0,
                transition: 'width 0.5s ease-in-out',
                transform: `scaleX(${progress / 100})`,
                transformOrigin: 'left',
              },
              '& .MuiButton-label': {
                position: 'relative',
                zIndex: 1, // Keep label above background
                opacity: loading ? 0.8 : 1,
                transition: 'opacity 0.5s ease',
              },
            }
          : {}),
      }}
      type='submit'
      variant='contained'
      {...props}
    >
      <span>
        {loading ? (
          <>
            {t('converting-btn')}
            <CircularProgressWithLabel value={progress} />
          </>
        ) : isComplete ? (
          <>
            OK
            <CircularProgressWithLabel value={progress} />
          </>
        ) : (
          t('convert-btn')
        )}
      </span>
    </LoadingButton>
  );
}
