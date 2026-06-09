export interface ApiKey {
  id: number
  provider_id: number
  name: string
  masked_preview: string
  description: string | null
  openai_base_url: string | null
  anthropic_base_url: string | null
  parent_id: number | null
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
  openai_base_url: string | null
  anthropic_base_url: string | null
  parent_id: number | null
  created_at: string
  updated_at: string
}

export interface CreateKeyInput {
  provider_id: number
  name: string
  raw_key: string
  description?: string
  openai_base_url?: string
  anthropic_base_url?: string
  parent_id?: number
}

export interface UpdateKeyInput {
  id: number
  provider_id: number
  name: string
  raw_key?: string
  description?: string
  openai_base_url?: string
  anthropic_base_url?: string
  parent_id?: number
}
