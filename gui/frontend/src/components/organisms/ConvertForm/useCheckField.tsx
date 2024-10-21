import AutoFixNormalIcon from '@mui/icons-material/AutoFixNormal';
import DynamicFeedIcon from '@mui/icons-material/DynamicFeed';
import SlideshowIcon from '@mui/icons-material/Slideshow';
import VisibilityOffIcon from '@mui/icons-material/VisibilityOff';

import { useTranslation } from '@/components/hooks/useTranslation';

import type { CheckboxField } from './CheckboxField';
import type { ComponentPropsWithRef } from 'react';

export const useCheckFields = () => {
  const { t } = useTranslation();

  return [
    {
      icon: <AutoFixNormalIcon />,
      label: t('infer-btn'),
      name: 'inferPath',
      tooltipText: t('infer-btn-tooltip'),
    },
    {
      icon: <VisibilityOffIcon />,
      label: t('hide-dar-btn'),
      name: 'hideDar',
      tooltipText: t('hide-dar-btn-tooltip'),
    },

    {
      icon: <SlideshowIcon />,
      label: t('progress-btn'),
      name: 'showProgress',
      tooltipText: t('progress-btn-tooltip'),
    },
    {
      icon: <DynamicFeedIcon />,
      label: t('run-parallel-label'),
      name: 'runParallel',
      tooltipText: (
        <p>
          {t('run-parallel-btn-tooltip')} <br />
          {t('run-parallel-btn-tooltip2')}
        </p>
      ),
    },
  ] satisfies ComponentPropsWithRef<typeof CheckboxField>[];
};
