function createPreset() {
  return `body {
  background-attachment: fixed;
  background-image: var(--image-url);
  background-repeat: no-repeat;
  background-size: cover;
  background-color: #000;
}

.ace_gutter,
.MuiOutlinedInput-root {
  background-color: #2424248c !important;
}

p.Mui-error {
  color: var(--error-color);
  background-color: #2f2e2eba;
}

main:has(.ace_editor) > .MuiInputLabel-animated {
  padding-right: 1rem !important;
  padding-left: 1rem !important;
  background-color: #2f2e2eba;
}

a,
span.Mui-checked > svg,
.MuiInputBase-root.MuiOutlinedInput-root.MuiInputBase-colorPrimary.Mui-focused > fieldset,
.Mui-selected, /* Bottom Navigation */
.MuiButton-outlined {
  color: var(--theme-color) !important;
  border-color: var(--theme-color);
}

label.Mui-focused {
  color: var(--theme-color) !important;
}

.MuiButton-outlined {
  background-color: #2424248c;
}

.MuiButton-outlined:hover {
  color: #fff !important;
  background-color: var(--hover-btn-color);
}

.MuiLoadingButton-root {
  color: #fff;
  background-color: var(--convert-btn-color);
}

.MuiLoadingButton-root:hover {
  background-color: var(--hover-convert-btn-color);
}

.MuiLinearProgress-bar {
  background-color: var(--theme-color);
}`;
}

const preset1 = `:root {
  --theme-color: #ff8e16;
  --hover-btn-color: #ff89898b;
  --convert-btn-color: #ab2b7e6e;
  --hover-convert-btn-color: #fd3b3b6e;
  --error-color: #ff1655;
  --image-url: url("https://i.redd.it/red-forest-1920-1080-v0-s9u8ki2rr70a1.jpg?s=139edf608c428656505a143635a0687dec086229")
}

${createPreset()}`;

const preset2 = `:root {
  --image-url: url("https://images.pexels.com/photos/2817421/pexels-photo-2817421.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750& dpr=1");
}

body {
  background-attachment: fixed;
  background-image: var(--image-url);
  background-repeat: no-repeat;
  background-size: cover;
  background-color: #000;
}

.ace_gutter,
.MuiOutlinedInput-root {
  background-color: #2424248c !important;
}

p.Mui-error {
  color: var(--error-color);
  background-color: #2f2e2eba;
}

main:has(.ace_editor) > .MuiInputLabel-animated {
  padding-right: 1rem !important;
  padding-left: 1rem !important;
  background-color: #2f2e2eba;
}

.MuiButton-outlined {
  background-color: #2424248c;
}`;

const preset3 = `:root {
  --theme-color: #9644f1;
  --hover-btn-color: #8b51fb8b;
  --convert-btn-color: #ab2b7e6e;
  --hover-convert-btn-color: #fd3b3b6e;
  --image-url: url("https://images.pexels.com/photos/6162265/pexels-photo-6162265.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1");
}

${createPreset()}`;

export const presetStyles = {
  "1": preset1,
  "2": preset2,
  "3": preset3,
} as const;

export function selectPreset(select: string) {
  switch (select) {
    case "1":
    case "2":
    case "3":
      return select;
    default:
      return "0";
  }
}
