'use client';

import TabContext from '@mui/lab/TabContext';
import TabList from '@mui/lab/TabList';
import TabPanel from '@mui/lab/TabPanel';
import { Box, Button, Grid, Skeleton } from '@mui/material';
import Tab from '@mui/material/Tab';
import { Suspense, type SyntheticEvent } from 'react';

import { ExportBackupButton, ImportBackupButton, ImportLangButton } from '@/components/buttons';
import { CssEditor, type CssEditorProps, JsEditor } from '@/components/editor';
import {
  NoticeSettingsList,
  SelectEditorMode,
  type SelectEditorProps,
  StyleList,
  type StyleListProps,
  TranslationList,
} from '@/components/lists';
import { useDynStyle, useLocale, useStorageState, useTranslation } from '@/hooks';
import { start } from '@/tauri_cmd';
import { type EditorMode, selectEditorMode } from '@/utils/selector';

import packageJson from '@/../../package.json';

export default function Settings() {
  useLocale();
  const [editorMode, setEditorMode] = useStorageState('editorMode', 'default');
  const [preset, setPreset] = useStorageState('presetNumber', '0');
  const [style, setStyle] = useDynStyle();
  const validEditorMode = selectEditorMode(editorMode);

  const setEditorKeyMode = (editorMode: EditorMode) => setEditorMode(editorMode);
  return (
    <Box
      component='main'
      sx={{
        alignItems: 'center',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        minHeight: 'calc(100vh - 56px)',
        width: '100%',
      }}
    >
      <Suspense fallback={<Skeleton />}>
        <CssEditor editorMode={validEditorMode} setPreset={setPreset} setStyle={setStyle} style={style} />
      </Suspense>
      <Suspense fallback={<Skeleton />}>
        <JsEditor editorMode={validEditorMode} marginTop={'20px'} />
      </Suspense>

      <Grid container sx={{ width: '95%' }}>
        <Grid sx={{ overflowX: 'auto' }} xs={8}>
          <Tabs
            editorMode={validEditorMode}
            preset={preset}
            setEditorMode={setEditorKeyMode}
            setPreset={setPreset}
            setStyle={setStyle}
            style={style}
          />
        </Grid>
        <Grid sx={{ overflowX: 'auto' }} xs={4}>
          <Help />
        </Grid>
      </Grid>
    </Box>
  );
}

type TabsProps = StyleListProps & SelectEditorProps & CssEditorProps;
const Tabs = ({ editorMode, setEditorMode, preset, setPreset, setStyle }: TabsProps) => {
  const [value, setValue] = useStorageState('settings-tab-select', 'editor');
  const { t } = useTranslation();

  const handleChange = (_event: SyntheticEvent, newValue: string) => {
    setValue(newValue);
  };

  return (
    <Box
      sx={{
        minWidth: 'max-content',
        typography: 'body1',
      }}
    >
      <TabContext value={value}>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <TabList aria-label='Setting tabs' onChange={handleChange}>
            <Tab label={t('tab-label-editor')} value='editor' />
            <Tab label={t('tab-label-notice')} value='notice' />
            <Tab label={t('tab-label-lang')} value='lang' />
            <Tab label={t('tab-label-backup')} value='backup' />
          </TabList>
        </Box>
        <TabPanel value='editor'>
          <SelectEditorMode editorMode={editorMode} setEditorMode={setEditorMode} />
          <StyleList preset={preset} setPreset={setPreset} setStyle={setStyle} />
        </TabPanel>
        <TabPanel value='notice'>
          <NoticeSettingsList />
        </TabPanel>
        <TabPanel value='lang'>
          <ImportLangButton />
          <TranslationList />
        </TabPanel>
        <TabPanel value='backup'>
          <ImportBackupButton />
          <ExportBackupButton />
        </TabPanel>
      </TabContext>
    </Box>
  );
};

const Help = () => {
  const handleClick = () => start(packageJson.homepage);
  return (
    <Box
      sx={{
        display: 'flex',
        alignItems: 'center',
        flexDirection: 'column',
        height: '100%',
        justifyContent: 'space-evenly',
      }}
    >
      <div>Version: {packageJson.version}</div>
      <div>
        Source:{' '}
        <Button onClick={handleClick} sx={{ fontSize: 'large' }} type='button' variant='text'>
          GitHub
        </Button>
      </div>
    </Box>
  );
};
