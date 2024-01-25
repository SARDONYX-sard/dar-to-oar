/**
 * Creates a CSS preset.
 *
 * @template T - The type of the CSS string.
 * @param {T} css - The CSS string.
 * @returns {`${T} as const`} - The CSS preset as a readonly string literal type.
 */
function createPreset<T extends string>(css: T) {
  return /* css */ `:root {
  ${css}
}

body {
  background-attachment: fixed;
  background-color: #000;
  background-image: var(--image-url);
  background-position-x: var(--image-position-x);
  background-position-y: var(--image-position-y);
  background-repeat: no-repeat;
  background-size: var(--image-size);
}

main {
  background-color: var(--main-bg-color);
}

:-webkit-autofill {
  box-shadow: var(--autofill-color) 0px 0px 0px 100px inset !important;
}

.ace_gutter,
.MuiOutlinedInput-root {
  background-color: #2424248c !important;
}

p.Mui-error {
  background-color: #2f2e2eba;
  color: var(--error-color);
}

main:has(.ace_editor) > .MuiInputLabel-animated {
  background-color: #2f2e2eba;
  padding-left: 1rem !important;
  padding-right: 1rem !important;
}

a,
span.Mui-checked > svg,
.MuiInputBase-root.MuiOutlinedInput-root.MuiInputBase-colorPrimary.Mui-focused > fieldset,
.Mui-selected, /* Bottom Navigation */
.MuiButton-outlined {
  color: var(--theme-color) !important;
  border-color: var(--theme-color);
}

label.Mui-focused,
.MuiButton-root.MuiButton-text,
.MuiCircularProgress-svg {
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
}
` as const;
}

const preset1 = createPreset(
  `--autofill-color: #691c3747;
  --convert-btn-color: #ab2b7e6e;
  --error-color: #ff1655;
  --hover-btn-color: #ff89898b;
  --hover-convert-btn-color: #fd3b3b6e;
  --image-size: cover;
  --image-url: url("https://i.redd.it/red-forest-1920-1080-v0-s9u8ki2rr70a1.jpg?s=139edf608c428656505a143635a0687dec086229");
  --main-bg-color: #2223;
  --theme-color: #ff8e16;` as const,
);

const preset2 = createPreset(
  `--autofill-color: #5eb1ef24;
  --convert-btn-color: #3369ad7d;
  --hover-btn-color: #1d5aa58b;
  --hover-convert-btn-color: #2665b5d1;
  --image-size: cover;
  --image-url: url("https://images.pexels.com/photos/2817421/pexels-photo-2817421.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750& dpr=1");
  --main-bg-color: #2223;
  --theme-color: #5a9ab9;` as const,
);

const preset3 = createPreset(
  `--autofill-color: #eb37ff1c;
  --convert-btn-color: #ab2b7e6e;
  --hover-btn-color: #8b51fb8b;
  --hover-convert-btn-color: #7d00c9a3;
  --image-size: cover;
  --image-url: url("https://images.pexels.com/photos/6162265/pexels-photo-6162265.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1");
  --main-bg-color: #2223;
  --theme-color: #9644f1;` as const,
);

const preset4 = createPreset(
  `--autofill-color: #a19c0038;
  --convert-btn-color: #94ce7c6e;
  --hover-btn-color: #cefb518b;
  --hover-convert-btn-color: #81c462a3;
  --image-position-x: center;
  --image-position-y: center;
  --image-url: url('https://images.pexels.com/photos/973324/pexels-photo-973324.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1');
  --main-bg-color: #222a;
  --theme-color: rgb(185, 185, 90);` as const,
);

export const presetStyles = {
  '1': preset1,
  '2': preset2,
  '3': preset3,
  '4': preset4,
} as const;

export function selectPreset(select: string) {
  switch (select) {
    case '1':
    case '2':
    case '3':
    case '4':
      return select;
    default:
      return '0';
  }
}
