import { useInsertionEffect, useRef, useState } from 'react';

import { CSS_PRESETS, type CssPresets } from '@/lib/css';
import { NOTIFY } from '@/lib/notify';

const USER_CUSTOM_CSS_ID = 'user-custom-css';
const initCss = () => CSS_PRESETS.getPreset(CSS_PRESETS.get());

/**
 * Inject CSS dynamically on the client side.
 */
export function useInjectCss() {
  const [preset, setPreset] = useState<CssPresets>(CSS_PRESETS.get());
  const [css, setCss] = useState(initCss());
  const style = useRef<HTMLStyleElement | null>(null);

  const setPresetHook = (value: CssPresets) => {
    setPreset(value);
    CSS_PRESETS.setPreset(value);
  };

  const setHook = (value?: string) => {
    setCss(value ?? '');
  };

  // NOTE: Frequent style recalculation is inevitable, but this hook can solve the delay problem caused by style injection lifecycle discrepancies.
  //  - See: [useInsertionEffect](https://react.dev/reference/react/useInsertionEffect)
  useInsertionEffect(() => {
    const styleElement = document.createElement('style');

    if (!style.current) {
      styleElement.id = USER_CUSTOM_CSS_ID; // Assign ID so that user can edit
      styleElement.innerHTML = css;
      style.current = styleElement;
      NOTIFY.try(() => document.head.appendChild(styleElement));
    }

    return () => {
      style.current?.remove();
      style.current = null;
    };
  }, [css]);

  return { preset, setPreset: setPresetHook, css, setCss: setHook } as const;
}
