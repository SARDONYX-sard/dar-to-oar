//@ts-check

(() => {
  // - Custom Transaction
  //   How to apply?
  //    1. Use comment-out toggle key: Ctrl + /
  //    2. Apply by twice reload: (Ctrl + Shift + r) * 2
  //    3. Use comment-out toggle key again: Ctrl + /
  setCustomTranslation();
  // clearCustomTranslation();

  function setCustomTranslation() {
    /** ref: https://github.com/SARDONYX-sard/dar-to-oar/blob/0.9.0/locales/en-US.json */
    const i18n = {
      'all-clear-btn': 'All Clear',
      'backup-dialog-all-checked-label': 'Check all',
      'backup-dialog-pub-checked-label': 'Check all public',
      'backup-export-btn-name': 'Export',
      'backup-export-dialog-title': 'Export Settings',
      'backup-export-success': 'Settings exported.',
      'backup-export-tooltip':
        'Export current settings.(Please be careful to handle personal information when transferring to others.)',
      'backup-import-btn-name': 'Import',
      'backup-import-dialog-title': 'Import Settings',
      'backup-import-tooltip':
        "Import settings from Json file.(JavaScript is also executed at the moment of import! If it is someone else's file, please be careful that the JavaScript is not malicious.)",
      'cancel-btn': 'Cancel',
      'conversion-complete': 'Conversion Complete.',
      'convert-btn': 'Convert',
      'convert-form-author-name': 'Mod Author Name',
      'convert-form-author-name-helper': '[Optional]',
      'convert-form-author-placeholder': 'Name',
      'convert-form-dar-helper': '[Required] Path of dir containing "DynamicAnimationReplacer".',
      'convert-form-dar-helper2': '"C:/[...]/Mod Name/" -> Convert 1st & 3rd person',
      'convert-form-dar-helper3': '"[...]/animations/DynamicAnimationReplacer" -> Convert 3rd person',
      'convert-form-dar-label': 'DAR(source) Directory',
      'convert-form-mapping-1st-label': 'Mapping Table Path(For _1st_person)',
      'convert-form-mapping-help-link-name': 'What is the mapping file?',
      'convert-form-mapping-helper': '[Optional] File path that helps map priority number to a section name.',
      'convert-form-mapping-helper2': 'Help: ',
      'convert-form-mapping-label': 'Mapping Table Path',
      'convert-form-mod-name': 'Mod Name',
      'convert-form-mod-name-helper': '[Optional]',
      'convert-form-oar-helper':
        '[Optional] Creates a OAR path in specified directory.(e.g. "NewMod" -> "NewMod/meshes/[...])"',
      'convert-form-oar-helper2': 'If not specified, an OAR is created at the same level as the DAR.',
      'convert-form-oar-label': 'OAR(destination) Directory',
      'converting-btn': 'Converting...',
      'css-preset-list-item0': 'Custom',
      'css-preset-list-item1': 'Preset1',
      'css-preset-list-item2': 'Preset2',
      'css-preset-list-item3': 'Preset3',
      'css-preset-list-item4': 'Preset4',
      'css-preset-list-label': 'CSS Preset',
      'css-preset-list-tooltip': 'You can choose a CSS preset.',
      'css-preset-list-tooltip2': 'Note: Editing "Preset" will overwrite "Custom".',
      'custom-css-label': 'Currently applied CSS',
      'custom-js-auto-run-label': 'JS Auto run',
      'custom-js-auto-run-tooltip':
        'Automatically run JavaScript on every page transition.(If disabled, it will automatically reload to apply the settings.)',
      'custom-js-auto-run-tooltip2':
        'This configuration item will not be activated unless manually selected by the user.',
      'custom-js-label': 'JavaScript to be executed each time you move pages (only when you give permission to do so)',
      'editor-mode-list-label': 'Editor Mode',
      'hide-dar-btn': 'Hide DAR',
      'hide-dar-btn-tooltip': 'After conversion, add ".mohidden" to all DAR files to hide them.(For MO2 user)',
      'hide-dar-btn-tooltip2': 'INFO: This will be effective when the destination of the OAR output is not specified.',
      'import-lang-btn': 'Import Language',
      'import-lang-tooltip': 'Import any language from a Json file. (automatically reloads for validation).',
      'import-lang-tooltip2': 'Note: For invalid Json, fall back to English. (See Wiki for how to write Json)',
      'infer-btn': 'Infer',
      'infer-btn-tooltip':
        'Infer OAR and ModName from DAR (input). (Even if each item is not entered without this function, it is inferred to some extent on the back-end side.)',
      'lang-preset-auto': 'Auto',
      'lang-preset-custom': 'Custom',
      'lang-preset-label': 'Language',
      'log-level-list-label': 'Log Level',
      'log-level-list-tooltip': 'Minor log level encompasses the more critical log levels. (i.e. Error ⊂ Info)',
      'log-level-list-tooltip2': 'Debug: Logs data on the way of the converted condition.',
      'log-level-list-tooltip3': ' Info: Log the conversion time.',
      'log-level-list-tooltip4': 'Error: Logs nothing but errors.',
      'mapping-wiki-url-leaf': 'wiki#what-is-the-mapping-file',
      'notice-limit': 'Limit',
      'notice-position-bottom-center': 'Bottom Center',
      'notice-position-bottom-left': 'Bottom Left',
      'notice-position-bottom-right': 'Bottom Right',
      'notice-position-list-label': 'Notice Position',
      'notice-position-top-center': 'Top Center',
      'notice-position-top-left': 'Top Left',
      'notice-position-top-right': 'Top Right',
      'open-log-btn': 'Open log',
      'open-log-dir-btn': 'Log(dir)',
      'open-log-dir-tooltip': 'Open the log storage location.',
      'open-log-tooltip': 'Open current log file.(Rotate to a new log file each time the application is launched.)',
      'progress-btn': 'Progress',
      'progress-btn-tooltip': 'Let the back-end report detailed progress.',
      'progress-btn-tooltip2': '(conversion may be slightly slower)',
      'remove-oar-btn': 'Remove OAR',
      'remove-oar-failed': 'Not found "OpenAnimationReplacer" directory',
      'remove-oar-specify-error': 'DAR or OAR dir must be specified.',
      'remove-oar-success': 'Removed OAR directory.',
      'remove-oar-tooltip':
        'Find and delete OAR dir from "OAR(destination) Directory"(or "DAR(source) Directory*" if not specified).',
      'run-parallel-btn-tooltip': 'Attempt file-by-file parallel conversion.',
      'run-parallel-btn-tooltip2':
        'Pros: extremely fast conversion / Cons: entries in logs are out of order and difficult to read',
      'run-parallel-label': 'Parallel',
      'select-btn': 'Select',
      'tab-label-backup': 'Backup',
      'tab-label-editor': 'Editor / Preset',
      'tab-label-lang': 'Language',
      'tab-label-notice': 'Notice',
      'tab-label-tab': 'Tab',
      'tab-list-position-bottom': 'Bottom',
      'tab-list-position-label': 'Tab Position',
      'tab-list-position-top': 'Top',
      'unhide-dar-btn': 'Unhide DAR',
      'unhide-dar-btn-tooltip': 'Unhide DAR files hidden by "Hide DAR".(For MO2 user)',
      'unhide-dar-failed': 'Could not find files with ".mohidden" extension',
      'unhide-dar-specify-error': 'DAR dir must be specified.',
      'unhide-dar-success': 'Redisplay of DAR directory.',
    };

    localStorage.setItem('custom-translation-dict', JSON.stringify(i18n));
    localStorage.setItem('locale', 'custom');
  }

  // biome-ignore lint/correctness/noUnusedVariables: ignore
  function clearCustomTranslation() {
    localStorage.removeItem('custom-translation-dict');
  }
})();
