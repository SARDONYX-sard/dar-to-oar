import AddIcon from '@mui/icons-material/Add';
import { Box, Button, IconButton, Tooltip } from '@mui/material';
import { useState } from 'react';
import { useTranslation } from '../../../hooks/useTranslation';
import { useTabEditorCommands } from '../hooks/useTabEditorCommands';
import { ClosableTabs } from './ClosableTabs';
import { MappingOptionsDialog } from './MappingOptionsDialog';

export const TopBar = () => {
  const { handleOpenClick, handleNewClick } = useTabEditorCommands();
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);

  return (
    <Box
      sx={{
        display: 'flex',
        alignItems: 'center',
        px: 1,
        borderBottom: '1px solid #333',
        bgcolor: '#1e1e1e',
      }}
    >
      <ClosableTabs />

      <IconButton
        size='small'
        onClick={handleNewClick}
        sx={{
          ml: 0.5,
          color: '#ccc',
          '&:hover': { color: '#fff' },
        }}
      >
        <AddIcon fontSize='small' />
      </IconButton>

      <Box sx={{ flexGrow: 1 }} />

      <Button variant='outlined' color='primary' size='small' onClick={handleOpenClick}>
        {t('select-btn')}
      </Button>

      <Tooltip title={t('mapping-options-title')}>
        <IconButton onClick={() => setOpen(true)}>⚙</IconButton>
      </Tooltip>

      <MappingOptionsDialog open={open} onClose={() => setOpen(false)} />
    </Box>
  );
};
