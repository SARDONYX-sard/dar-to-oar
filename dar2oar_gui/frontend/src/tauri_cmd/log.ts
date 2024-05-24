import { appLogDir } from '@tauri-apps/api/path';
import { open as openShell } from '@tauri-apps/api/shell';

/**
 * Opens the log file.
 * @throws - if not found path
 */
export async function openLogFile() {
  const logFile = `${await appLogDir()}/g_dar2oar.log`;
  await openShell(logFile);
}

/**
 * Opens the log directory.
 * @throws - if not found path
 */
export async function openLogDir() {
  await openShell(await appLogDir());
}
