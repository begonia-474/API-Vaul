import { createI18n } from 'vue-i18n'
import en from './en.json'
import zhCN from './zh-CN.json'
import zhTW from './zh-TW.json'
import ja from './ja.json'

export type SupportedLocale = 'en' | 'zh-CN' | 'zh-TW' | 'ja'

export const LOCALE_OPTIONS = [
  { label: 'English', value: 'en' as SupportedLocale },
  { label: 'Chinese (Simplified)', value: 'zh-CN' as SupportedLocale },
  { label: 'Chinese (Traditional)', value: 'zh-TW' as SupportedLocale },
  { label: 'Japanese', value: 'ja' as SupportedLocale },
]

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  messages: {
    en,
    'zh-CN': zhCN,
    'zh-TW': zhTW,
    ja,
  },
})

export default i18n