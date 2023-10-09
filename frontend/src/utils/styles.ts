const preset1 = `body {
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

.Mui-error {
  color: #ff1655;
  background-color: #2f2e2eba;
}

a,
span.Mui-checked > svg,
.MuiInputBase-root.MuiOutlinedInput-root.MuiInputBase-colorPrimary.Mui-focused > fieldset,
.Mui-selected, /* Bottom Navigation */
.MuiButton-outlined {
  color: #ff8e16 !important;
  border-color: #ff8e16;
}


label.Mui-focused {
  color: #ff8e16 !important;
}

.MuiButton-outlined {
  background-color: #2424248c;
}

.MuiButton-outlined:hover {
  color: #fff !important;
  background-color: #ff89898b;
}

.MuiLoadingButton-root {
  color: #fff;
  background-color: #ab2b7e6e;
}

.MuiLoadingButton-root:hover {
  background-color: #fd3b3b6e;
}`;

const preset2 = `body {
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
}`;

export const presetStyles = {
  "1": preset1,
  "2": preset2,
} as const;

export function selectPreset(select: string) {
  switch (select) {
    case "1":
    case "2":
      return select;
    default:
      return "0";
  }
}
