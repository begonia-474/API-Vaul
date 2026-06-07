import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'
import { useAuthStore } from '@/stores/auth'

const mockInvoke = vi.mocked(invoke)

describe('auth store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockInvoke.mockReset()
  })

  it('checkFirstRun sets isFirstRun', async () => {
    mockInvoke.mockResolvedValueOnce(true)
    const auth = useAuthStore()
    await auth.checkFirstRun()
    expect(auth.isFirstRun).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('is_first_run')
  })

  it('checkFirstRun sets isFirstRun to false when not first run', async () => {
    mockInvoke.mockResolvedValueOnce(false)
    const auth = useAuthStore()
    await auth.checkFirstRun()
    expect(auth.isFirstRun).toBe(false)
    expect(mockInvoke).toHaveBeenCalledWith('is_first_run')
  })

  it('checkFirstRun defaults isFirstRun to true on error', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('network error'))
    const auth = useAuthStore()
    await auth.checkFirstRun()
    expect(auth.isFirstRun).toBe(true)
  })

  it('setupPassword sets isUnlocked on success', async () => {
    mockInvoke.mockResolvedValueOnce(true)
    const auth = useAuthStore()
    const result = await auth.setupPassword('test123')
    expect(result).toBe(true)
    expect(auth.isUnlocked).toBe(true)
    expect(auth.isFirstRun).toBe(false)
    expect(mockInvoke).toHaveBeenCalledWith('setup_password', { passwordStr: 'test123' })
  })

  it('setupPassword throws on failure', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('fail'))
    const auth = useAuthStore()
    await expect(auth.setupPassword('test123')).rejects.toThrow()
    expect(auth.isUnlocked).toBe(false)
  })

  it('unlock sets isUnlocked on success', async () => {
    mockInvoke.mockResolvedValueOnce(true)
    const auth = useAuthStore()
    const result = await auth.unlock('test123')
    expect(result).toBe(true)
    expect(auth.isUnlocked).toBe(true)
  })

  it('unlock returns false on failure', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('bad password'))
    const auth = useAuthStore()
    const result = await auth.unlock('wrong')
    expect(result).toBe(false)
    expect(auth.isUnlocked).toBe(false)
  })

  it('lock sets isUnlocked to false', async () => {
    mockInvoke.mockResolvedValueOnce(true)
    const auth = useAuthStore()
    await auth.unlock('test123')
    expect(auth.isUnlocked).toBe(true)
    mockInvoke.mockResolvedValueOnce(undefined)
    await auth.lock()
    expect(auth.isUnlocked).toBe(false)
  })

  it('changePassword returns true on success', async () => {
    mockInvoke.mockResolvedValueOnce(true)
    const auth = useAuthStore()
    const result = await auth.changePassword('old123', 'new123')
    expect(result).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('change_password', {
      oldPassword: 'old123',
      newPassword: 'new123',
    })
  })

  it('changePassword returns false on failure', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('wrong password'))
    const auth = useAuthStore()
    const result = await auth.changePassword('wrong', 'new123')
    expect(result).toBe(false)
  })
})