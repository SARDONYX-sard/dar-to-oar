import { useInsertionEffect, useState } from "react";
import { selectPreset, presetStyles } from "../utils/styles";

const getStyle = () => {
  const presetNumber = selectPreset(localStorage.getItem("presetNumber") ?? "");
  if (presetNumber === "0") {
    return localStorage.getItem("customCSS") ?? "";
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
    const styleElement = document.createElement("style");
    styleElement.innerHTML = style;
    document.head.appendChild(styleElement);
    return () => {
      document.head.removeChild(styleElement);
    };
  }, [style]);

  return [style, setStyle] as const;
}
