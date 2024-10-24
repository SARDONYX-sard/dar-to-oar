/// The reason for the key value pairs is to simplify refactoring of the language server.
/// If we leave them as strings, we cannot automate symbol changes.
import { OBJECT } from '@/lib/object-utils';

const FORM_PUB_CACHE_KEYS_OBJ = {
  hideDar: 'hideDar',
  runParallel: 'runParallel',
  showProgress: 'showProgress',
  inferPath: 'inferPath',
} as const;

const FORM_PRIVATE_CACHE_KEYS_OBJ = {
  cachedDst: 'cached-dst',
  cachedMapping1PersonPath: 'cached-mapping1personPath',
  cachedMappingPath: 'cached-mappingPath',
  cachedModAuthor: 'cached-modName',
  cachedModName: 'cached-modAuthor',
  cachedSrc: 'cached-src',
  dst: 'dst',
  mapping1personPath: 'mapping1personPath',
  mappingPath: 'mappingPath',
  modAuthor: 'modAuthor',
  modName: 'modName',
  src: 'src',
} as const;

const PUB_CACHE_KEYS_OBJ = {
  customCss: 'customCSS',
  presetNumber: 'presetNumber',
  editorMode: 'editorMode',
  customJs: 'customJS',
  logLevel: 'logLevel',
  customTranslationDict: 'custom-translation-dict',
  editorTabSelect: 'editor-tab-select',
  locale: 'locale',
  settingsTabSelect: 'settings-tab-select',
  settingsTabPosition: 'settings-tab-position',
  snackbarLimit: 'snackbar-limit',
  snackbarPosition: 'snackbar-position',
} as const;

const PRIVATE_CACHE_KEYS_OBJ = {
  exportSettingsPath: 'export-settings-path',
  importSettingsPath: 'import-backup-path',
  langFilePath: 'lang-file-path',
} as const;

export const PUB_CACHE_OBJ = {
  ...FORM_PUB_CACHE_KEYS_OBJ,
  ...PUB_CACHE_KEYS_OBJ,
  ...FORM_PUB_CACHE_KEYS_OBJ,
} as const;

export const PRIVATE_CACHE_OBJ = {
  ...FORM_PRIVATE_CACHE_KEYS_OBJ,
  ...PRIVATE_CACHE_KEYS_OBJ,
} as const;

export const HIDDEN_CACHE_OBJ = {
  runScript: 'run-script',
} as const;

/** Public cache keys that are available and exposed for standard use in the application. */
export const PUB_CACHE_KEYS = [...OBJECT.values(PUB_CACHE_OBJ)] as const;

/** Private cache keys that are internal to the application and may involve sensitive data or paths. */
const PRIVATE_CACHE_KEYS = [...OBJECT.values(PRIVATE_CACHE_OBJ)] as const;

/** Hidden cache keys, typically used for restricted data like permissions for running scripts. */
export const HIDDEN_CACHE_KEYS = [...OBJECT.values(HIDDEN_CACHE_OBJ)] as const;

/** Aggregated list of both public and private cache keys. */
export const CACHE_KEYS = [...PUB_CACHE_KEYS, ...PRIVATE_CACHE_KEYS] as const;
