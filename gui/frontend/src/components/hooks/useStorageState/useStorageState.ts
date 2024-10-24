import { useCallback, useState } from 'react';

import { STORAGE, type WithHideCacheKey } from '@/lib/storage';

// Helper function to retrieve the cache value and parse it from localStorage
const getCacheValue = <T>(key: WithHideCacheKey, fallback: T): T => {
  const cached = STORAGE.getSome(key);
  if (cached === null) {
    return fallback;
  }

  try {
    return JSON.parse(cached) as T; // FIXME: unsafe type conversion
  } catch {
    return fallback;
  }
};

/**
 * A custom React hook that syncs state with localStorage.
 *
 * This hook behaves like `useState`, but persists the state in localStorage.
 * It supports any serializable type, such as strings, numbers, booleans, or objects.
 *
 * @template T - The type of the state value.
 * @param {CacheKey} keyName - The key to store the value in localStorage.
 * @param {T} fallbackState - The default state to use if nothing is found in localStorage.
 * @returns {[T, (newValue: T) => void]} A stateful value and a function to update it, which also updates localStorage.
 *
 * @example
 * // For a string:
 * const [username, setUsername] = useStorageState<string>('username', 'Guest');
 *
 * // For a number:
 * const [count, setCount] = useStorageState<number>('count', 0);
 *
 * // For an object:
 * const [settings, setSettings] = useStorageState<{ theme: string }>('settings', { theme: 'light' });
 */
export function useStorageState<T>(keyName: WithHideCacheKey, fallbackState: T) {
  const [value, setValue] = useState<T>(getCacheValue<T>(keyName, fallbackState));

  const setState = useCallback(
    (newValue: T) => {
      setValue(newValue);

      const jsonStr = JSON.stringify(newValue);
      if (typeof jsonStr === 'string') {
        STORAGE.setSome(keyName, jsonStr);
      }
    },
    [keyName],
  );

  return [value, setState] as const;
}
