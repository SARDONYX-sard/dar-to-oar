//@ts-check
(() => {
  const RED_THEME = `
  --mui-palette-primary-main: #ff8e16;
  --mui-palette-error-main: #ff1655;
  --mui-palette-background-default: #2223;
  --mui-palette-secondary-main: #ab2b7e6e;
  --mui-filled-input-background: #2424248c;
  --mui-focus-visible: #c9623db3;
  --mui-palette-primary-dark: #cd2c6c95;

  --autofill-color: #691c3747;
  --convert-btn-color: #ab2b7e6e;
  --hover-btn-color: #c9623db3;
  --hover-convert-btn-color: #cd2c6c95;
  `;
  const BLUE_THEME = `
  --mui-palette-primary-main: #5a9ab9;
  --mui-palette-background-default: #2226;
  --mui-palette-secondary-main: #0288d162;
  --mui-filled-input-background: #26323866;
  --mui-focus-visible: #01579b80;
  --mui-palette-primary-dark: #1c2c3a;

  --autofill-color: #5eb1ef24;
  --convert-btn-color: #3369ad7d;
  --hover-btn-color: #1d5aa58b;
  --hover-convert-btn-color: #2665b5d1;
  --main-bg-color: #2222228e;
  `;
  // oxlint-disable-next-line no-unused-vars
  const VIOLET_THEME = `
  --mui-palette-primary-main:#9644f1;
  --mui-palette-error-main: #9c27b0;
  --mui-palette-background-default: #2225;
  --mui-palette-secondary-main: #4a148c5c;
  --mui-filled-input-background: #3c003fa8;
  --mui-focus-visible: #673ab7c2;
  --mui-palette-primary-dark: #330066;

  --autofill-color: #eb37ff1c;
  --convert-btn-color: #ab2b7e6e;
  --hover-btn-color: #8b51fb8b;
  --hover-convert-btn-color: #7d00c9a3;
  --main-bg-color: #2223;
  `;
  const GREEN_THEME = `
  --mui-palette-primary-main: rgb(185, 185, 90);
  --mui-palette-error-main: #ff5722;
  --mui-palette-background-default: #2020208a;
  --mui-palette-secondary-main: #64dd178a;
  --mui-filled-input-background: #31313161;
  --mui-focus-visible: #6d6d6d94;
  --mui-palette-primary-dark: #3dcb5e;

  --autofill-color: #a19c0038;
  --convert-btn-color: #94ce7c6e;
  --hover-btn-color:rgba(161, 196, 66, 0.31);
  --hover-convert-btn-color: #81c462a3;
  --main-bg-color: #222a;
  `;

  /**
   * @typedef {typeof RED_THEME|typeof BLUE_THEME|typeof VIOLET_THEME|typeof GREEN_THEME} PresetType
   *
   * @typedef {{ url: string, preset: PresetType }} BgEntry
   */

  /**
   * Generates a random image URL and selects a corresponding theme preset.
   * @returns A containing the image URL and the CSS preset.
   */
  const pexelsEntries = () => {
    const IMG_NUM_LIST = /** @type {const} */ ([1543801, 1547163, 4589833, 7325003, 14133639]);
    return IMG_NUM_LIST.map((imgNum) => {
      /** @type {PresetType} */
      let preset = BLUE_THEME;
      if ([1547163, 14133639].includes(imgNum)) {
        preset = RED_THEME;
      } else if ([4589833, 7325003].includes(imgNum)) {
        preset = GREEN_THEME;
      }

      return /** @type {const} @satisfies {BgEntry} */ ({
        url: `https://images.pexels.com/photos/${imgNum}/pexels-photo-${imgNum}.jpeg`,
        preset,
      });
    });
  };

  /** Add any extra URLs + presets here. */
  const URLS = /** @type {const} @satisfies {BgEntry[]} */ ([
    ...pexelsEntries(),
    {
      url: 'https://images.unsplash.com/photo-1506744038136-46273834b3fb',
      preset: BLUE_THEME,
    },
    // { url: 'https://i.redd.it/red-forest-...', preset: preset1 },
  ]);

  const getRandom = () => URLS[Math.floor(Math.random() * URLS.length)];

  /** @param {BgEntry} entry */
  const applyCss = ({ url, preset }) => {
    const commonVariables = `
        --image-position-x: center;
        --image-position-y: center;
        --image-size: cover;
        --main-bg-color: #222a;
        ${preset}
`;
    const style = document.getElementById('dyn-style') ?? document.createElement('style');
    style.id = 'dyn-style';
    style.innerHTML = `:root { ${commonVariables} --image-url: url('${url}'); }`;
    document.body.appendChild(style);
  };

  applyCss(getRandom());
})();
