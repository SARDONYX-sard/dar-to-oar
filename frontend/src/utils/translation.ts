'use client';
import { use, type Resource } from 'i18next';
import { initReactI18next } from 'react-i18next';

import dictEnUS from '@/../../locales/en-US.json';
import dictJaJP from '@/../../locales/ja-JP.json';

// The keys in RESOURCE are language tags according to the BCP-47 standard.
// See: https://partnerhub.warnermediagroup.com/metadata/languages
const resources = {
  'en-US': {
    translation: dictEnUS,
  },
  'ja-JP': {
    translation: dictJaJP,
  },
} as const satisfies Resource;
export type I18nKeys = keyof (typeof resources)['en-US']['translation'];

use(initReactI18next) // passes i18n down to react-i18next
  .init({
    resources,
    // NOTE:
    // Since it seems that `window.navigator.language` cannot automatically detect the language,
    // I have created a hook called useLocale as a substitute.
    lng: localStorage.getItem('locale') ?? window.navigator.language,
    fallbackLng: 'en-US',
    interpolation: {
      escapeValue: false, // react already safes from xss
    },
  });
