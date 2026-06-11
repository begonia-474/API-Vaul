export type ProviderCategory =
  | 'official'
  | 'cn_official'
  | 'cloud_provider'
  | 'aggregator'
  | 'third_party'
  | 'custom'

export type CompatType = 'openai_compatible' | 'anthropic_compatible'

export interface CompatOption {
  label: string
  authType: string
  baseUrlPlaceholder?: string
}

export interface ProviderPreset {
  presetId: string
  name: string
  category: ProviderCategory
  icon?: string
  iconColor?: string
  websiteUrl?: string
  apiKeyUrl?: string
  defaultCompatType: CompatType
  compatOptions: Record<CompatType, CompatOption>
}

export interface Provider {
  id: number
  name: string
  display_name: string
  icon: string | null
  base_url: string
  api_type: string
  compat_type: CompatType | null
  category: ProviderCategory | null
  website_url: string | null
  api_key_url: string | null
  preset_id: string | null
  openai_base_url: string | null
  anthropic_base_url: string | null
  description: string | null
  created_at: string
}

export interface CreateProviderInput {
  name: string
  display_name: string
  icon?: string
  base_url: string
  api_type: string
  compat_type: CompatType
  category: ProviderCategory
  website_url?: string
  api_key_url?: string
  preset_id?: string
}

export interface UpdateProviderInput {
  id: number
  name: string
  display_name: string
  icon?: string
  base_url: string
  api_type: string
  compat_type: CompatType
  category: ProviderCategory
  website_url?: string
  api_key_url?: string
  preset_id?: string
}