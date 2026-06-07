export interface ApiKey {
  id: number
  provider_id: number
  name: string
  masked_preview: string
  description: string | null
  base_url: string | null
  created_at: string
  updated_at: string
}

export interface ApiKeyView {
  id: number
  provider_id: number
  provider_name: string
  provider_display_name: string
  name: string
  masked_preview: string
  description: string | null
  base_url: string | null
  created_at: string
  updated_at: string
}

export interface CreateKeyInput {
  provider_id: number
  name: string
  raw_key: string
  description?: string
  base_url?: string
}

export interface UpdateKeyInput {
  id: number
  provider_id: number
  name: string
  raw_key?: string
  description?: string
  base_url?: string
}
