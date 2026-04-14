import ClearAllIcon from '@mui/icons-material/ClearAll';
import { Button, FormGroup, Grid } from '@mui/material';
import { FormProvider, type SubmitHandler, useForm } from 'react-hook-form';
import { CheckboxField } from './CheckboxField';
import { useCheckFields } from './hooks/useCheckField';
import { useInputPathFields } from './hooks/useInputPathField';
import { useModInfoFields } from './hooks/useModInfoField';
import { InputModInfoField } from './InputModInfoField';
import { InputPathField } from './InputPathField';
import { useTranslation } from '@/components/hooks/useTranslation';
import { ConvertNav, ConvertNavPadding } from '@/components/organisms/ConvertForm/ConvertNav';
import { STORAGE } from '@/lib/storage';
import { PRIVATE_CACHE_OBJ, PUB_CACHE_OBJ } from '@/lib/storage/cacheKeys';
import { convertDar2oar } from '@/services/api/convert';
import { progressListener } from '@/services/api/event';
import { LOG, type LogLevel } from '@/services/api/log';

export type FormProps = {
  src: string;
  dst: string;
  modName: string;
  modAuthor: string;
  modDescription: string;
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

const defaultFormValues = (): FormProps => ({
  src: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.src),
  dst: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.dst),
  modName: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.modName),
  modAuthor: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.modAuthor),
  modDescription: STORAGE.getOrDefault(PRIVATE_CACHE_OBJ.modDescription),
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

const PATH_FORM_VALUES = [
  'src',
  'dst',
  'mapping1personPath',
  'mappingPath',
  'modAuthor',
  'modName',
  'modDescription',
] as const;

export type PathFormKeys = (typeof PATH_FORM_VALUES)[number];

export const setPathToStorage = (name: PathFormKeys, path: string) => {
  STORAGE.set(name, path);
  if (path !== '') {
    localStorage.setItem(`cached-${name}`, path);
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
    defaultValues: defaultFormValues(),
  });
  const { setValue } = methods;

  const handleAllClear = () => {
    for (const key of PATH_FORM_VALUES) {
      setValue(key, '');
      setPathToStorage(key, '');
    }
  };

  const onSubmit: SubmitHandler<FormProps> = async (formProps) => {
    const setLoading = (loading: boolean) => setValue('loading', loading);
    const task = async () => await convertDar2oar(formProps);
    const start = Date.now();

    await progressListener('/dar2oar/progress/converter', task, {
      setLoading,
      setProgress: (percentage: number) => setValue('progress', percentage),
      success: () => t('conversion-complete') + ` (${((Date.now() - start) / 1000).toFixed(2)}s)`,
    });
  };

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

          <InputPathFields />

          <Grid columnSpacing={2} container={true} sx={{ width: '100%' }}>
            {modInfoFields.map((props) => {
              return <InputModInfoField key={props.name} {...props} />;
            })}
          </Grid>

          <Grid
            columnSpacing={1}
            container={true}
            sx={{
              width: '100%',
              justifyContent: 'flex-end',
            }}
          >
            {checkFields.map((props) => (
              <CheckboxField {...props} />
            ))}
          </Grid>

          <ConvertNavPadding />
        </FormGroup>

        <ConvertNav />
      </Grid>
    </FormProvider>
  );
}

// NOTE: It is necessary to call `useFormContext` in `FormProvider` since it is called internally.
//       This avoids the wrapper component wrapping inaccessibility error.
const InputPathFields = () => {
  const pathFields = useInputPathFields();

  return pathFields.map((props) => {
    return <InputPathField key={props.name} {...props} />;
  });
};
