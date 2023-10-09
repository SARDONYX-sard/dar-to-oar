import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { useEffect } from "react";
import { useStorageState } from "@/hooks";

function selectPreset(select: string) {
  switch (select) {
    case "1":
    case "2":
      return select;
    default:
      return "0";
  }
}

type Props = {
  setStyle: (value: string) => void;
};

export const SelectStyleList = ({ setStyle }: Props) => {
  const [preset, setPreset] = useStorageState("presetNumber", "0");

  useEffect(() => {
    const presetNumber = selectPreset(
      localStorage.getItem("presetNumber") ?? ""
    );
    if (presetNumber === "0") {
      setStyle(localStorage.getItem("customCSS") ?? "");
    } else {
      setStyle(presetStyles[presetNumber]);
    }
  }, [setStyle]);

  return (
    <>
      <Select
        name={preset}
        onChange={(e) => {
          const presetNumber = selectPreset(e.target.value);
          setPreset(presetNumber);
          if (presetNumber === "0") {
            setStyle(localStorage.getItem("customCSS") ?? "");
            return;
          }
          setStyle(presetStyles[presetNumber]);
        }}
        labelId="style-select-label"
        id="style-select"
        value={preset}
      >
        <MenuItem value={"0"}>Yourself CSS</MenuItem>
        <MenuItem value={"1"}>Preset1</MenuItem>
        <MenuItem value={"2"}>Preset2</MenuItem>
      </Select>
    </>
  );
};

const presetStyles = {
  "1": `body {
    background-attachment: fixed;
    background-image: url("https://i.redd.it/red-forest-1920-1080-v0-s9u8ki2rr70a1.jpg?s=139edf608c428656505a143635a0687dec086229");
    background-repeat: no-repeat;
    background-size: cover;
    background-color: #000;
}

div:has(textarea),
.MuiOutlinedInput-root {
    background-color: #2424248c;
}

.MuiButton-outlined {
    color: #ff8e16;
    border-color: #ff8e16;
    background-color: #2424248c;
}
.MuiButton-outlined:hover {
    color: #fff;
    background-color: #ff89898b;
}

.MuiLoadingButton-root {
  color: #fff;
  background-color: #caaa6dc6;
}
.MuiLoadingButton-root:hover {
    background-color: #ff8e16;
}`,
  "2": `body {
    background-attachment: fixed;
    background-image: url("https://images.pexels.com/photos/2817421/pexels-photo-2817421.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750& dpr=1");
    background-repeat: no-repeat;
    background-size: cover;
    background-color: #000;
}

div:has(textarea),
.MuiButton-outlined,
.MuiOutlinedInput-root {
    background-color: #2424248c;
}

.MuiButton-outlined:hover {
    background-color: #0e0d0dc7;
}`,
};
