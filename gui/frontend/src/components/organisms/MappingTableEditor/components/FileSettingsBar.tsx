import { Box, TextField } from '@mui/material';
import { useEditorContext } from '../context/editorContext';
import { InputField } from './InputField';
import { useTranslation } from '@/components/hooks/useTranslation';
import { OpenIcon } from '@/components/organisms/OpenIcon/OpenIcon';

/** Output path and format settings bar */
export const FileSettingsBar = () => {
  const [state, dispatch] = useEditorContext();
  const { t } = useTranslation();
  const tab = state.tabs[state.active];

  if (!tab) return null;

  return (
    <Box
      sx={{
        display: 'flex',
        alignItems: 'center',
        px: 2,
        py: 1,
        borderBottom: '1px solid #444',
      }}
    >
      <div style={{ display: 'flex', flexDirection: 'column', width: '100%' }}>
        <InputField />

        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            gap: 1,
            width: '100%',
          }}
        >
          <OpenIcon path={tab.outputPath} />
          <TextField
            label={t('file-settings-output-path')}
            size='small'
            fullWidth
            variant='standard'
            value={tab.outputPath}
            onChange={(e) =>
              dispatch({
                type: 'UPDATE_OUTPUT',
                outputPath: e.target.value,
              })
            }
          />
        </Box>
      </div>
    </Box>
  );
};
