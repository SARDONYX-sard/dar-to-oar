import { TabContext, TabList, type TabListProps } from '@mui/lab';
import { Box, type SxProps, Tab, type Theme } from '@mui/material';

import { useStorageState } from '@/components/hooks/useStorageState/useStorageState';
import { useTranslation } from '@/components/hooks/useTranslation';
import { useCssContext } from '@/components/providers/CssProvider';
import { useJsContext } from '@/components/providers/JsProvider';
import { CSS_PRESETS } from '@/lib/css';

import { type EditorInfo, EditorInitializer } from './EditorInitializer';

export const CodeEditor = () => {
  const { js, setJs } = useJsContext();
  const { css, setCss, setPreset } = useCssContext();
  const { t } = useTranslation();

  const [value, setValue] = useStorageState<'javascript' | 'css'>('editor-tab-select', 'javascript');
  const handleTabChange: TabListProps['onChange'] = (_event, newValue) => {
    setValue(newValue);
  };

  const editorValues = {
    css: {
      value: css,
      fileName: 'style.css',
      label: t('custom-css-label'),
      language: 'css',
      onChange: (value) => {
        setCss(value);
        CSS_PRESETS.setCss(value ?? '');
        setPreset('0');
      },
    },

    javascript: {
      value: js,
      fileName: 'script.js',
      label: t('custom-js-label'),
      language: 'javascript',
      onChange: setJs,
    },
  } as const satisfies EditorInfo;

  const editorValue = editorValues[value];
  const labelSx = { textTransform: 'capitalize' } as const satisfies SxProps<Theme>;

  return (
    <TabContext value={value}>
      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <TabList aria-label='Setting tabs' onChange={handleTabChange}>
          <Tab label='CSS' value='css' />
          <Tab label='JavaScript' sx={labelSx} value='javascript' />
        </TabList>
      </Box>

      <EditorInitializer
        fileName={editorValue.fileName}
        label={editorValue.label}
        language={editorValue.language}
        onChange={editorValue.onChange}
        value={editorValue.value}
      />
    </TabContext>
  );
};
