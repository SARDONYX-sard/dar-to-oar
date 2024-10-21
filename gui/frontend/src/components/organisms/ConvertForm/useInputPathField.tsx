import { useTranslation } from '@/components/hooks/useTranslation';

import { MappingHelpButton } from './MappingHelpButton';

import type { InputPathField } from './InputPathField';
import type { ComponentPropsWithRef } from 'react';

export const useInputPathFields = () => {
  const { t } = useTranslation();

  return [
    {
      helperText: (
        <>
          {t('convert-form-dar-helper')} <br />
          {t('convert-form-dar-helper2')} <br />
          {t('convert-form-dar-helper3')}
        </>
      ),
      label: t('convert-form-dar-label'),
      name: 'src',
      placeholder: '[...]/<MOD NAME>',
    },
    {
      helperText: (
        <>
          {t('convert-form-oar-helper')} <br />
          {t('convert-form-oar-helper2')}
        </>
      ),
      label: t('convert-form-oar-label'),
      name: 'dst',
      placeholder: '[...]/<MOD NAME>',
    },

    {
      helperText: <MappingHelpButton />,
      label: t('convert-form-mapping-label'),
      name: 'mapping1personPath',
      placeholder: './mapping_table.txt',
    },
    {
      helperText: t('convert-form-mapping-helper'),
      label: t('convert-form-mapping-1st-label'),
      name: 'mappingPath',
      placeholder: './mapping_table_for_1st_person.txt',
    },
  ] satisfies ComponentPropsWithRef<typeof InputPathField>[];
};
