const formPubCacheKeys = ['hideDar', 'logLevel', 'runParallel', 'showProgress'] as const;
const formPrivateCacheKeys = [
  'cached-dst',
  'cached-mapping1personPath',
  'cached-mappingPath',
  'cached-src',
  'dst',
  'mapping1personPath',
  'mappingPath',
  'modAuthor',
  'modName',
  'src',
] as const;

export const pubCacheKeys = [
  ...formPubCacheKeys,
  'custom-translation-dict',
  'customCSS',
  'customJS',
  'editorMode',
  'locale',
  'presetNumber',
  'settings-tab-select',
  'snackbar-limit',
  'snackbar-position',
] as const;
export const privateCacheKeys = [
  ...formPrivateCacheKeys,
  'import-backup-path',
  'import-settings-path',
  'export-settings-path',
  'lang-file-path',
] as const;
export const cacheKeys = [...pubCacheKeys, ...privateCacheKeys];

export type CacheKey = (typeof cacheKeys)[number];
export type LocalCache = Partial<{ [key in CacheKey]: string }>;

/** Wrapper for type completion of keys to be cached */
export const localStorageManager = {
  /**
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Storage/getItem)
   * @returns
   * - Value associated with the given key
   * - `null` if the given key does not exist.
   */
  get(key: CacheKey) {
    return localStorage.getItem(key);
  },
  getFromKeys(keys: CacheKey[]) {
    const res: LocalCache = {};
    keys.forEach((key) => {
      const value = localStorageManager.get(key);
      if (value) {
        res[key] = value;
      }
    });
    return res;
  },
  /** Get all cache */
  getAll() {
    const res: LocalCache = {};
    cacheKeys.forEach((key) => {
      const val = localStorageManager.get(key);
      if (val) {
        res[key] = val;
      }
    });
    return res;
  },
  /** Set cache */
  set(key: CacheKey, value: string) {
    return localStorage.setItem(key, value);
  },
  removeFromKeys(keys: CacheKey[]) {
    keys.forEach((key) => localStorage.removeItem(key));
  },
};
