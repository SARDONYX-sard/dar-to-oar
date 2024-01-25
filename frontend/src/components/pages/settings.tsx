'use client';

import TabContext from '@mui/lab/TabContext';
import TabList from '@mui/lab/TabList';
import TabPanel from '@mui/lab/TabPanel';
import { Box, Button, Grid } from '@mui/material';
import InputLabel from '@mui/material/InputLabel';
import Tab from '@mui/material/Tab';
import { useState } from 'react';
import AceEditor from 'react-ace';

import { ImportLangButton } from '@/components/buttons';
import { SelectEditorMode, SelectEditorProps, StyleList, StyleListProps, TranslationList } from '@/components/lists';
import { useDynStyle, useInjectScript, useLocale, useStorageState, useTranslation } from '@/hooks';
import { start } from '@/tauri_cmd';
import { selectEditorMode, type EditorMode } from '@/utils/selector';

import packageJson from '@/../../package.json';

// NOTE: These extensions must be loaded after importing AceEditor or they will error
import 'ace-builds/src-noconflict/ext-code_lens';
import 'ace-builds/src-noconflict/ext-language_tools';
import 'ace-builds/src-noconflict/keybinding-vim';
import 'ace-builds/src-noconflict/mode-css';
import 'ace-builds/src-noconflict/mode-javascript';
import 'ace-builds/src-noconflict/snippets/css';
import 'ace-builds/src-noconflict/snippets/javascript';
import 'ace-builds/src-noconflict/theme-one_dark';

export default function Settings() {
  useLocale();
  const [editorMode, setEditorMode] = useStorageState('editorMode', 'default');
  const [preset, setPreset] = useStorageState('presetNumber', '0');
  const [style, setStyle] = useDynStyle();

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
      <CSSEditor editorMode={editorMode} setPreset={setPreset} setStyle={setStyle} style={style} />

      <JSEditor editorMode={editorMode} />

      <Grid container sx={{ width: '95%' }}>
        <Grid sx={{ overflowX: 'auto' }} xs={8}>
          <Tabs
            editorMode={selectEditorMode(editorMode) ?? 'default'}
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

type CSSEditorProps = {
  editorMode: string;
  setPreset: (script: string) => void;
  setStyle: (style: string) => void;
  style: string;
};
const CSSEditor = ({ editorMode, setPreset, setStyle, style }: CSSEditorProps) => {
  const { t } = useTranslation();

  return (
    <>
      <InputLabel sx={{ marginTop: '20px' }}>{t('custom-css-label')}</InputLabel>
      <AceEditor
        style={{
          width: '95%',
          backgroundColor: '#2424248c',
        }}
        onChange={(value) => {
          setStyle(value);
          localStorage.setItem('customCSS', value);
          setPreset('0');
        }}
        fontSize={'1rem'}
        height="300px"
        mode="css"
        theme="one_dark"
        value={style}
        setOptions={{ useWorker: false }}
        placeholder="{ body: url('https://localhost' }"
        name="Custom CSS"
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        keyboardHandler={selectEditorMode(editorMode)}
        highlightActiveLine
        tabSize={2}
        editorProps={{ $blockScrolling: true }}
      />
    </>
  );
};

type JSEditorProps = {
  editorMode: string;
};
const JSEditor = ({ editorMode }: JSEditorProps) => {
  const { t } = useTranslation();
  const [script, setScript] = useInjectScript();

  return (
    <>
      <InputLabel error sx={{ marginTop: '20px' }}>
        {t('custom-js-label')}
      </InputLabel>
      <AceEditor
        style={{
          width: '95%',
          backgroundColor: '#2424248c',
        }}
        onChange={(value) => {
          localStorage.setItem('customJS', value);
          setScript(value);
        }}
        placeholder={`(()=> {
    const p = document.createElement('p');
    p.innerText = 'Hello';
    document.body.appendChild(p);
)()`}
        editorProps={{ $blockScrolling: true }}
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        fontSize={'1rem'}
        height="250px"
        highlightActiveLine
        keyboardHandler={selectEditorMode(editorMode)}
        mode="javascript"
        name="Custom JavaScript"
        setOptions={{ useWorker: false }}
        tabSize={2}
        theme="one_dark"
        value={script}
      />
    </>
  );
};

function Tabs({
  editorMode,
  setEditorMode,
  preset,
  setPreset,
  setStyle,
}: StyleListProps & SelectEditorProps & CSSEditorProps) {
  const [value, setValue] = useState('1');
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
            <Tab label={t('tab-label-1')} value="1" />
            <Tab label={t('tab-label-2')} value="2" />
          </TabList>
        </Box>
        <TabPanel value="1">
          <SelectEditorMode editorMode={editorMode} setEditorMode={setEditorMode} />
          <StyleList preset={preset} setPreset={setPreset} setStyle={setStyle} />
        </TabPanel>
        <TabPanel value="2">
          <ImportLangButton />
          <TranslationList />
        </TabPanel>
      </TabContext>
    </Box>
  );
}

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
