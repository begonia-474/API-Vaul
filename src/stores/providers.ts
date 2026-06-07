import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Provider, CreateProviderInput, UpdateProviderInput } from '@/types/provider'

export const useProvidersStore = defineStore('providers', () => {
  const providers = ref<Provider[]>([])
  const loading = ref(false)

  async function fetchProviders(): Promise<void> {
    loading.value = true
    try {
      providers.value = await invoke<Provider[]>('get_all_providers')
    } catch (err) {
      console.error('Failed to fetch providers:', err)
    } finally {
      loading.value = false
    }
  }

  async function createProvider(input: CreateProviderInput): Promise<boolean> {
    try {
      await invoke<Provider>('create_provider', { new: input })
      await fetchProviders()
      return true
    } catch (err) {
      console.error('Failed to create provider:', err)
      return false
    }
  }

  async function updateProvider(input: UpdateProviderInput): Promise<boolean> {
    try {
      await invoke<Provider>('update_provider', { update: input })
      await fetchProviders()
      return true
    } catch (err) {
      console.error('Failed to update provider:', err)
      return false
    }
  }

  async function deleteProvider(id: number): Promise<boolean> {
    try {
      await invoke('delete_provider', { id })
      providers.value = providers.value.filter((p) => p.id !== id)
      return true
    } catch (err) {
      console.error('Failed to delete provider:', err)
      return false
    }
  }

  function getProviderById(id: number): Provider | undefined {
    return providers.value.find((p) => p.id === id)
  }

  function getProviderByName(name: string): Provider | undefined {
    return providers.value.find((p) => p.name === name)
  }

  return {
    providers,
    loading,
    fetchProviders,
    createProvider,
    updateProvider,
    deleteProvider,
    getProviderById,
    getProviderByName,
  }
})