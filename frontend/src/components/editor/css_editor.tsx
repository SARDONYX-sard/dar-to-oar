import InputLabel from '@mui/material/InputLabel';
import AceEditor from 'react-ace';

import { useTranslation } from '@/hooks';
import { selectEditorMode } from '@/utils/selector';

export type CSSEditorProps = {
  editorMode: string;
  setPreset: (script: string) => void;
  setStyle: (style: string) => void;
  style: string;
};

export const CSSEditor = ({ editorMode, setPreset, setStyle, style }: CSSEditorProps) => {
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
