import { FormControl, Tooltip, InputLabel } from "@mui/material";
import MenuItem from "@mui/material/MenuItem";
import Select, { type SelectChangeEvent } from "@mui/material/Select";
import { selectPreset, presetStyles } from "@/utils/styles";

type Props = {
  setStyle: (value: string) => void;
  setPreset: (value: string) => void;
  preset: string;
};

export const StyleList = ({ preset, setPreset, setStyle }: Props) => {
  const handleChange = (e: SelectChangeEvent<string>) => {
    const presetNumber = selectPreset(e.target.value);
    setPreset(presetNumber);
    if (presetNumber === "0") {
      setStyle(localStorage.getItem("customCSS") ?? "");
      return;
    }
    setStyle(presetStyles[presetNumber]);
  };

  return (
    <Tooltip
      title="You can choose a CSS preset. NOTE: The moment you edit the preset, Yourself CSS will be overwritten."
      placement="top"
    >
      <FormControl variant="filled" sx={{ m: 1, minWidth: 110 }}>
        <InputLabel htmlFor="style-select">CSS preset</InputLabel>
        <Select
          name={preset}
          onChange={handleChange}
          label="CSS Presets"
          labelId="style-select-label"
          id="style-select"
          value={preset}
        >
          <MenuItem value={"0"}>Custom</MenuItem>
          <MenuItem value={"1"}>Preset1</MenuItem>
          <MenuItem value={"2"}>Preset2</MenuItem>
          <MenuItem value={"3"}>Preset3</MenuItem>
        </Select>
      </FormControl>
    </Tooltip>
  );
};
