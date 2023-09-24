"use client";

import styles from "./converter.module.css";
import { convertDar2oar, setDir } from "../tauri_cmd";
import { useEffect, useState } from "react";
import toast, { Toaster, useToasterStore } from "react-hot-toast";

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
  const [cachePath, setCachePath] = useState(getCacheStr("defaultPath"));
  const [modName, setModName] = useState(getCacheStr("modName"));
  const [authorName, setAuthorName] = useState(getCacheStr("authorName"));

  const handleConverter = (): void => {
    if (cachePath === "") {
      toast.error("First, please select Directory.");
      handleDefaultPath();
      return;
    }

    toast.promise(convertDar2oar(cachePath, cachePath, modName, authorName), {
      success: "Completed.",
      loading: "Converting...",
      error: (e) => {
        console.error(e);
        return `${e}`;
      },
    });

    convertDar2oar(cachePath, cachePath, modName, authorName).catch((e) =>
      toast.error(`${e}`)
    );
  };

  const handleDefaultPath = async () => {
    const setDefaultPath = (str: string) => {
      localStorage.setItem("defaultPath", str);
      setCachePath(str);
    };

    setDir({
      defaultPath: cachePath,
      setDefaultPath,
    }).catch((e) => toast.error(`${e}`));
  };

  const handleAuthorName = (e: React.FormEvent<HTMLInputElement>): void => {
    window.localStorage.setItem("authorName", e.currentTarget.value);
    setAuthorName(e.currentTarget.value);
  };
  const handleModName = (e: React.FormEvent<HTMLInputElement>): void => {
    localStorage.setItem("modName", e.currentTarget.value);
    setModName(e.currentTarget.value);
  };

  return (
    <main className={styles.main}>
      <Toaster position="bottom-right" reverseOrder={false} />
      <div className={styles.description}>
        <h2>{cachePath}</h2>
      </div>

      <div className={styles.grid}>
        <div className={styles.card}>
          <h2>
            Mod Name <span>-&gt;</span>
          </h2>

          <input type="text" value={modName} onInput={handleModName} />
        </div>

        <div className={styles.card}>
          <h2>
            Author Name <span>-&gt;</span>
          </h2>
          <input type="text" value={authorName} onInput={handleAuthorName} />
        </div>

        <a
          href="https://nextjs.org/learn?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
          className={styles.card}
          target="_blank"
          rel="noopener noreferrer"
        >
          <h2>
            Link <span>-&gt;</span>
          </h2>
        </a>

        <div className={styles.card}>
          <h2>
            Directory <span>-&gt;</span>
          </h2>
          <button className={styles.card} onClick={handleDefaultPath}>
            Select
          </button>
        </div>
      </div>

      <button className={styles.card} onClick={handleConverter}>
        Convert DAR to OAR
      </button>
    </main>
  );
}
