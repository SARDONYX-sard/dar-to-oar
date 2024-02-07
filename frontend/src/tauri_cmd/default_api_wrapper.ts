import { type OpenDialogOptions, open } from '@tauri-apps/api/dialog';
import { readTextFile } from '@tauri-apps/api/fs';
import { open as openShell } from '@tauri-apps/api/shell';

import { notify } from '@/components/notifications';

/**
 * Read the entire contents of a file into a string.
 * @param {string} pathKey - target path cache key
 * @return [isCancelled, contents]
 * @throws
 */
export async function readFile(pathKey: string, filterName: string) {
  let path = localStorage.getItem(pathKey) ?? '';

  const setPath = (newPath: string) => {
    path = newPath;
    localStorage.setItem(pathKey, path);
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
