import ClearAllIcon from '@mui/icons-material/ClearAll';
import SlideshowIcon from '@mui/icons-material/Slideshow';
import VisibilityOffIcon from '@mui/icons-material/VisibilityOff';
import { Box, Button, FormControlLabel, FormGroup, TextField, Tooltip } from '@mui/material';
import Checkbox from '@mui/material/Checkbox';
import Grid from '@mui/material/Grid2';
import { Controller, type SubmitHandler, useForm } from 'react-hook-form';

import {
  ConvertButton,
  LogDirButton,
  LogFileButton,
  RemoveOarBtn,
  SelectPathButton,
  UnhideDarBtn,
} from '@/components/buttons';
import { SelectLogLevel } from '@/components/lists';
import { LinearWithValueLabel } from '@/components/notifications';
import { useTranslation } from '@/hooks';
import { type LogLevel, convertDar2oar, progressListener, start } from '@/tauri_cmd';
import { localStorageManager } from '@/utils/local_storage_manager';
import { getParent } from '@/utils/path';
import { selectLogLevel } from '@/utils/selector';

type FormProps = {
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
  progress: number;
};

const getInitialFormValues = (): FormProps => ({
  src: localStorageManager.get('src') ?? '',
  dst: localStorageManager.get('dst') ?? '',
  modName: localStorageManager.get('modName') ?? '',
  modAuthor: localStorageManager.get('modAuthor') ?? '',
  mappingPath: localStorageManager.get('mappingPath') ?? '',
  mapping1personPath: localStorageManager.get('mapping1personPath') ?? '',
  loading: false as boolean,
  logLevel: selectLogLevel(localStorageManager.get('logLevel')),
  runParallel: localStorageManager.get('runParallel') === 'true',
  hideDar: localStorageManager.get('hideDar') === 'true',
  showProgress: localStorageManager.get('showProgress') === 'true',
  progress: 0,
});

export function ConvertForm() {
  const { t } = useTranslation();
  const { register, handleSubmit, control, setValue, watch } = useForm({
    mode: 'onBlur',
    criteriaMode: 'all',
    shouldFocusError: false,
    defaultValues: getInitialFormValues(),
  });

  /** Use `getValues` to get the old values and use `watch` to monitor `src` and `dst`. */
  const watchFields = watch(['src', 'dst']);

  const setStorage = (key: keyof FormProps) => {
    return (value: string) => {
      if (!(key === 'loading' || key === 'progress')) {
        localStorageManager.set(key, value);
      }

      if (value === '') {
        localStorage.removeItem(key);
      } else {
        localStorage.setItem(`cached-${key}`, value);
      }

      setValue(key, value);
    };
  };

  const setLoading = (loading: boolean) => setValue('loading', loading);
  const handleAllClear = () => {
    const formValues = ['src', 'dst', 'mapping1personPath', 'mappingPath', 'modAuthor', 'modName'] as const;

    for (const key of formValues) {
      setStorage(key)('');
    }
  };

  const onSubmit: SubmitHandler<FormProps> = async (formProps) => {
    await progressListener('/dar2oar/progress/converter', async () => await convertDar2oar(formProps), {
      setLoading,
      setProgress(percentage: number) {
        setValue('progress', percentage);
      },
      success: t('conversion-complete'),
    });
  };

  return (
    <Grid component='form' container onSubmit={handleSubmit(onSubmit)} sx={{ display: 'block', width: '95vw' }}>
      <Button
        onClick={handleAllClear}
        startIcon={<ClearAllIcon />}
        sx={{ width: '100%', marginBottom: '15px' }}
        variant='outlined'
      >
        {t('all-clear-btn')}
      </Button>
      <FormGroup onSubmit={handleSubmit(onSubmit)}>
        <Controller
          control={control}
          name='src'
          render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
            <Grid container spacing={2}>
              <Grid size={10}>
                <TextField
                  error={Boolean(error)}
                  helperText={
                    <>
                      {t('convert-form-dar-helper')} <br />
                      {t('convert-form-dar-helper2')} <br />
                      {t('convert-form-dar-helper3')}
                    </>
                  }
                  label={t('convert-form-dar-label')}
                  margin='dense'
                  onBlur={onBlur}
                  onChange={(e) => {
                    onChange(e);
                    const path = e.target.value;
                    localStorageManager.set('src', path); // For reload cache
                    if (path !== '') {
                      localStorageManager.set('cached-src', path); // For empty string
                    }
                  }}
                  placeholder='[...]/<MOD NAME>'
                  required
                  sx={{ width: '100%' }}
                  value={value}
                  variant='outlined'
                />
              </Grid>

              <Grid size={2}>
                <SelectPathButton
                  isDir
                  path={getParent(value === '' ? (localStorageManager.get('cached-src') ?? '') : value)}
                  setPath={setStorage('src')}
                />
              </Grid>
            </Grid>
          )}
          rules={{
            required: 'Need Path',
          }}
        />

        <Controller
          control={control}
          name='dst'
          render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
            <Grid container spacing={2}>
              <Grid size={10}>
                <TextField
                  error={Boolean(error)}
                  helperText={
                    <>
                      {t('convert-form-oar-helper')} <br />
                      {t('convert-form-oar-helper2')}
                    </>
                  }
                  label={t('convert-form-oar-label')}
                  margin='dense'
                  onBlur={onBlur}
                  onChange={(e) => {
                    onChange(e);
                    const path = e.target.value;
                    localStorageManager.set('dst', path);
                    if (path !== '') {
                      localStorageManager.set('cached-dst', path);
                    }
                  }}
                  placeholder='<MOD NAME>'
                  sx={{ width: '100%' }}
                  value={value}
                  variant='outlined'
                />
              </Grid>
              <Grid size={2}>
                <SelectPathButton
                  isDir
                  path={getParent(value === '' ? (localStorageManager.get('cached-dst') ?? '') : value)}
                  setPath={setStorage('dst')}
                />
              </Grid>
            </Grid>
          )}
        />

        <Controller
          control={control}
          name='mappingPath'
          render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
            <Grid container spacing={2}>
              <Grid size={10}>
                <TextField
                  error={Boolean(error)}
                  helperText={<MappingHelpBtn />}
                  label={t('convert-form-mapping-label')}
                  margin='dense'
                  onBlur={onBlur}
                  onChange={(e) => {
                    const path = e.target.value;
                    localStorageManager.set('mappingPath', path);
                    if (path !== '') {
                      localStorageManager.set('cached-mappingPath', path);
                    }
                    onChange(e);
                  }}
                  placeholder='./mapping_table.txt'
                  sx={{ width: '100%' }}
                  value={value}
                  variant='outlined'
                />
              </Grid>

              <Grid size={2}>
                <SelectPathButton
                  path={value === '' ? (localStorageManager.get('cached-mappingPath') ?? '') : value}
                  setPath={(value) => {
                    localStorageManager.set('cached-mappingPath', value);
                    setStorage('mappingPath')(value);
                  }}
                />
              </Grid>
            </Grid>
          )}
        />

        <Controller
          control={control}
          name='mapping1personPath'
          render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
            <Grid container spacing={2}>
              <Grid size={10}>
                <TextField
                  error={Boolean(error)}
                  helperText={t('convert-form-mapping-helper')}
                  label={t('convert-form-mapping-1st-label')}
                  margin='dense'
                  onBlur={onBlur}
                  onChange={(e) => {
                    const path = e.target.value;
                    localStorageManager.set('mapping1personPath', path);
                    if (path !== '') {
                      localStorageManager.set('cached-mapping1personPath', path);
                    }
                    onChange(e);
                  }}
                  placeholder='./mapping_table_for_1st_person.txt'
                  sx={{ minWidth: '100%' }}
                  value={value}
                  variant='outlined'
                />
              </Grid>

              <Grid size={2}>
                <SelectPathButton
                  path={value === '' ? (localStorageManager.get('cached-mapping1personPath') ?? '') : value}
                  setPath={setStorage('mapping1personPath')}
                />
              </Grid>
            </Grid>
          )}
        />

        <Grid container spacing={2}>
          <Grid size={3}>
            <Controller
              control={control}
              name='modName'
              render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
                <TextField
                  error={Boolean(error)}
                  helperText={t('convert-form-mod-name-helper')}
                  label={t('convert-form-mod-name')}
                  margin='dense'
                  onBlur={onBlur}
                  onChange={(e) => {
                    localStorageManager.set('modName', e.target.value);
                    onChange(e);
                  }}
                  placeholder={t('convert-form-mod-name')}
                  value={value}
                  variant='outlined'
                />
              )}
            />
          </Grid>

          <Grid size={3}>
            <Controller
              control={control}
              name='modAuthor'
              render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
                <TextField
                  error={Boolean(error)}
                  helperText={t('convert-form-author-name-helper')}
                  label={t('convert-form-author-name')}
                  margin='dense'
                  onBlur={onBlur}
                  onChange={(e) => {
                    localStorageManager.set('modAuthor', e.target.value);
                    onChange(e);
                  }}
                  placeholder={t('convert-form-author-placeholder')}
                  value={value}
                  variant='outlined'
                />
              )}
            />
          </Grid>

          <Grid size={2}>
            <Controller
              control={control}
              name='logLevel'
              render={({ field: { value } }) => <SelectLogLevel value={value} {...register('logLevel')} />}
            />
          </Grid>

          <Grid size={2}>
            <LogFileButton />
          </Grid>

          <Grid size={2}>
            <LogDirButton />
          </Grid>
        </Grid>

        <Grid container sx={{ alignItems: 'center' }}>
          <Grid size={3}>
            <Controller
              control={control}
              name='hideDar'
              render={({ field: { value } }) => (
                <Tooltip
                  title={
                    <p>
                      {t('hide-dar-btn-tooltip')} <br />
                      {t('hide-dar-btn-tooltip2')}
                    </p>
                  }
                >
                  <FormControlLabel
                    control={
                      <Checkbox
                        aria-label='Hide DAR'
                        checked={value}
                        onClick={() => {
                          localStorageManager.set('hideDar', `${!value}`);
                          setValue('hideDar', !value);
                        }}
                      />
                    }
                    label={
                      <Box component='div' sx={{ display: 'flex' }}>
                        <VisibilityOffIcon />
                        {t('hide-dar-btn')}
                      </Box>
                    }
                  />
                </Tooltip>
              )}
            />
          </Grid>

          <Grid size={3}>
            <Controller
              control={control}
              name='showProgress'
              render={({ field: { value } }) => (
                <Tooltip
                  title={
                    <>
                      {t('progress-btn-tooltip')} <br />
                      {t('progress-btn-tooltip2')}
                    </>
                  }
                >
                  <FormControlLabel
                    control={
                      <Checkbox
                        aria-label='Show Progress'
                        checked={value}
                        onClick={() => {
                          setValue('showProgress', !value);
                          localStorageManager.set('showProgress', `${!value}`);
                        }}
                      />
                    }
                    label={
                      <Box component='div' sx={{ display: 'flex' }}>
                        <SlideshowIcon />
                        {t('progress-btn')}
                      </Box>
                    }
                  />
                </Tooltip>
              )}
            />
          </Grid>

          <Grid size={3}>
            <Controller
              control={control}
              name='runParallel'
              render={({ field: { value } }) => (
                <Tooltip
                  title={
                    <p>
                      {t('run-parallel-btn-tooltip')} <br />
                      {t('run-parallel-btn-tooltip2')}
                    </p>
                  }
                >
                  <FormControlLabel
                    control={
                      <Checkbox
                        aria-label='Run Parallel'
                        checked={value}
                        onClick={() => {
                          localStorageManager.set('runParallel', `${!value}`);
                          setValue('runParallel', !value);
                        }}
                      />
                    }
                    label={t('run-parallel-label')}
                  />
                </Tooltip>
              )}
            />
          </Grid>
        </Grid>

        <Grid container spacing={2}>
          <Grid size={3}>
            <UnhideDarBtn path={watchFields[0]} />
          </Grid>
          <Grid size={3}>
            <RemoveOarBtn darPath={watchFields[0]} oarPath={watchFields[1]} />
          </Grid>
        </Grid>

        <Controller
          control={control}
          name='loading'
          render={({ field: { value } }) => (
            <Box sx={{ width: '100%', paddingTop: '10px' }}>
              <ConvertButton loading={value} setLoading={setLoading} />
            </Box>
          )}
        />

        <Controller
          control={control}
          name='progress'
          render={({ field: { value } }) => <LinearWithValueLabel progress={value} />}
        />
      </FormGroup>
    </Grid>
  );
}

function MappingHelpBtn() {
  const { t } = useTranslation();
  const handleMappingClick = () => start(`https://github.com/SARDONYX-sard/dar-to-oar/${t('mapping-wiki-url-leaf')}`);

  return (
    <>
      {t('convert-form-mapping-helper')}
      <br />
      {t('convert-form-mapping-helper2')}
      <Button onClick={handleMappingClick} style={{ fontSize: 'small' }} type='button'>
        [{t('convert-form-mapping-help-link-name')}]
      </Button>
    </>
  );
}
