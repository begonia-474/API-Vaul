export interface ApiKey {
  id: number
  provider_id: number
  name: string
  masked_preview: string
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
  parent_id: number | null
  created_at: string
  updated_at: string
}

export interface CreateKeyInput {
  provider_id: number
  name: string
  raw_key: string
  parent_id?: number
}

export interface UpdateKeyInput {
  id: number
  provider_id: number
  name: string
  raw_key?: string
  parent_id?: number
}
