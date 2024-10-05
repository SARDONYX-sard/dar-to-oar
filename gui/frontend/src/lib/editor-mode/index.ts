import { STORAGE } from '@/lib/storage';
import { PUB_CACHE_OBJ } from '@/lib/storage/cacheKeys';

export type EditorMode = 'default' | 'vim';

const DEFAULT = 'default';

/**
 * `'error'` if null or undefined
 * @default `error`
 */
const normalize = (mode?: string | null): EditorMode => {
  if (mode === 'vim') {
    return mode;
  }
  return DEFAULT;
};

export const EDITOR_MODE = {
  default: DEFAULT,

  /** Fallback to `'default'` if `null` or `undefined`. */
  normalize,

  /** get current editor code from `LocalStorage`. */
  get() {
    return normalize(STORAGE.get(PUB_CACHE_OBJ.editorMode));
  },

  /** set editor mode to `LocalStorage`. */
  set(level: EditorMode) {
    STORAGE.set(PUB_CACHE_OBJ.editorMode, level);
  },
};
