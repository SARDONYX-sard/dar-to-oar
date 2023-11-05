import MenuItem from "@mui/material/MenuItem";
import Select, { type SelectChangeEvent } from "@mui/material/Select";
import type { EditorMode } from "@/utils/editor";
import { InputLabel, FormControl } from "@mui/material";
import { selectEditorMode } from "@/utils/editor";
import { useTranslation } from "react-i18next";

type Props = {
  setEditorMode: (value: EditorMode) => void;
  editorMode: EditorMode;
};

export const SelectEditorMode = ({ editorMode, setEditorMode }: Props) => {
  const { t } = useTranslation();
  const handleChange = (e: SelectChangeEvent<string>) => {
    const presetEditor = selectEditorMode(e.target.value);
    setEditorMode(presetEditor);
  };

  return (
    <FormControl variant="filled" sx={{ m: 1, minWidth: 100 }}>
      <InputLabel htmlFor="editor-select">
        {t("editor-mode-list-label")}
      </InputLabel>
      <Select
        name={editorMode}
        onChange={handleChange}
        label="Editor Mode"
        labelId="editor-select-label"
        id="editor-select"
        value={editorMode}
      >
        <MenuItem value={"default"}>Default</MenuItem>
        <MenuItem value={"vim"}>Vim</MenuItem>
      </Select>
    </FormControl>
  );
};
