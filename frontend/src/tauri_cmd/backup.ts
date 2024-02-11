import { save } from '@tauri-apps/api/dialog';

import { readFile, writeFile } from '.';

export const backup = {
  /** @throws Error */
  async import() {
    const pathKey = 'import-backup-path';
    const settings = await readFile(pathKey, 'g_dar2oar_settings');
    if (settings) {
      // TODO: This is unsafe because the key is not validated.
      const obj = JSON.parse(settings);
      Object.keys(obj).forEach((key) => {
        // The import path does not need to be overwritten.
        if (key === pathKey) {
          return;
        }
        localStorage.setItem(key, obj[key]);
      });
      window.location.reload(); // To enable
    }
  },

  /** @throws Error */
  async export() {
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
      if (cachedPath === '') {
        localStorage.removeItem(pathKey);
      } else {
        localStorage.setItem(pathKey, path);
      }
      await writeFile(path, JSON.stringify(localStorage, null, 2));
      return path;
    } else {
      return null;
    }
  },
};
