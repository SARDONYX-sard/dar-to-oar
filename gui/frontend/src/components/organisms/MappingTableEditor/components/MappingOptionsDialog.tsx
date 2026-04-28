import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  FormControlLabel,
  Checkbox,
  Button,
  Box,
  Typography,
  Divider,
} from '@mui/material';
import { useEditorContext } from '../context/editorContext';
import { useTranslation } from '@/components/hooks/useTranslation';
import { OBJECT } from '@/lib/object-utils';

const OPTION_META_KEYS = {
  completion: {
    labelKey: 'mapping-options-completion',
    descriptionKey: 'mapping-options-completion-desc',
  },
  diagnostics: {
    labelKey: 'mapping-options-diagnostics',
    descriptionKey: 'mapping-options-diagnostics-desc',
  },
  formatter: {
    labelKey: 'mapping-options-formatter',
    descriptionKey: 'mapping-options-formatter-desc',
  },
  semanticTokens: {
    labelKey: 'mapping-options-semantic-tokens',
    descriptionKey: 'mapping-options-semantic-tokens-desc',
  },
  hover: {
    labelKey: 'mapping-options-hover',
    descriptionKey: 'mapping-options-hover-desc',
  },
  inlayHints: {
    labelKey: 'mapping-options-inlay-hints',
    descriptionKey: 'mapping-options-inlay-hints-desc',
  },
  signatureHelp: {
    labelKey: 'mapping-options-signature-help',
    descriptionKey: 'mapping-options-signature-help-desc',
  },
} as const;

export const MappingOptionsDialog = ({ open, onClose }: { open: boolean; onClose: () => void }) => {
  const [state, dispatch] = useEditorContext();
  const { t } = useTranslation();
  const options = state.options;

  const handleChange = (key: keyof typeof options) => (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch({
      type: 'SET_MAPPING_OPTIONS',
      options: { [key]: e.target.checked },
    });
  };

  return (
    <Dialog open={open} onClose={onClose} maxWidth='xs' fullWidth>
      <DialogTitle>{t('mapping-options-title')}</DialogTitle>

      <DialogContent dividers>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
          {OBJECT.entries(options).map(([key, value]) => {
            const metaKeys = OPTION_META_KEYS[key];

            return (
              <Box key={key}>
                <FormControlLabel
                  control={<Checkbox checked={value} onChange={handleChange(key)} />}
                  label={
                    <Box>
                      <Typography variant='body2'>{metaKeys ? t(metaKeys.labelKey) : key}</Typography>
                      <Typography variant='caption' color='text.secondary'>
                        {metaKeys ? t(metaKeys.descriptionKey) : ''}
                      </Typography>
                    </Box>
                  }
                />
                <Divider sx={{ mt: 1 }} />
              </Box>
            );
          })}
        </Box>
      </DialogContent>

      <DialogActions>
        <Button onClick={onClose}>{t('mapping-options-close')}</Button>
      </DialogActions>
    </Dialog>
  );
};
