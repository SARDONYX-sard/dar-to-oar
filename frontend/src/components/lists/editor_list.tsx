import { InputLabel, FormControl } from '@mui/material';
import MenuItem from '@mui/material/MenuItem';
import Select, { type SelectChangeEvent } from '@mui/material/Select';
import { useCallback } from 'react';

import { useTranslation } from '@/hooks';
import type { EditorMode } from '@/utils/selector';
import { selectEditorMode } from '@/utils/selector';

type Props = {
  setEditorMode: (value: EditorMode) => void;
  editorMode: EditorMode;
};

export const SelectEditorMode = ({ editorMode, setEditorMode }: Props) => {
  const { t } = useTranslation();

  const handleChange = useCallback(
    (e: SelectChangeEvent<string>) => {
      const presetEditor = selectEditorMode(e.target.value);
      setEditorMode(presetEditor);
    },
    [setEditorMode],
  );

  return (
    <FormControl variant="filled" sx={{ m: 1, minWidth: 100 }}>
      <InputLabel htmlFor="editor-select">{t('editor-mode-list-label')}</InputLabel>
      <Select
        name={editorMode}
        onChange={handleChange}
        label="Editor Mode"
        labelId="editor-select-label"
        id="editor-select"
        value={editorMode}
      >
        <MenuItem value={'default'}>Default</MenuItem>
        <MenuItem value={'vim'}>Vim</MenuItem>
      </Select>
    </FormControl>
  );
};
