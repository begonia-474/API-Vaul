import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import i18n, { type SupportedLocale } from '@/locales'

type ThemeChoice = 'dark' | 'light' | 'system'

export const useSettingsStore = defineStore('settings', () => {
  const theme = ref<ThemeChoice>('light')
  const autoLockMinutes = ref(5)
  const language = ref<SupportedLocale>('zh-CN')
  const loading = ref(false)
  const initialized = ref(false)

  const systemTheme = ref<'dark' | 'light'>('dark')

  const resolvedTheme = computed<'dark' | 'light'>(() => {
    if (theme.value === 'system') {
      return systemTheme.value
    }
    return theme.value
  })

  function applyThemeAttribute(value: 'dark' | 'light') {
    document.documentElement.setAttribute('data-theme', value)
  }

  function applyLanguage(lang: SupportedLocale) {
    i18n.global.locale.value = lang
  }

  watch(resolvedTheme, (val) => {
    applyThemeAttribute(val)
  })

  function syncSystemTheme() {
    if (typeof window === 'undefined') return
    const mql = window.matchMedia('(prefers-color-scheme: dark)')
    // Only trust matchMedia if it actually supports the query
    if (mql.media !== 'not all') {
      systemTheme.value = mql.matches ? 'dark' : 'light'
    }
    // else keep current default ('dark')
  }

  function bindSystemThemeListener() {
    if (typeof window === 'undefined') return
    syncSystemTheme()

    const mql = window.matchMedia('(prefers-color-scheme: dark)')
    mql.addEventListener('change', (event) => {
      systemTheme.value = event.matches ? 'dark' : 'light'
    })
  }

  // Re-check system theme when user switches to 'system'
  watch(theme, (val) => {
    if (val === 'system') {
      syncSystemTheme()
    }
  })

  async function fetchSettings(): Promise<void> {
    loading.value = true
    try {
      const settings = await invoke<{ theme: string; auto_lock_minutes: number; language: string }>('get_settings')
      theme.value = (['dark', 'light', 'system'].includes(settings.theme) ? settings.theme : 'dark') as ThemeChoice
      autoLockMinutes.value = settings.auto_lock_minutes
      language.value = (['en', 'zh-CN', 'zh-TW', 'ja'].includes(settings.language) ? settings.language : 'en') as SupportedLocale
      initialized.value = true
      applyThemeAttribute(resolvedTheme.value)
      applyLanguage(language.value)
    } catch (err) {
      console.error('Failed to load settings:', err)
    } finally {
      loading.value = false
    }
  }

  async function updateSettings(update: { theme?: ThemeChoice; auto_lock_minutes?: number; language?: SupportedLocale }): Promise<boolean> {
    try {
      const settings = await invoke<{ theme: string; auto_lock_minutes: number; language: string }>('update_settings', { update })
      theme.value = (['dark', 'light', 'system'].includes(settings.theme) ? settings.theme : theme.value) as ThemeChoice
      autoLockMinutes.value = settings.auto_lock_minutes
      language.value = (['en', 'zh-CN', 'zh-TW', 'ja'].includes(settings.language) ? settings.language : language.value) as SupportedLocale
      applyLanguage(language.value)
      return true
    } catch (err) {
      console.error('Failed to update settings:', err)
      return false
    }
  }

  async function setTheme(next: ThemeChoice): Promise<boolean> {
    theme.value = next
    return updateSettings({ theme: next })
  }

  async function setAutoLockMinutes(minutes: number): Promise<boolean> {
    autoLockMinutes.value = minutes
    return updateSettings({ auto_lock_minutes: minutes })
  }

  async function setLanguage(lang: SupportedLocale): Promise<boolean> {
    language.value = lang
    return updateSettings({ language: lang })
  }

  return {
    theme,
    autoLockMinutes,
    language,
    loading,
    initialized,
    resolvedTheme,
    fetchSettings,
    updateSettings,
    setTheme,
    setAutoLockMinutes,
    setLanguage,
    bindSystemThemeListener,
  }
})