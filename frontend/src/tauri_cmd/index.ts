import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";

/**
 *
 * @param srcDir
 * @param distDir
 * @param modName
 * @param modAuthor
 *
 * # Throw Error
 */
export async function convertDar2oar(
  srcDir: string,
  distDir: string,
  modName?: string,
  modAuthor?: string,
  mappingPath?: string
): Promise<void> {
  try {
    await invoke("convert_dar2oar", {
      darModFolder: srcDir,
      oarModFolder: distDir,
      modName,
      modAuthor,
      mappingPath
    });
  } catch (e) {
    throw new Error(`${e}`);
  }
}

/**
 * @param pathState
 *
 * # Throw Error
 */
export async function setDir(pathState: {
  defaultPath: string;
  setDefaultPath: (s: string) => void;
}): Promise<void> {
  const { defaultPath, setDefaultPath } = pathState;

  const res = await open({
    defaultPath,
    directory: true,
  });

  if (typeof res === "string") {
    setDefaultPath(res);
  }
}
