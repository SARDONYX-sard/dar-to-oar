import { Checkbox, FormControlLabel, Grid, Tooltip } from '@mui/material';
import InputLabel from '@mui/material/InputLabel';
import AceEditor from 'react-ace';

import { useInjectScript, useStorageState, useTranslation } from '@/hooks';
import { selectEditorMode } from '@/utils/selector';

export type JsEditorProps = {
  editorMode: string;
  marginTop?: string;
};
export const JsEditor = ({ editorMode, marginTop }: JsEditorProps) => {
  const { t } = useTranslation();

  const [script, setScript] = useInjectScript();
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

      <AceEditor
        editorProps={{ $blockScrolling: true }}
        enableBasicAutocompletion
        enableLiveAutocompletion
        enableSnippets
        fontSize={'1rem'}
        height='250px'
        highlightActiveLine
        keyboardHandler={selectEditorMode(editorMode)}
        mode='javascript'
        name='Custom JavaScript'
        onChange={(value) => {
          localStorage.setItem('customJS', value);
          setScript(value);
        }}
        placeholder={`(()=> {
    const p = document.createElement('p');
    p.innerText = 'Hello';
    document.body.appendChild(p);
)()`}
        setOptions={{ useWorker: false }}
        style={{
          width: '95%',
          backgroundColor: '#2424248c',
        }}
        tabSize={2}
        theme='one_dark'
        value={script}
      />
    </>
  );
};
