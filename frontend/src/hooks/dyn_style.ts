import { useInsertionEffect, useState } from "react";
import { useStorageState } from "@/hooks";

/**
 * Inject CSS dynamically on the client side. & auto set local storage
 * # NOTE
 * Frequent style recalculation is inevitable,
 * but this hook can solve the delay problem caused by style injection lifecycle discrepancies.
 *  - See: [useInsertionEffect](https://react.dev/reference/react/useInsertionEffect)
 */
export function useDynStyleWithStorage() {
  const [style, setStyle] = useStorageState("customCSS");

  useInsertionEffect(() => {
    const styleElement = document.createElement("style");
    styleElement.innerHTML = style;
    document.head.appendChild(styleElement);
    return () => {
      document.head.removeChild(styleElement);
    };
  }, [style]);

  return [style, setStyle] as const;
}

/**
 * Inject CSS dynamically on the client side.
 * # NOTE
 * Frequent style recalculation is inevitable,
 * but this hook can solve the delay problem caused by style injection lifecycle discrepancies.
 *  - See: [useInsertionEffect](https://react.dev/reference/react/useInsertionEffect)
 */
export function useDynStyle() {
  const [style, setStyle] = useState("");

  useInsertionEffect(() => {
    const styleElement = document.createElement("style");
    styleElement.innerHTML = style;
    document.head.appendChild(styleElement);
    return () => {
      document.head.removeChild(styleElement);
    };
  }, [style]);

  return [style, setStyle] as const;
}
