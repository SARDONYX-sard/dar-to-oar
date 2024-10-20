import { type ReactNode, createContext, useContext, useState } from 'react';

import { STORAGE } from '@/lib/storage';
import { PUB_CACHE_OBJ } from '@/lib/storage/cacheKeys';

type ContextType = {
  js: string;
  setJs: (value?: string) => void;
};
const Context = createContext<ContextType | undefined>(undefined);

type Props = { children: ReactNode };
export const JsProvider = ({ children }: Props) => {
  const [js, setJs] = useState(STORAGE.get(PUB_CACHE_OBJ.customJs) ?? '');

  const setHook = (value?: string) => {
    if (value) {
      setJs(value);
      STORAGE.set(PUB_CACHE_OBJ.customJs, value);
    } else {
      STORAGE.remove(PUB_CACHE_OBJ.customJs);
    }
  };

  return <Context.Provider value={{ js, setJs: setHook }}>{children}</Context.Provider>;
};

/**
 * @throws `useJsContext must be used within a JsProvider`
 */
export const useJsContext = () => {
  const context = useContext(Context);
  if (!context) {
    throw new Error('useJsContext must be used within a JsProvider');
  }
  return context;
};
