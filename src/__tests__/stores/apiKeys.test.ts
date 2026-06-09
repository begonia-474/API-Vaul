import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'
import { useApiKeysStore } from '@/stores/apiKeys'
import type { ApiKeyView } from '@/types/apiKey'

const mockInvoke = vi.mocked(invoke)

const mockKeys: ApiKeyView[] = [
  {
    id: 1,
    provider_id: 1,
    provider_name: 'openai',
    provider_display_name: 'OpenAI',
    name: 'Test Key 1',
    masked_preview: 'sk-a****1234',
    description: 'Test desc',
    openai_base_url: 'https://api.openai.com/v1',
    anthropic_base_url: null,
    parent_id: null,
    created_at: '2026-01-01',
    updated_at: '2026-01-01',
  },
  {
    id: 2,
    provider_id: 2,
    provider_name: 'anthropic',
    provider_display_name: 'Anthropic',
    name: 'Test Key 2',
    masked_preview: 'sk-a****5678',
    description: null,
    openai_base_url: null,
    anthropic_base_url: null,
    parent_id: null,
    created_at: '2026-01-02',
    updated_at: '2026-01-02',
  },
]

describe('apiKeys store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockInvoke.mockReset()
  })

  it('fetchKeys loads keys from backend', async () => {
    mockInvoke.mockResolvedValueOnce(mockKeys)
    const store = useApiKeysStore()
    await store.fetchKeys()
    expect(store.keys).toHaveLength(2)
    expect(store.keys[0].name).toBe('Test Key 1')
    expect(store.loading).toBe(false)
  })

  it('filteredKeys filters by search query', async () => {
    mockInvoke.mockResolvedValueOnce(mockKeys)
    const store = useApiKeysStore()
    await store.fetchKeys()

    store.setSearch('openai')
    expect(store.filteredKeys).toHaveLength(1)
    expect(store.filteredKeys[0].provider_name).toBe('openai')
  })

  it('createKey calls invoke and refreshes keys', async () => {
    const newKeys = [...mockKeys]
    mockInvoke
      .mockResolvedValueOnce(undefined)
      .mockResolvedValueOnce(newKeys)

    const store = useApiKeysStore()
    const result = await store.createKey({
      provider_id: 1,
      name: 'New Key',
      raw_key: 'sk-new1234567890',
      description: 'desc',
      openai_base_url: 'https://api.openai.com/v1',
      anthropic_base_url: 'https://api.anthropic.com/v1',
    })

    expect(result).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('create_key', {
      newKey: {
        provider_id: 1,
        name: 'New Key',
        raw_key: 'sk-new1234567890',
        description: 'desc',
        openai_base_url: 'https://api.openai.com/v1',
        anthropic_base_url: 'https://api.anthropic.com/v1',
      },
    })
  })

  it('updateKey calls invoke and refreshes keys', async () => {
    mockInvoke
      .mockResolvedValueOnce(undefined)
      .mockResolvedValueOnce(mockKeys)

    const store = useApiKeysStore()
    const result = await store.updateKey({
      id: 1,
      provider_id: 1,
      name: 'Updated',
      raw_key: undefined,
      description: 'desc',
      openai_base_url: 'https://api.openai.com/v1',
      anthropic_base_url: 'https://api.anthropic.com/v1',
    })

    expect(result).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('update_key', {
      update: {
        id: 1,
        provider_id: 1,
        name: 'Updated',
        raw_key: undefined,
        description: 'desc',
        openai_base_url: 'https://api.openai.com/v1',
        anthropic_base_url: 'https://api.anthropic.com/v1',
      },
    })
  })

  it('deleteKey removes key locally', async () => {
    mockInvoke.mockResolvedValueOnce(mockKeys)
    const store = useApiKeysStore()
    await store.fetchKeys()

    mockInvoke.mockResolvedValueOnce(undefined)
    await store.deleteKey(1)

    expect(store.keys.find((k) => k.id === 1)).toBeUndefined()
  })
})
