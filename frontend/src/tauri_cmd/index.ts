import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { appLogDir } from "@tauri-apps/api/path";
import { open as openShell } from "@tauri-apps/api/shell";

export type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

type ConverterArgs = {
  src: string;
  dist?: string;
  modName?: string;
  modAuthor?: string;
  mappingPath?: string;
  mapping1personPath?: string;
  runParallel?: boolean;
  hideDar?: boolean;
};

/**
 * Convert DAR to OAR
 * # Throw Error
 */
export async function convertDar2oar(props: ConverterArgs): Promise<string> {
  const darDir = props.src === "" ? undefined : props.src;
  const oarDir = props.dist === "" ? undefined : props.dist;
  const modName = props.modName === "" ? undefined : props.modName;
  const modAuthor = props.modAuthor === "" ? undefined : props.modAuthor;
  const mapping1personPath =
    props.mapping1personPath === "" ? undefined : props.mapping1personPath;
  const mappingPath = props.mappingPath === "" ? undefined : props.mappingPath;
  const runParallel = props.runParallel ?? false;
  const hideDar = props.hideDar ?? false;

  return invoke<string>("convert_dar2oar", {
    options: {
      darDir,
      oarDir,
      modName,
      modAuthor,
      mappingPath,
      mapping1personPath,
      runParallel,
      hideDar,
    },
  });
}

export async function changeLogLevel(logLevel?: LogLevel): Promise<void> {
  return invoke("change_log_level", { logLevel });
}

/**
 * @param darPath
 *
 * # Throw Error
 */
export async function restoreDarDir(darDir: string) {
  if (darDir === "") {
    throw new Error("DAR dir must be specified.");
  }
  return invoke<string>("restore_dar_dir", { darDir });
}

/**
 * @param darPath
 *
 * # Throw Error
 */
export async function removeOarDir(path: string) {
  await invoke("remove_oar_dir", { path });
}

/**
 * Open a file or Dir
 *
 * # Throw Error
 */
export async function openPath(
  path: string,
  setPath: (path: string) => void,
  isDir: boolean,
) {
  const res = await open({
    defaultPath: path,
    directory: isDir,
  });

  if (typeof res === "string") {
    //! NOTE:
    //! It is important to use setter here!
    //! If we don't get the result within this function, somehow the previous value comes in.
    setPath(res);
  }
}

export async function openLogFile() {
  const logDir = await appLogDir();
  const logFile = `${logDir}g_dar2oar.log`;
  console.log(logFile);
  await openShell(logFile);
}
