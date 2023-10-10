import { InputLabel, FormControl } from "@mui/material";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { selectEditorMode } from "../utils/editor";
import type { EditorMode } from "@/utils/editor";

type Props = {
  setEditorMode: (value: EditorMode) => void;
  editorMode: EditorMode;
};

export const SelectEditorMode = ({ editorMode, setEditorMode }: Props) => {
  return (
    <FormControl>
      <InputLabel htmlFor="editor-select">Editor Mode</InputLabel>
      <Select
        name={editorMode ?? "default"}
        onChange={(e) => {
          const presetEditor = selectEditorMode(e.target.value);
          setEditorMode(presetEditor);
        }}
        label="Editor Mode"
        labelId="editor-select-label"
        id="editor-select"
        value={editorMode ?? "default"}
      >
        <MenuItem value={"default"}>Default</MenuItem>
        <MenuItem value={"vim"}>Vim</MenuItem>
      </Select>
    </FormControl>
  );
};
