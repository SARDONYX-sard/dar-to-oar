"use client";
import i18n, { type Resource } from "i18next";
import { initReactI18next } from "react-i18next";
import translation_en from "@/../../locales/en.json";
import translation_ja from "@/../../locales/ja.json";

const resources = {
  "en-US": {
    translation: translation_en,
  },
  "ja-JP": {
    translation: translation_ja,
  },
} as const satisfies Resource;

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    resources,
    lng: localStorage.getItem("locale") ?? "en-US",
    fallbackLng: "en-US",
    interpolation: {
      escapeValue: false, // react already safes from xss
    },
  });

export default i18n;
