'use client';

import TabContext from '@mui/lab/TabContext';
import TabList from '@mui/lab/TabList';
import TabPanel from '@mui/lab/TabPanel';
import { Box, Button, Grid } from '@mui/material';
import Tab from '@mui/material/Tab';

import { ImportBackupButton, ExportBackupButton, ImportLangButton } from '@/components/buttons';
import { CSSEditor, type CSSEditorProps, JSEditor } from '@/components/editor';
import {
  NoticePositionList,
  SelectEditorMode,
  StyleList,
  TranslationList,
  type SelectEditorProps,
  type StyleListProps,
} from '@/components/lists';
import { useDynStyle, useLocale, useStorageState, useTranslation } from '@/hooks';
import { start } from '@/tauri_cmd';
import { selectEditorMode, type EditorMode } from '@/utils/selector';

import packageJson from '@/../../package.json';

export default function Settings() {
  useLocale();
  const [editorMode, setEditorMode] = useStorageState('editorMode', 'default');
  const [preset, setPreset] = useStorageState('presetNumber', '0');
  const [style, setStyle] = useDynStyle();
  const validEditorMode = selectEditorMode(editorMode);

  const setEditorKeyMode = (editorMode: EditorMode) => setEditorMode(editorMode ?? 'default');
  return (
    <Box
      component="main"
      sx={{
        alignItems: 'center',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        minHeight: 'calc(100vh - 56px)',
        width: '100%',
      }}
    >
      <CSSEditor editorMode={validEditorMode} setPreset={setPreset} setStyle={setStyle} style={style} />
      <JSEditor editorMode={validEditorMode} marginTop={'20px'} />

      <Grid container sx={{ width: '95%' }}>
        <Grid sx={{ overflowX: 'auto' }} xs={8}>
          <Tabs
            editorMode={validEditorMode}
            preset={preset}
            setPreset={setPreset}
            setStyle={setStyle}
            style={style}
            setEditorMode={setEditorKeyMode}
          />
        </Grid>
        <Grid sx={{ overflowX: 'auto' }} xs={4}>
          <Help />
        </Grid>
      </Grid>
    </Box>
  );
}

type TabsProps = StyleListProps & SelectEditorProps & CSSEditorProps;
const Tabs = ({ editorMode, setEditorMode, preset, setPreset, setStyle }: TabsProps) => {
  const [value, setValue] = useStorageState('settings-tab-select', 'editor');
  const { t } = useTranslation();

  const handleChange = (_event: React.SyntheticEvent, newValue: string) => {
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
          <TabList onChange={handleChange} aria-label="Setting tabs">
            <Tab label={t('tab-label-editor')} value="editor" />
            <Tab label={t('tab-label-notice')} value="notice" />
            <Tab label={t('tab-label-lang')} value="lang" />
            <Tab label={t('tab-label-backup')} value="backup" />
          </TabList>
        </Box>
        <TabPanel value="editor">
          <SelectEditorMode editorMode={editorMode} setEditorMode={setEditorMode} />
          <StyleList preset={preset} setPreset={setPreset} setStyle={setStyle} />
        </TabPanel>
        <TabPanel value="notice">
          <NoticePositionList />
        </TabPanel>
        <TabPanel value="lang">
          <ImportLangButton />
          <TranslationList />
        </TabPanel>
        <TabPanel value="backup">
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
        <Button onClick={handleClick} sx={{ fontSize: 'large' }} type="button" variant="text">
          GitHub
        </Button>
      </div>
    </Box>
  );
};
