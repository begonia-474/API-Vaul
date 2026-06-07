import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ApiKeyView, CreateKeyInput, UpdateKeyInput } from '@/types/apiKey'

export const useApiKeysStore = defineStore('apiKeys', () => {
  const keys = ref<ApiKeyView[]>([])
  const loading = ref(false)
  const searchQuery = ref('')

  const filteredKeys = computed(() => {
    let result = keys.value

    if (searchQuery.value.trim()) {
      const q = searchQuery.value.toLowerCase()
      result = result.filter(
        (k) =>
          k.name.toLowerCase().includes(q) ||
          k.masked_preview.toLowerCase().includes(q) ||
          k.provider_display_name.toLowerCase().includes(q) ||
          k.provider_name.toLowerCase().includes(q) ||
          (k.description ?? '').toLowerCase().includes(q),
      )
    }

    return result
  })

  async function fetchKeys(): Promise<void> {
    loading.value = true
    try {
      keys.value = await invoke<ApiKeyView[]>('get_all_keys')
    } catch (err) {
      console.error('Failed to fetch keys:', err)
    } finally {
      loading.value = false
    }
  }

  async function searchKeys(query: string): Promise<void> {
    loading.value = true
    try {
      keys.value = await invoke<ApiKeyView[]>('search_keys', { query })
    } catch (err) {
      console.error('Failed to search keys:', err)
    } finally {
      loading.value = false
    }
  }

  async function createKey(input: CreateKeyInput): Promise<boolean> {
    try {
      await invoke<ApiKeyView>('create_key', { newKey: input })
      await fetchKeys()
      return true
    } catch (err) {
      console.error('Failed to create key:', err)
      return false
    }
  }

  async function updateKey(input: UpdateKeyInput): Promise<boolean> {
    try {
      await invoke<ApiKeyView>('update_key', { update: input })
      await fetchKeys()
      return true
    } catch (err) {
      console.error('Failed to update key:', err)
      return false
    }
  }

  async function deleteKey(id: number): Promise<boolean> {
    try {
      await invoke('delete_key', { id })
      keys.value = keys.value.filter((k) => k.id !== id)
      return true
    } catch (err) {
      console.error('Failed to delete key:', err)
      return false
    }
  }

  async function getDecryptedKey(id: number): Promise<string | null> {
    try {
      return await invoke<string>('get_decrypted_key', { id })
    } catch (err) {
      console.error('Failed to get decrypted key:', err)
      return null
    }
  }

  function getKeyById(id: number): ApiKeyView | undefined {
    return keys.value.find((k) => k.id === id)
  }

  function setSearch(query: string) {
    searchQuery.value = query
  }

  return {
    keys,
    loading,
    searchQuery,
    filteredKeys,
    fetchKeys,
    searchKeys,
    createKey,
    updateKey,
    deleteKey,
    getDecryptedKey,
    getKeyById,
    setSearch,
  }
})
