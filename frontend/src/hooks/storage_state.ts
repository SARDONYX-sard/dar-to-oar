import { useState } from "react";

const getCacheStr = (cacheKey: string, initialState: string) =>
  localStorage.getItem(cacheKey) ?? initialState;

/**
 * useState with localStorage
 * @param keyName
 * @param fallbackState - if localStorage.getItem is returned null, then use.
 */
export function useStorageState(keyName: string, fallbackState = "") {
  const [value, setValue] = useState(getCacheStr(keyName, fallbackState));

  const setState = (value_: string) => {
    localStorage.setItem(keyName, value_);
    setValue(value_);
  };

  return [value, setState] as const;
}
