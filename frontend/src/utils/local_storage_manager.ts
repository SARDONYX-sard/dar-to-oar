export const pubCacheKeys = [
  'hideDar',
  'logLevel',
  'runParallel',
  'showProgress',

  'custom-translation-dict',
  'customCSS',
  'customJS',
  'editorMode',
  'locale',
  'presetNumber',
  'settings-tab-select',
  'snackbar-position',
] as const;

export const privateCacheKeys = [
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

  'import-backup-path',
  'import-settings-path',
  'lang-file-path',
] as const;

export const cacheKeys = [...pubCacheKeys, ...privateCacheKeys];

export type CacheKey = (typeof pubCacheKeys)[number] | (typeof privateCacheKeys)[number];
export type LocalCache = Partial<{
  [key in CacheKey]: string;
}>;

export const localStorageManager = {
  /**
   * @returns
   * - Value associated with the given key
   * - `null` if the given key does not exist.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Storage/getItem)
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
  getPubValues() {
    const res: LocalCache = {};
    pubCacheKeys.forEach((key) => {
      const val = localStorage.getItem(key);
      if (val) {
        res[key] = val;
      }
    });
    return res;
  },
  getAll() {
    const res: LocalCache = {};
    cacheKeys.forEach((key) => {
      const val = localStorage.getItem(key);
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
  /** Remove cache */
  remove: (key: CacheKey) => localStorage.removeItem(key),
  removePrivateItems() {
    privateCacheKeys.forEach((key) => localStorage.removeItem(key));
  },
};
