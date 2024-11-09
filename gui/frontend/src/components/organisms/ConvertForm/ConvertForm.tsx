import ClearAllIcon from '@mui/icons-material/ClearAll';
import { Button, FormGroup } from '@mui/material';
import Grid from '@mui/material/Grid2';
import { FormProvider, type SubmitHandler, useForm } from 'react-hook-form';

import { useTranslation } from '@/components/hooks/useTranslation';
import { ConvertNav, ConvertNavPadding } from '@/components/organisms/ConvertNav';
import { parseDarPath } from '@/lib/path/parseDarPath';
import { STORAGE } from '@/lib/storage';
import { PRIVATE_CACHE_OBJ, PUB_CACHE_OBJ } from '@/lib/storage/cacheKeys';
import { convertDar2oar } from '@/services/api/convert';
import { progressListener } from '@/services/api/event';
import { LOG, type LogLevel } from '@/services/api/log';

import { CheckboxField } from './CheckboxField';
import { InputModInfoField } from './InputModInfoField';
import { InputPathField } from './InputPathField';
import { useCheckFields } from './useCheckField';
import { useInputPathFields } from './useInputPathField';
import { useModInfoFields } from './useModInfoField';

import type { ComponentPropsWithRef } from 'react';

export type FormProps = {
  src: string;
  dst: string;
  modName: string;
  modAuthor: string;
  mappingPath: string;
  mapping1personPath: string;
  loading: boolean;
  logLevel: LogLevel;
  runParallel: boolean;
  hideDar: boolean;
  showProgress: boolean;
  inferPath: boolean;
  progress: number;
};

const getInitialFormValues = (): FormProps => ({
  src: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.src),
  dst: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.dst),
  modName: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.modName),
  modAuthor: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.modAuthor),
  mappingPath: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.mappingPath),
  mapping1personPath: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.mapping1personPath),
  loading: false,
  logLevel: LOG.get(),
  runParallel: STORAGE.get(PUB_CACHE_OBJ.runParallel) === 'true',
  hideDar: STORAGE.get(PUB_CACHE_OBJ.hideDar) === 'true',
  showProgress: STORAGE.get(PUB_CACHE_OBJ.showProgress) === 'true',
  inferPath: STORAGE.get(PUB_CACHE_OBJ.inferPath) === 'true',
  progress: 0,
});

const PATH_FORM_VALUES = ['src', 'dst', 'mapping1personPath', 'mappingPath', 'modAuthor', 'modName'] as const;

export type PathFormKeys = (typeof PATH_FORM_VALUES)[number];

export const setPathToStorage = (name: PathFormKeys, path: string) => {
  STORAGE.set(name, path);
  if (path !== '') {
    STORAGE.set(`cached-${name}`, path);
    return;
  }
  STORAGE.remove(name);
};

export function ConvertForm() {
  const { t } = useTranslation();

  const methods = useForm({
    mode: 'onBlur',
    criteriaMode: 'all',
    shouldFocusError: false,
    defaultValues: getInitialFormValues(),
  });
  const { setValue, getValues } = methods;

  const handleAllClear = () => {
    for (const key of PATH_FORM_VALUES) {
      setValue(key, '');
      setPathToStorage(key, '');
    }
  };

  const onSubmit: SubmitHandler<FormProps> = async (formProps) => {
    const setLoading = (loading: boolean) => setValue('loading', loading);
    const task = async () => await convertDar2oar(formProps);

    await progressListener('/dar2oar/progress/converter', task, {
      setLoading,
      setProgress: (percentage: number) => setValue('progress', percentage),
      success: t('conversion-complete'),
    });
  };

  const pathFields = useInputPathFields();
  const modInfoFields = useModInfoFields();
  const checkFields = useCheckFields();

  return (
    <FormProvider {...methods}>
      <Grid
        component='form'
        container={true}
        onSubmit={methods.handleSubmit(onSubmit)}
        sx={{ width: '100vw', justifyContent: 'center' }}
      >
        <FormGroup sx={{ width: '95%' }}>
          <Button
            onClick={handleAllClear}
            startIcon={<ClearAllIcon />}
            sx={{ width: '100%', marginTop: '15px', marginBottom: '15px' }}
            variant='outlined'
          >
            {t('all-clear-btn')}
          </Button>

          {pathFields.map((props) => {
            let onChange: ComponentPropsWithRef<typeof InputPathField>['onChange'] | undefined;
            let setPathHook: ((path: string) => void) | undefined;

            if (props.name === 'src') {
              onChange = (e) => {
                if (getValues('inferPath')) {
                  const parsedPath = parseDarPath(e.target.value);
                  setValue('dst', parsedPath.oarRoot);
                  setValue('modName', parsedPath.modName ?? '');
                }
              };
              setPathHook = (path: string) => {
                if (getValues('inferPath')) {
                  const parsedPath = parseDarPath(path);
                  setValue('dst', parsedPath.oarRoot);
                  setValue('modName', parsedPath.modName ?? '');
                }
              };
            }
            return <InputPathField key={props.name} onChange={onChange} setPathHook={setPathHook} {...props} />;
          })}

          <Grid columnSpacing={1} container={true} gap={2} sx={{ width: '100%' }}>
            {modInfoFields.map((props) => {
              return <InputModInfoField key={props.name} {...props} />;
            })}

            {checkFields.map((props) => {
              return (
                <Grid key={props.name} size={1.5} sx={{ display: 'flex', placeItems: 'center' }}>
                  <CheckboxField {...props} />
                </Grid>
              );
            })}
          </Grid>
          <ConvertNavPadding />
        </FormGroup>

        <ConvertNav />
      </Grid>
    </FormProvider>
  );
}
