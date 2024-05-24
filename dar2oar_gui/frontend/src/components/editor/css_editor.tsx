import InputLabel from '@mui/material/InputLabel';
import AceEditor from 'react-ace';

import { useTranslation } from '@/hooks';
import { selectEditorMode } from '@/utils/selector';

export type CssEditorProps = {
  editorMode: string;
  setPreset: (script: string) => void;
  setStyle: (style: string) => void;
  style: string;
};

export const CssEditor = ({ editorMode, setPreset, setStyle, style }: CssEditorProps) => {
  const { t } = useTranslation();

  return (
    <>
      <InputLabel sx={{ marginTop: '20px' }}>{t('custom-css-label')}</InputLabel>
      <AceEditor
        editorProps={{ $blockScrolling: true }}
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        fontSize={'1rem'}
        height='300px'
        highlightActiveLine
        keyboardHandler={selectEditorMode(editorMode)}
        mode='css'
        name='Custom CSS'
        onChange={(value) => {
          setStyle(value);
          localStorage.setItem('customCSS', value);
          setPreset('0');
        }}
        placeholder="{ body: url('https://localhost' }"
        setOptions={{ useWorker: false }}
        style={{
          width: '95%',
          backgroundColor: '#2424248c',
        }}
        tabSize={2}
        theme='one_dark'
        value={style}
      />
    </>
  );
};
