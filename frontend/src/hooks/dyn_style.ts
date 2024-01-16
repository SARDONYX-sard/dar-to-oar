import { useEffect, useInsertionEffect, useState } from 'react';
import { toast } from 'react-hot-toast';

import { selectPreset, presetStyles } from '@/utils/styles';

const getStyle = () => {
  const presetNumber = selectPreset(localStorage.getItem('presetNumber') ?? '');
  if (presetNumber === '0') {
    return localStorage.getItem('customCSS') ?? '';
  } else {
    return presetStyles[presetNumber];
  }
};

/**
 * Inject CSS dynamically on the client side.
 * # NOTE
 * Frequent style recalculation is inevitable,
 * but this hook can solve the delay problem caused by style injection lifecycle discrepancies.
 *  - See: [useInsertionEffect](https://react.dev/reference/react/useInsertionEffect)
 */
export function useDynStyle(initialState = getStyle()) {
  const [style, setStyle] = useState(initialState);

  useInsertionEffect(() => {
    const styleElement = document.createElement('style');
    styleElement.innerHTML = style;
    document.head.appendChild(styleElement);
    return () => {
      document.head.removeChild(styleElement);
    };
  }, [style]);

  return [style, setStyle] as const;
}

/**
 * Inject JavaScript
 */
export function useInjectScript(initialState = (() => localStorage.getItem('customJS') ?? '')()) {
  const [script, setScript] = useState(initialState);
  const [pathname, setPathname] = useState<string | null>(null);

  useEffect(() => {
    const scriptElement = document.createElement('script');
    if (pathname !== window.location.pathname) {
      try {
        // comment remove script
        scriptElement.innerHTML = script;
        scriptElement.id = 'custom-script';
        if (!document.getElementById('custom-script')) {
          document.body.appendChild(scriptElement);
        }
      } catch (e) {
        toast.error(`${e}`);
      }
      setPathname(window.location.pathname);
    }
    return () => {
      if (document.getElementById('custom-script')) {
        document.body.removeChild(scriptElement);
      }
    };
  }, [script, pathname]);

  return [script, setScript] as const;
}
