import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useAuthStore = defineStore('auth', () => {
  const isUnlocked = ref(false)
  const isFirstRun = ref(false)

  async function checkFirstRun(): Promise<void> {
    try {
      isFirstRun.value = await invoke<boolean>('is_first_run')
    } catch (err) {
      console.error('Failed to check first run:', err)
      isFirstRun.value = true
    }
  }

  async function setupPassword(password: string): Promise<boolean> {
    try {
      const result = await invoke<boolean>('setup_password', { passwordStr: password })
      if (result) {
        isUnlocked.value = true
        isFirstRun.value = false
      }
      return result
    } catch (err) {
      console.error('Failed to setup password:', err)
      throw err
    }
  }

  async function unlock(password: string): Promise<boolean> {
    try {
      const result = await invoke<boolean>('verify_password', { passwordStr: password })
      if (result) {
        isUnlocked.value = true
      }
      return result
    } catch (err) {
      console.error('Failed to unlock:', err)
      return false
    }
  }

  function lock(): void {
    isUnlocked.value = false
  }

  async function changePassword(oldPassword: string, newPassword: string): Promise<boolean> {
    try {
      const result = await invoke<boolean>('change_password', {
        oldPassword: oldPassword,
        newPassword: newPassword,
      })
      return result
    } catch (err) {
      console.error('Failed to change password:', err)
      return false
    }
  }

  return { isUnlocked, isFirstRun, checkFirstRun, setupPassword, unlock, lock, changePassword }
})