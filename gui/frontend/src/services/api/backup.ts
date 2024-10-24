import { save } from '@tauri-apps/plugin-dialog';

import { OBJECT } from '@/lib/object-utils';
import { CACHE_KEYS, type Cache, STORAGE } from '@/lib/storage';
import { PRIVATE_CACHE_OBJ } from '@/lib/storage/cacheKeys';

import { readFile, writeFile } from './fs';

const SETTINGS_FILE_NAME = 'settings';

export const BACKUP = {
  /** @throws Error */
  async import(): Promise<Cache | null> {
    const settings = await readFile(PRIVATE_CACHE_OBJ.importSettingsPath, SETTINGS_FILE_NAME);
    if (settings === null) {
      return null;
    }

    const json = JSON.parse(settings);
    if (!OBJECT.isPropertyAccessible(json)) {
      return null;
    }

    // Validate
    for (const key of Object.keys(json)) {
      if (key === PRIVATE_CACHE_OBJ.importSettingsPath) {
        continue; // The import path does not need to be overwritten.
      }

      // Remove invalid settings values
      const isInvalidKey = !CACHE_KEYS.some((cacheKey) => cacheKey === key);
      if (isInvalidKey) {
        delete json[key];
      }
    }

    return json;
  },

  /** @throws Json parse Error */
  async export(settings: Cache) {
    const cachedPath = STORAGE.get(PRIVATE_CACHE_OBJ.exportSettingsPath);
    const path = await save({
      defaultPath: cachedPath ?? undefined,
      filters: [{ name: SETTINGS_FILE_NAME, extensions: ['json'] }],
    });

    if (typeof path === 'string') {
      await writeFile(path, `${JSON.stringify(settings, null, 2)}\n`);
      return path;
    }
    return null;
  },
} as const;
