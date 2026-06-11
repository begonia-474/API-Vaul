import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useApiKeysStore } from './apiKeys'
import type { Provider, UpdateProviderInput } from '@/types/provider'
import type { ApiKeyView } from '@/types/apiKey'

export interface KeyGroup {
  key: string
  provider: Provider
  description: string
  keys: ApiKeyView[]
}

export const useProvidersStore = defineStore('providers', () => {
  const providers = ref<Provider[]>([])
  const providersWithKeys = ref<Provider[]>([])
  const loading = ref(false)
  const searchQuery = ref('')

  const filteredProviders = computed(() => {
    let result = providersWithKeys.value
    if (searchQuery.value.trim()) {
      const q = searchQuery.value.toLowerCase()
      result = result.filter(
        (p) =>
          p.display_name.toLowerCase().includes(q) ||
          p.name.toLowerCase().includes(q),
      )
    }
    return result
  })

  const keyGroups = computed<KeyGroup[]>(() => {
    const apiKeysStore = useApiKeysStore()
    const allKeys = apiKeysStore.keys
    const groupMap = new Map<number, KeyGroup>()

    for (const key of allKeys) {
      if (!groupMap.has(key.provider_id)) {
        const provider = providers.value.find((p) => p.id === key.provider_id)
        if (provider) {
          groupMap.set(key.provider_id, {
            key: `${key.provider_id}`,
            provider,
            description: provider.description ?? '',
            keys: [],
          })
        }
      }
      groupMap.get(key.provider_id)!.keys.push(key)
    }

    return Array.from(groupMap.values())
  })

  const filteredKeyGroups = computed<KeyGroup[]>(() => {
    let result = keyGroups.value
    if (searchQuery.value.trim()) {
      const q = searchQuery.value.toLowerCase()
      result = result.filter(
        (g) =>
          g.provider.display_name.toLowerCase().includes(q) ||
          g.provider.name.toLowerCase().includes(q) ||
          g.description.toLowerCase().includes(q),
      )
    }
    return result
  })

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

  async function fetchProvidersWithKeys(): Promise<void> {
    loading.value = true
    try {
      providersWithKeys.value = await invoke<Provider[]>('get_providers_with_keys')
    } catch (err) {
      console.error('Failed to fetch providers with keys:', err)
    } finally {
      loading.value = false
    }
  }

  async function fetchProviderKeys(providerId: number): Promise<ApiKeyView[]> {
    try {
      return await invoke<ApiKeyView[]>('get_provider_keys', { providerId })
    } catch (err) {
      console.error('Failed to fetch provider keys:', err)
      return []
    }
  }

  async function updateProviderMetadata(
    id: number,
    name?: string,
    displayName?: string,
    openaiBaseUrl?: string,
    anthropicBaseUrl?: string,
    description?: string,
  ): Promise<boolean> {
    try {
      await invoke<Provider>('update_provider_metadata', {
        id,
        name: name || null,
        displayName: displayName || null,
        openaiBaseUrl: openaiBaseUrl || null,
        anthropicBaseUrl: anthropicBaseUrl || null,
        description: description || null,
      })
      await fetchProviders()
      await fetchProvidersWithKeys()
      return true
    } catch (err) {
      console.error('Failed to update provider metadata:', err)
      return false
    }
  }

  async function createProvider(params: {
    name: string
    display_name: string
    icon?: string
    category?: string
    openai_base_url?: string
    anthropic_base_url?: string
    description?: string
    preset_id?: string
  }): Promise<Provider | null> {
    try {
      const provider = await invoke<Provider>('create_provider', {
        name: params.name,
        displayName: params.display_name,
        icon: params.icon || null,
        category: params.category || 'custom',
        openaiBaseUrl: params.openai_base_url || null,
        anthropicBaseUrl: params.anthropic_base_url || null,
        description: params.description || null,
        presetId: params.preset_id || null,
      })
      await fetchProviders()
      return provider
    } catch (err) {
      console.error('Failed to create provider:', err)
      return null
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
      providersWithKeys.value = providersWithKeys.value.filter((p) => p.id !== id)
      return true
    } catch (err) {
      console.error('Failed to delete provider:', err)
      return false
    }
  }

  function getProviderById(id: number): Provider | undefined {
    return providers.value.find((p) => p.id === id) ?? providersWithKeys.value.find((p) => p.id === id)
  }

  function getProviderByName(name: string): Provider | undefined {
    return providers.value.find((p) => p.name === name)
  }

  function setSearch(query: string) {
    searchQuery.value = query
  }

  return {
    providers,
    providersWithKeys,
    loading,
    searchQuery,
    filteredProviders,
    keyGroups,
    filteredKeyGroups,
    fetchProviders,
    fetchProvidersWithKeys,
    fetchProviderKeys,
    updateProviderMetadata,
    createProvider,
    updateProvider,
    deleteProvider,
    getProviderById,
    getProviderByName,
    setSearch,
  }
})
