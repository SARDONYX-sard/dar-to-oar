import InputLabel from '@mui/material/InputLabel';

import { CodeEditor } from '@/components/editor';
import { useTranslation } from '@/hooks';

export type CssEditorProps = {
  editorMode: string;
  setPreset: (script: string) => void;
  setStyle: (style: string) => void;
  style: string;
};

export const CssEditor = ({ editorMode, setPreset, setStyle, style }: CssEditorProps) => {
  const { t } = useTranslation();

  const handleCodeChange = (newValue: string | undefined) => {
    const value = newValue ?? '';
    setStyle(value);
    localStorage.setItem('customCSS', value);
    setPreset('0');
  };

  return (
    <>
      <InputLabel sx={{ marginTop: '20px' }}>{t('custom-css-label')}</InputLabel>
      <CodeEditor
        defaultValue={style}
        height='260px'
        language={'css'}
        onChange={handleCodeChange}
        options={{
          suggestOnTriggerCharacters: true,
          renderWhitespace: 'boundary',
          rulers: [120],
          hover: {
            above: true,
          },
        }}
        theme='vs-dark'
        vimMode={editorMode === 'vim'}
        width={'95%'}
      />
    </>
  );
};
