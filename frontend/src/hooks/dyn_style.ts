import { useInsertionEffect } from "react";

/**
 * Inject CSS dynamically on the client side.
 * # NOTE
 * Frequent style recalculation is inevitable,
 * but this hook can solve the delay problem caused by style injection lifecycle discrepancies.
 *  - See: [useInsertionEffect](https://react.dev/reference/react/useInsertionEffect)
 * @param css
 */
export function useDynStyle(css: string) {
  useInsertionEffect(() => {
    const styleElement = document.createElement("style");
    styleElement.innerHTML = css;
    document.head.appendChild(styleElement);
    return () => {
      document.head.removeChild(styleElement);
    };
  }, [css]);
}
