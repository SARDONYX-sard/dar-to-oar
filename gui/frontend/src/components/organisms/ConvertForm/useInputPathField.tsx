import { useFormContext } from 'react-hook-form';

import { useTranslation } from '@/components/hooks/useTranslation';
import { parseDarPath } from '@/lib/path/parseDarPath';

import { MappingHelpButton } from './MappingHelpButton';

import type { FormProps } from './ConvertForm';
import type { InputPathField } from './InputPathField';
import type { ComponentPropsWithRef } from 'react';

/** NOTE: Must be called in `FormProvider` as it calls `useFormContext` internally */
export const useInputPathFields = () => {
  const { t } = useTranslation();
  const { setValue, getValues } = useFormContext<FormProps>();

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
      isDir: true,
      onChange(e) {
        if (getValues('inferPath')) {
          const parsedPath = parseDarPath(e.target.value);
          setValue('dst', parsedPath.oarRoot);
          setValue('modName', parsedPath.modName ?? '');
        }
      },
      setPathHook(path: string) {
        if (getValues('inferPath')) {
          const parsedPath = parseDarPath(path);
          setValue('dst', parsedPath.oarRoot);
          setValue('modName', parsedPath.modName ?? '');
        }
      },
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
      isDir: true,
    },

    {
      helperText: <MappingHelpButton />,
      label: t('convert-form-mapping-label'),
      name: 'mapping1personPath',
      placeholder: './mapping_table.txt',
      isDir: false,
    },
    {
      helperText: t('convert-form-mapping-helper'),
      label: t('convert-form-mapping-1st-label'),
      name: 'mappingPath',
      placeholder: './mapping_table_for_1st_person.txt',
      isDir: false,
    },
  ] satisfies ComponentPropsWithRef<typeof InputPathField>[];
};
