import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { type OpenDialogOptions, open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import { open as openShell } from '@tauri-apps/plugin-shell';

import { notify } from '@/components/notifications';
import { type CacheKey, localStorageManager } from '@/utils/local_storage_manager';

/**
 * Read the entire contents of a file into a string.
 * @param pathKey - target path cache key
 * @return contents
 * @throws `Error`
 */
export async function readFile(pathKey: CacheKey, filterName: string) {
  let path = localStorageManager.get(pathKey) ?? '';

  const setPath = (newPath: string) => {
    path = newPath;
    localStorageManager.set(pathKey, path);
  };

  if (
    await openPath(path, {
      setPath,
      filters: [
        {
          name: filterName,
          extensions: ['json'],
        },
      ],
    })
  ) {
    return await readTextFile(path);
  }
  return null;
}

/**
 *Alternative file writing API to avoid tauri API bug.
 * # NOTE
 * We couldn't use `writeTextFile`!
 * - The `writeTextFile` of tauri's api has a bug that the data order of some contents is unintentionally swapped.
 * @param path - path to write
 * @param content - string content
 * @throws Error
 */
export async function writeFile(path: string, content: string) {
  await invoke('write_file', { path, content });
}

type OpenOptions = {
  /**
   * path setter.
   * - If we don't get the result within this function, somehow the previous value comes in.(React component)
   * @param path
   * @returns
   */
  setPath?: (path: string) => void;
} & OpenDialogOptions;

/**
 * Open a file or Dir
 * @returns selected path or cancelled null
 * @throws
 */
export async function openPath(path: string, options: OpenOptions) {
  const res = await open({
    defaultPath: path,
    ...options,
  });

  if (typeof res === 'string' && options.setPath) {
    options.setPath(res);
  }
  return res;
}

/**
 * Wrapper tauri's `open` with `notify.error`
 * @export
 * @param {string} path
 * @param {string} [openWith]
 */
export async function start(path: string, openWith?: string) {
  try {
    await openShell(path, openWith);
  } catch (error) {
    if (error instanceof Error) {
      notify.error(error.message);
    }
  }
}

/** HACK: Avoid blank white screen on load.
 * - https://github.com/tauri-apps/tauri/issues/5170#issuecomment-2176923461
 * - https://github.com/tauri-apps/tauri/issues/7488
 */
export function showWindow() {
  if (typeof window !== 'undefined') {
    getCurrentWindow().show();
  }
}
