import CloseIcon from '@mui/icons-material/Close';
import { Box, Tab, Tabs, Tooltip } from '@mui/material';
import { useEditorContext } from '../context/editorContext';
import { useTranslation } from '@/components/hooks/useTranslation';

export const ClosableTabs = () => {
  const [state, dispatch] = useEditorContext();
  const { t } = useTranslation();

  return (
    <Tabs value={state.active} onChange={(_, v) => dispatch({ type: 'SET_ACTIVE', index: v })} variant='scrollable'>
      {state.tabs.map((tab, i) => (
        <Tooltip
          key={tab.id}
          title={
            <>
              <p>
                - {t('closable-tabs-dar')}: {tab.inputPath}
              </p>
              <p>
                - {t('closable-tabs-mapping-table')}: {tab.outputPath}
              </p>
            </>
          }
          enterDelay={2000}
        >
          <Tab
            key={tab.id}
            label={
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5, textTransform: 'none' }}>
                {tab.outputPath.split(/[\\/]/).pop()}
                <CloseIcon
                  sx={{ fontSize: 14 }}
                  onClick={(e) => {
                    e.stopPropagation();
                    dispatch({ type: 'CLOSE_TAB', index: i });
                  }}
                />
              </Box>
            }
          />
        </Tooltip>
      ))}
    </Tabs>
  );
};
