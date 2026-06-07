/**
 * Tauri invoke wrapper types.
 * All Tauri commands return a typed payload or throw a CommandError.
 */

export interface CommandError {
  code: string
  message: string
}

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: CommandError
}

/** Generic command options for filtering / pagination */
export interface ListOptions {
  search?: string
  provider_id?: number
  tags?: string[]
  page?: number
  per_page?: number
}

/** App settings read/write shape */
export interface AppSettings {
  lock_timeout_minutes: number
  theme: 'dark' | 'light'
  show_provider_icons: boolean
  masked_by_default: boolean
}
