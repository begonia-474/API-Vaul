export interface ValidationResult {
  valid: boolean
  message?: string
}

/**
 * Validate a key name (required, 1-100 chars).
 */
export function validateKeyName(name: string): ValidationResult {
  const trimmed = name.trim()
  if (!trimmed) {
    return { valid: false, message: '名称不能为空' }
  }
  if (trimmed.length > 100) {
    return { valid: false, message: '名称不能超过 100 个字符' }
  }
  return { valid: true }
}

/**
 * Validate an API key value (required, minimum length).
 */
export function validateKey(key: string): ValidationResult {
  const trimmed = key.trim()
  if (!trimmed) {
    return { valid: false, message: 'Key 不能为空' }
  }
  if (trimmed.length < 8) {
    return { valid: false, message: 'Key 长度不能少于 8 个字符' }
  }
  return { valid: true }
}

/**
 * Validate that a provider is selected.
 */
export function validateProviderId(id: number | null | undefined): ValidationResult {
  if (!id || id <= 0) {
    return { valid: false, message: '请选择供应商' }
  }
  return { valid: true }
}
