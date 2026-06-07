import { ref } from 'vue'

const clipboardText = ref('')
let clearTimer: ReturnType<typeof setTimeout> | null = null

/**
 * Clipboard composable with auto-clear after 30 seconds.
 * Uses navigator.clipboard with a fallback to execCommand.
 */
export function useClipboard() {
  const copied = ref(false)

  async function copy(text: string): Promise<boolean> {
    try {
      await navigator.clipboard.writeText(text)
      clipboardText.value = text
      copied.value = true

      if (clearTimer) clearTimeout(clearTimer)
      clearTimer = setTimeout(() => {
        clipboardText.value = ''
        copied.value = false
      }, 30_000)

      return true
    } catch {
      // Fallback for older browsers / non-HTTPS
      try {
        const textarea = document.createElement('textarea')
        textarea.value = text
        textarea.style.position = 'fixed'
        textarea.style.opacity = '0'
        document.body.appendChild(textarea)
        textarea.select()
        document.execCommand('copy')
        document.body.removeChild(textarea)

        clipboardText.value = text
        copied.value = true

        if (clearTimer) clearTimeout(clearTimer)
        clearTimer = setTimeout(() => {
          clipboardText.value = ''
          copied.value = false
        }, 30_000)

        return true
      } catch {
        return false
      }
    }
  }

  function clear() {
    if (clearTimer) clearTimeout(clearTimer)
    clipboardText.value = ''
    copied.value = false
  }

  return { copy, clear, copied, clipboardText }
}
