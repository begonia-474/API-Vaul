import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'

const mockInvoke = vi.mocked(invoke)

describe('settings store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockInvoke.mockReset()
    document.documentElement.removeAttribute('data-theme')
  })

  it('fetchSettings loads from backend and applies theme', async () => {
    mockInvoke.mockResolvedValueOnce({ theme: 'light', auto_lock_minutes: 10 })
    const store = useSettingsStore()
    await store.fetchSettings()
    expect(store.theme).toBe('light')
    expect(store.autoLockMinutes).toBe(10)
    expect(store.initialized).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('light')
  })

  it('resolvedTheme returns theme directly when not system', async () => {
    mockInvoke.mockResolvedValueOnce({ theme: 'dark', auto_lock_minutes: 5 })
    const store = useSettingsStore()
    await store.fetchSettings()
    expect(store.resolvedTheme).toBe('dark')
  })

  it('resolvedTheme returns systemTheme when theme is system', async () => {
    // Mock matchMedia to simulate a dark system preference
    const matchMediaSpy = vi.spyOn(window, 'matchMedia').mockImplementation((query: string) => ({
      matches: query === '(prefers-color-scheme: dark)',
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    }))
    mockInvoke.mockResolvedValueOnce({ theme: 'system', auto_lock_minutes: 5 })
    const store = useSettingsStore()
    await store.fetchSettings()
    expect(store.resolvedTheme).toBe('dark')
    matchMediaSpy.mockRestore()
  })

  it('updateSettings persists and updates local state', async () => {
    mockInvoke.mockResolvedValueOnce({ theme: 'light', auto_lock_minutes: 15 })
    const store = useSettingsStore()
    const result = await store.updateSettings({ theme: 'light', auto_lock_minutes: 15 })
    expect(result).toBe(true)
    expect(store.theme).toBe('light')
    expect(store.autoLockMinutes).toBe(15)
  })

  it('updateSettings returns false on failure', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('db error'))
    const store = useSettingsStore()
    const result = await store.updateSettings({ theme: 'dark' })
    expect(result).toBe(false)
  })

  it('setTheme calls updateSettings', async () => {
    mockInvoke.mockResolvedValueOnce({ theme: 'light', auto_lock_minutes: 5 })
    const store = useSettingsStore()
    const result = await store.setTheme('light')
    expect(result).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('update_settings', { update: { theme: 'light' } })
  })

  it('setAutoLockMinutes calls updateSettings', async () => {
    mockInvoke.mockResolvedValueOnce({ theme: 'dark', auto_lock_minutes: 20 })
    const store = useSettingsStore()
    const result = await store.setAutoLockMinutes(20)
    expect(result).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('update_settings', { update: { auto_lock_minutes: 20 } })
  })
})