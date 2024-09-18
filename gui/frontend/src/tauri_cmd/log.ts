import { appLogDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/plugin-shell';

/**
 * Opens the log file.
 * @throws - if not found path
 */
export async function openLogFile() {
  const logFile = `${await appLogDir()}/g_dar2oar.log`;
  await open(logFile);
}

/**
 * Opens the log directory.
 * @throws - if not found path
 */
export async function openLogDir() {
  await open(await appLogDir());
}
