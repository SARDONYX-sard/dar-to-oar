"use client";

import styles from "./converter.module.css";
import { convertDar2oar, setDir, setFile } from "../tauri_cmd";
import { useEffect, useState } from "react";
import toast, { Toaster, useToasterStore } from "react-hot-toast";
import { SelectLogLevel } from "./select_log_level";

export default function Converter() {
  // Avoid endless toast notices.
  const TOAST_LIMIT = 1;
  const { toasts } = useToasterStore();
  useEffect(() => {
    toasts
      .filter((t) => t.visible)
      .filter((_, i) => i >= TOAST_LIMIT)
      .forEach((t) => toast.dismiss(t.id));
  }, [toasts]);

  const getCacheStr = (cacheKey: string) =>
    localStorage.getItem(cacheKey) ?? "";
  const [src, setSrc] = useState(getCacheStr("src"));
  const [dist, setDist] = useState(getCacheStr("dist"));
  const [modName, setModName] = useState(getCacheStr("modName"));
  const [authorName, setAuthorName] = useState(getCacheStr("authorName"));
  const [table, setTable] = useState(getCacheStr("tablePath"));

  const handleConverter = (): void => {
    if (src === "") {
      toast.error("First, please select Directory.");
      handleSrc();
      return;
    }

    toast.promise(
      convertDar2oar(src, dist, modName, authorName, table, "error"),
      {
        success: "Completed.",
        loading: "Converting...",
        error: (e) => {
          console.error(e);
          return `${e}`;
        },
      }
    );

    convertDar2oar(src, src, modName, authorName).catch((e) =>
      toast.error(`${e}`)
    );
  };

  const handleSrc = async () => {
    const setDefaultPath = (str: string) => {
      localStorage.setItem("src", str);
      setSrc(str);
    };
    setDir({
      defaultPath: src,
      setDefaultPath,
    }).catch((e) => toast.error(`${e}`));
  };

  const handleDist = async () => {
    const setDefaultPath = (str: string) => {
      localStorage.setItem("dist", str);
      setDist(str);
    };
    setDir({
      defaultPath: dist,
      setDefaultPath,
    }).catch((e) => toast.error(`${e}`));
  };

  const handleAuthorName = (e: React.FormEvent<HTMLInputElement>): void => {
    localStorage.setItem("authorName", e.currentTarget.value);
    setAuthorName(e.currentTarget.value);
  };
  const handleModName = (e: React.FormEvent<HTMLInputElement>): void => {
    localStorage.setItem("modName", e.currentTarget.value);
    setModName(e.currentTarget.value);
  };

  const setTableHook = (str: string) => {
    localStorage.setItem("tablePath", str);
    setTable(str);
  };
  const handleTable = async () => {
    setFile({
      defaultPath: table,
      setDefaultPath: setTableHook,
    }).catch((e) => toast.error(`${e}`));
  };

  return (
    <main className={styles.main}>
      <Toaster position="bottom-right" reverseOrder={false} />
      <div className={styles.description}>
        <h2>{src}</h2>
      </div>

      <span>-&gt;</span>

      <div className={styles.description}>
        <h2>{dist}</h2>
      </div>

      <div className={styles.grid}>
        <div className={styles.card}>
          <h2>DAR(src) Directory</h2>
          <button className={styles.card} onClick={handleSrc}>
            Select
          </button>
        </div>

        <div className={styles.card}>
          <h2>OAR(dist) Directory(Optional)</h2>
          <button className={styles.card} onClick={handleDist}>
            Select
          </button>
        </div>

        <div className={styles.card}>
          <h2>Mod Name(Optional)</h2>
          <input type="text" value={modName} onInput={handleModName} />
        </div>

        <div className={styles.card}>
          <h2>Author Name(Optional)</h2>
          <input type="text" value={authorName} onInput={handleAuthorName} />
        </div>

        <div className={styles.card}>
          <h2>Mapping Table Path(Optional)</h2>
          <input
            type="text"
            value={table}
            onInput={(e) => setTableHook(e.currentTarget.value)}
          />
          <button className={styles.card} onClick={handleTable}>
            Select
          </button>
        </div>

        <SelectLogLevel />
      </div>
      <button className={styles.card} onClick={handleConverter}>
        Convert DAR to OAR
      </button>
    </main>
  );
}
