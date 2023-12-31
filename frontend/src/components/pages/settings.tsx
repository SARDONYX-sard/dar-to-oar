'use client';

import { Box } from '@mui/material';
import InputLabel from '@mui/material/InputLabel';
import AceEditor from 'react-ace';

import { SelectEditorMode, StyleList, TranslationList } from '@/components/lists';
import { Toaster } from '@/components/notifications';
import { useDynStyle, useInjectScript, useLocale, useStorageState, useTranslation } from '@/hooks';
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
      <Toaster position="bottom-right" reverseOrder={false} />
      <CSSEditor editorMode={editorMode} setPreset={setPreset} setStyle={setStyle} style={style} />

      <JSEditor editorMode={editorMode} />
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          marginTop: '10px',
          overflowX: 'auto',
          width: '95%',
        }}
      >
        <SelectEditorMode editorMode={selectEditorMode(editorMode)} setEditorMode={setEditorKeyMode} />
        <StyleList preset={preset} setPreset={setPreset} setStyle={setStyle} />
        <TranslationList />
      </div>
      <Help />
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

const Help = () => {
  const handleClick = () => open(packageJson.homepage);
  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'space-around',
        marginTop: '10px',
        width: '55%',
      }}
    >
      <div>Version: {packageJson.version}</div>
      <div>
        Source:{' '}
        <a style={{ cursor: 'pointer', color: '#00c2ff' }} onClick={handleClick} onKeyDown={handleClick}>
          GitHub
        </a>
      </div>
    </div>
  );
};
