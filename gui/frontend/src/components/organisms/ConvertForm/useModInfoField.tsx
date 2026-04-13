import { useTranslation } from '@/components/hooks/useTranslation';

import type { InputModInfoField } from './InputModInfoField';
import type { ComponentPropsWithRef } from 'react';

export const useModInfoFields = () => {
  const { t } = useTranslation();

  return [
    {
      name: 'modName',
      helperText: t('convert-form-mod-name-helper'),
      label: t('convert-form-mod-name'),
      placeholder: t('convert-form-mod-name'),
    },
    {
      name: 'modAuthor',
      helperText: t('convert-form-author-name-helper'),
      label: t('convert-form-author-name'),
      placeholder: t('convert-form-author-placeholder'),
    },
  ] satisfies ComponentPropsWithRef<typeof InputModInfoField>[];
};
