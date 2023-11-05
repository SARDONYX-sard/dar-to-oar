import { changeLanguage } from "i18next";
import { useEffect } from "react";

/**
 * Change language
 */
export function useLocale() {
  useEffect(() => {
    changeLanguage(
      localStorage.getItem("locale") === "auto"
        ? window.navigator.language
        : "es-US"
    );
  }, []);
}
