import { save } from '@tauri-apps/api/dialog';

import { readFile, writeFile } from '@/tauri_cmd';
import { cacheKeys, type LocalCache } from '@/utils/local_storage_manager';

export const backup = {
  /** @throws Error */
  async import() {
    const pathKey = 'import-backup-path';
    const settings = await readFile(pathKey, 'g_dar2oar_settings');
    if (settings) {
      const obj = JSON.parse(settings);

      // Validate
      Object.keys(obj).forEach((key) => {
        // The import path does not need to be overwritten.
        if (key === pathKey) {
          return;
        }
        // Remove invalid settings values
        if (!cacheKeys.includes(key as any)) {
          delete obj[key];
        }
      });

      return obj as LocalCache;
    }
  },

  /** @throws Error */
  async export(settings: LocalCache) {
    const pathKey = 'export-settings-path';
    const cachedPath = localStorage.getItem(pathKey);
    const path = await save({
      defaultPath: cachedPath ?? undefined,
      filters: [
        {
          name: 'g_dar2oar_settings',
          extensions: ['json'],
        },
      ],
    });

    if (typeof path === 'string') {
      await writeFile(path, JSON.stringify(settings, null, 2));
      return path;
    } else {
      return null;
    }
  },
};
