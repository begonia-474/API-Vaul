import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { CommandError } from '@/types/tauri'

/**
 * Generic Tauri invoke wrapper with loading state and error handling.
 *
 * Usage:
 *   const { data, loading, error, execute } = useTauriCommand<ApiKey[]>('list_keys')
 *   await execute({ provider_id: 1 })
 */
export function useTauriCommand<T>(command: string) {
  const data = ref<T | null>(null)
  const loading = ref(false)
  const error = ref<CommandError | null>(null)

  async function execute<R = T>(args?: Record<string, unknown>): Promise<R | null> {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<R>(command, args ?? {})
      data.value = result as unknown as T
      return result
    } catch (err: unknown) {
      const cmdError: CommandError = {
        code: 'UNKNOWN',
        message: err instanceof Error ? err.message : String(err),
      }
      // Tauri errors may carry code + message
      if (typeof err === 'object' && err !== null && 'code' in err) {
        cmdError.code = (err as Record<string, string>).code ?? 'UNKNOWN'
        cmdError.message = (err as Record<string, string>).message ?? cmdError.message
      }
      error.value = cmdError
      return null
    } finally {
      loading.value = false
    }
  }

  function reset() {
    data.value = null
    error.value = null
    loading.value = false
  }

  return { data, loading, error, execute, reset }
}
