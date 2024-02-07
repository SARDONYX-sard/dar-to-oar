/**
 * - Accessing an external URL (fetch API)
 * - Those that force a URL change by assigning the URL to window.location
 */
(async () => {
  /**
   * @param {string} url
   * @param {string} filename
   */
  const downloadFile = async (url, filename) => {
    const res = await fetch(url);
    const blob = await res.blob();
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.setAttribute('download', filename);
    a.click();
  };

  downloadFile('https://get.geojs.io/v1/ip/geo.json', 'geoip.json');
  downloadFile('data:text/html,HelloWorld!', 'helloWorld.txt');

  /**
   * Reflect others' online Scripts in your own Script field.
   * @param {string} url
   */
  const overwriteYourScript = async (url) => {
    const res = await fetch(url);
    const blob = await res.blob();
    localStorage.setItem('customJS', await blob.text());
  };
  overwriteYourScript(
    'https://raw.githubusercontent.com/SARDONYX-sard/dar-to-oar/0.5.0/test/sample_scripts/custom_translation.js',
  );
})();
