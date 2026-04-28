import { Box, Button } from '@mui/material';
import { useState, useCallback } from 'react';
import { useEditorContext } from '../context/editorContext';
import { useTabEditorCommands } from '../hooks/useTabEditorCommands';
import { MappingPreviewDialog } from './MappingPreviewDialog';
import { useTranslation } from '@/components/hooks/useTranslation';

/** Top toolbar with save and preview controls */
export const EditToolbar = () => {
  const [state, dispatch] = useEditorContext();
  const { saveCurrent } = useTabEditorCommands();
  const { t } = useTranslation();

  const [confirmOpen, setConfirmOpen] = useState(false);
  const tab = state.tabs[state.active];

  const hasTab = Boolean(tab);

  const handleSaveClick = useCallback(async () => {
    const selectedPath = await saveCurrent();
    if (selectedPath) {
      dispatch({ type: 'UPDATE_OUTPUT', outputPath: selectedPath });
    }
  }, [saveCurrent, dispatch]);

  const handleRevertClick = useCallback(() => {
    setConfirmOpen(true);
  }, []);

  const handleConfirm = useCallback(() => {
    dispatch({ type: 'REVERT_ACTIVE_TAB' });
    setConfirmOpen(false);
  }, [dispatch]);

  const handleCancel = useCallback(() => {
    setConfirmOpen(false);
  }, []);

  return (
    <>
      <Box
        sx={{
          display: 'flex',
          alignItems: 'center',
          gap: 1,
          px: 1,
          py: 0.5,
          borderBottom: '1px solid #333',
        }}
      >
        <Button variant='contained' size='small' disabled={!hasTab} onClick={handleSaveClick}>
          {t('edit-toolbar-save')}
        </Button>

        <Button variant='outlined' size='small' disabled={!hasTab} onClick={handleRevertClick}>
          {t('edit-toolbar-revert')}
        </Button>
      </Box>

      <MappingPreviewDialog
        open={confirmOpen}
        title={t('revert-preview-title')}
        text={tab?.originalText || ''}
        onApply={handleConfirm}
        onCancel={handleCancel}
      />
    </>
  );
};
