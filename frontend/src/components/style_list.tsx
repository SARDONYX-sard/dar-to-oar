import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { useState } from "react";

function selectSample(select: string) {
  switch (select) {
    case "1":
    case "2":
      return select;
    default:
      return "unknown";
  }
}

type Props = {
  setStyle: (value: string) => void;
};

export const SelectStyleList = ({ setStyle }: Props) => {
  const [sample, setSample] = useState("unknown");

  return (
    <>
      <Select
        name={sample}
        onChange={(e) => {
          const style = selectSample(e.target.value);
          setSample(e.target.value);
          if (style === "unknown") {
            return;
          }
          setStyle(sampleStyles[style]);
        }}
        labelId="style-select-label"
        id="style-select"
        value={sample}
      >
        <MenuItem value={"unknown"}>Yourself CSS</MenuItem>
        <MenuItem value={"1"}>Preset1</MenuItem>
        <MenuItem value={"2"}>Preset2</MenuItem>
      </Select>
    </>
  );
};

const sampleStyles = {
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
