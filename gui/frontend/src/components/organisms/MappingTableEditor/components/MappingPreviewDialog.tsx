import { Dialog, DialogTitle, DialogContent, DialogActions, Button } from '@mui/material';
import { type MouseEventHandler } from 'react';
import { useEditorModeContext } from '../../../providers/EditorModeProvider';
import { MonacoEditor } from '../../MonacoEditor';
import { MONACO_OPTIONS } from './MappingTableEditor';
import { useTranslation } from '@/components/hooks/useTranslation';

export const MappingPreviewDialog = ({
  open,
  text,
  title,
  onApply,
  onAppend,
  onCancel,
}: {
  open: boolean;
  text: string;
  title?: string;
  onApply: MouseEventHandler<HTMLButtonElement>;
  onAppend?: MouseEventHandler<HTMLButtonElement>;
  onCancel: MouseEventHandler<HTMLButtonElement>;
}) => {
  const { editorMode } = useEditorModeContext();
  const { t } = useTranslation();

  return (
    <Dialog open={open} fullWidth maxWidth='md' onClose={onCancel}>
      <DialogTitle>{title || t('mapping-preview-default-title')}</DialogTitle>

      <DialogContent sx={{ height: '80vh' }}>
        <MonacoEditor
          value={text}
          language='mapping_table'
          height={'100%'}
          options={{
            ...MONACO_OPTIONS,
            readOnly: true,
          }}
          vimMode={editorMode === 'vim'}
        />
      </DialogContent>

      <DialogActions>
        <Button variant='contained' onClick={onApply}>
          {t('mapping-preview-apply')}
        </Button>

        {onAppend && (
          <Button variant='outlined' onClick={onAppend} title={t('mapping-preview-append-tooltip')}>
            {t('mapping-preview-append-missing')}
          </Button>
        )}

        <Button onClick={onCancel}>{t('mapping-preview-cancel')}</Button>
      </DialogActions>
    </Dialog>
  );
};
