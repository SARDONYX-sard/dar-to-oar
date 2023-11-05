import { changeLanguage } from "i18next";
import { useEffect } from "react";

/**
 * Change language
 */
export function useLocale() {
  useEffect(() => {
    const locale = localStorage.getItem("locale") ?? "es-US";
    changeLanguage(locale === "auto" ? window.navigator.language : locale);
  }, []);
}
