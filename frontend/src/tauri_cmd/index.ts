import { invoke } from '@tauri-apps/api';
import { type OpenDialogOptions, open } from '@tauri-apps/api/dialog';
import { appLogDir } from '@tauri-apps/api/path';
import { open as openShell } from '@tauri-apps/api/shell';

import { selectLogLevel } from '@/utils/selector';

export { open as openShell } from '@tauri-apps/api/shell';

type ConverterOptions = {
  src: string;
  dist?: string;
  modName?: string;
  modAuthor?: string;
  mappingPath?: string;
  mapping1personPath?: string;
  runParallel?: boolean;
  hideDar?: boolean;
  showProgress?: boolean;
};

/**
 * Converts DAR to OAR.
 *
 * This function converts a DAR (DynamicAnimationReplacer) to an OAR (OpenAnimationReplacer).
 * It takes the specified properties as arguments and invokes the appropriate conversion command.
 *
 * @param {ConverterOptions} props - Converter Options.
 * @returns {Promise<void>} A promise that resolves when converted.
 * @throws
 */
export async function convertDar2oar(props: ConverterOptions): Promise<void> {
  const args = {
    options: {
      darDir: props.src === '' ? undefined : props.src,
      oarDir: props.dist === '' ? undefined : props.dist,
      modName: props.modName === '' ? undefined : props.modName,
      modAuthor: props.modAuthor === '' ? undefined : props.modAuthor,
      mappingPath: props.mappingPath === '' ? undefined : props.mappingPath,
      mapping1personPath: props.mapping1personPath === '' ? undefined : props.mapping1personPath,
      runParallel: props.runParallel ?? false,
      hideDar: props.hideDar ?? false,
    },
  };

  let logLevel = selectLogLevel(localStorage.getItem('logLevel') ?? '');
  changeLogLevel(logLevel);

  const showProgress = props.showProgress ?? false;
  if (showProgress) {
    return invoke('convert_dar2oar_with_progress', args);
  } else {
    return invoke('convert_dar2oar', args);
  }
}

export type LogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error';

/**
 * Changes the log level.
 *
 * This function invokes the 'change_log_level' command with the specified log level.
 *
 * @param {LogLevel} [logLevel] - The log level to set. If not provided, the default log level will be used.
 * @returns {Promise<void>} A promise that resolves when the log level is changed.
 */
export async function changeLogLevel(logLevel?: LogLevel): Promise<void> {
  return invoke('change_log_level', { logLevel });
}

/**
 * @param darPath
 *
 * # Throw Error
 */
/**
 * Add '.mohidden' to DAR's files.
 * @param {string} darDir - A string representing the directory path of the DAR directory that needs to be
 * unhidden.
 *
 * @throws
 */
export async function unhideDarDir(darDir: string) {
  if (darDir === '') {
    throw new Error('darDir is empty string.');
  }
  await invoke('unhide_dar_dir', { darDir });
}

/**
 * The function `removeOarDir` is an asynchronous function that removes a specified directory and throws an error if the
 * path is empty.
 * @param {string} path - The `path` parameter is a string that specifies the directory path of the DAR or OAR directory
 * that needs to be removed.
 * @throws
 */
export async function removeOarDir(path: string) {
  if (path === '') {
    throw new Error('Specified path is empty string.');
  }
  await invoke('remove_oar_dir', { path });
}

/**
 * Read the entire contents of a file into a string.
 * @param {string} path - target path
 *
 * @return [isCancelled, contents]
 * @throws
 */
export async function importLang() {
  const langPathKey = 'lang-file-path';
  let path = localStorage.getItem(langPathKey) ?? '';

  const setPath = (newPath: string) => {
    path = newPath;
    localStorage.setItem(langPathKey, path);
  };

  if (await openPath(path, { setPath })) {
    return await invoke<string>('read_to_string', { path });
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
 *
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
 * Opens the log file.
 *
 * This function retrieves the log directory using the appLogDir function,
 * constructs the path to the log file, and opens the shell with the log file path.
 *
 * @throws - if not found path
 */
export async function openLogFile() {
  const logDir = await appLogDir();
  const logFile = `${logDir}g_dar2oar.log`;
  await openShell(logFile);
}

/**
 * Opens the log directory.
 *
 * This function opens the shell and awaits the result of the appLogDir function.
 *
 * @throws - if not found path
 */
export async function openLogDir() {
  await openShell(await appLogDir());
}
