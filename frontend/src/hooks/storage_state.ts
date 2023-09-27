import { useState } from "react";

const getCacheStr = (cacheKey: string) => localStorage.getItem(cacheKey) ?? "";

/**
 * useState with localStorage
 * @param keyName
 */
export function useStorageState(keyName: string) {
  const [value, setValue] = useState(getCacheStr(keyName));

  const setState = (value_: string) => {
    localStorage.setItem(keyName, value_);
    setValue(value_);
  };

  return [value, setState] as const;
}
