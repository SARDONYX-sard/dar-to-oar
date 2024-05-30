'use client';
import { Checkbox, FormControlLabel, Grid, Tooltip } from '@mui/material';
import InputLabel from '@mui/material/InputLabel';

import { CodeEditor } from '@/components/editor';
import { useInjectScript, useStorageState, useTranslation } from '@/hooks';

export type JsEditorProps = {
  editorMode: string;
  marginTop?: string;
};
export const JsEditor = ({ editorMode, marginTop }: JsEditorProps) => {
  const { t } = useTranslation();

  const [script, handleCodeChange] = useInjectScript();
  const [runScript, setRunScript] = useStorageState('runScript', 'false');

  return (
    <>
      <Grid
        container
        sx={{
          display: 'flex',
          justifyContent: 'space-evenly',
          width: '100%',
          marginTop: marginTop,
        }}
      >
        <InputLabel error sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
          {t('custom-js-label')}
        </InputLabel>
        <Tooltip
          title={
            <>
              {t('custom-js-auto-run-tooltip')}
              <br />
              {t('custom-js-auto-run-tooltip2')}
            </>
          }
        >
          <FormControlLabel
            control={
              <Checkbox
                checked={runScript === 'true'}
                onClick={() => {
                  const newValue = runScript === 'true' ? 'false' : 'true';
                  if (newValue === 'false') {
                    window.location.reload();
                  }
                  setRunScript(newValue);
                }}
              />
            }
            label={t('custom-js-auto-run-label')}
          />
        </Tooltip>
      </Grid>

      <CodeEditor
        defaultValue={script}
        height='280px'
        language={'javascript'}
        onChange={handleCodeChange}
        options={{
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
      <InputLabel id='status-node' sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }} />
    </>
  );
};
