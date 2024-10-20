//@ts-check

(() => {
  const preset1 = `
  --autofill-color: #691c3747;
  --convert-btn-color: #ab2b7e6e;
  --error-color: #ff1655;
  --hover-btn-color: #c9623db3;
  --hover-convert-btn-color: #cd2c6c95;
  --main-bg-color: #2223;
  --theme-color: #ff8e16;`;
  const preset2 = `
  --autofill-color: #5eb1ef24;
  --convert-btn-color: #3369ad7d;
  --hover-btn-color: #1d5aa58b;
  --hover-convert-btn-color: #2665b5d1;
  --theme-color: #5a9ab9;`;
  const preset4 = `
  --autofill-color: #a19c0038;
  --convert-btn-color: #94ce7c6e;
  --hover-btn-color: #cefb518b;
  --hover-convert-btn-color: #81c462a3;
  --main-bg-color: #222a;
  --theme-color: rgb(185, 185, 90);`;

  let preset = preset2;
  const getRandomUrl = () => {
    const imgNumList = [1543801, 1547163, 4589833, 7325003, 14133639];

    const imgNum = imgNumList[Math.floor(Math.random() * imgNumList.length)];
    if ([1547163, 14133639].includes(imgNum)) {
      preset = preset1;
    } else if ([4589833, 7325003].includes(imgNum)) {
      preset = preset4;
    }
    return `https://images.pexels.com/photos/${imgNum}/pexels-photo-${imgNum}.jpeg`;
  };

  // Change the background on each page.(JS is executed every time, so the same variable is fine.)
  dynImg(getRandomUrl(), preset);

  /**
   * Change the background on each page.
   * @param {string} imgUrl - Image URI
   * @param {string} preset - CSS
   */
  function dynImg(imgUrl, preset) {
    const commonVariables = `
        --image-position-x: center;
        --image-position-y: center;
        --image-size: cover;
        --main-bg-color: #222a;
        ${preset}
`;
    const style = document.getElementById('dyn-style') ?? document.createElement('style');
    style.id = 'dyn-style';
    style.innerHTML = `:root { ${commonVariables} --image-url: url('${imgUrl}'); }`;
    document.body.appendChild(style);
  }
})();
