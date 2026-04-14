import { useTranslation } from '@/components/hooks/useTranslation';

import type { InputModInfoField } from '../InputModInfoField';
import type { ComponentPropsWithRef } from 'react';

export const useModInfoFields = () => {
  const { t } = useTranslation();

  return [
    {
      name: 'modName',
      helperText: t('convert-form-mod-name-helper'),
      label: t('convert-form-mod-name'),
      placeholder: '',
    },
    {
      name: 'modAuthor',
      helperText: t('convert-form-author-name-helper'),
      label: t('convert-form-author-name'),
      placeholder: t('convert-form-author-placeholder'),
    },
    {
      name: 'modDescription',
      helperText: t('convert-form-mod-description-helper'),
      label: t('convert-form-mod-description'),
      placeholder: t('convert-form-mod-description-placeholder'),
    },
  ] satisfies ComponentPropsWithRef<typeof InputModInfoField>[];
};
