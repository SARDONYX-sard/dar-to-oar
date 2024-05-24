/**
 * Examples of what to watch out for in other people's JavaScript (The safest way, of course, is not to do it.)
 * - Obfuscation by base64 encoding (base64 is not encryption)
 * - Accessing an external URL (fetch, XMLHttpRequest API)
 * - Those that force a URL change by assigning the URL to window.location
 */
(async () => {
  /**
   * - Q. Why is this dangerous?
   * - A. There is a possibility of downloading and accidentally executing dangerous binary files.
   * - If this pattern is abused, locations can be posted externally via the fetch API.
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
   * - Q. Why is this dangerous?
   * - A. It may be possible to download and execute malicious JavaScript.
   *
   * Reflect others' online Scripts in your own Script field.
   * @param {string} url
   */
  const overwriteYourScript = async (url) => {
    const res = await fetch(url);
    const blob = await res.blob();
    localStorage.setItem('customJS', await blob.text());
  };
  await overwriteYourScript(
    'https://raw.githubusercontent.com/SARDONYX-sard/dar-to-oar/0.5.0/test/sample_scripts/custom_translation.js',
  );
})();
