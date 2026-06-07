import { ref, watch, onUnmounted } from 'vue'
import type { Ref } from 'vue'

export function useIdleTimer(timeoutMs: Ref<number>, onIdle: () => void) {
  let timer: ReturnType<typeof setTimeout> | null = null
  const active = ref(false)

  const activityEvents: Array<keyof DocumentEventMap> = ['mousemove', 'mousedown', 'keydown', 'touchstart', 'scroll']

  function clearTimer() {
    if (timer) {
      clearTimeout(timer)
      timer = null
    }
  }

  function resetTimer() {
    clearTimer()
    if (!active.value) return
    const timeout = Math.max(timeoutMs.value, 1_000)
    timer = setTimeout(() => onIdle(), timeout)
  }

  function onActivity() {
    resetTimer()
  }

  function start() {
    if (active.value) return
    active.value = true
    activityEvents.forEach((event) => document.addEventListener(event, onActivity, { passive: true }))
    document.addEventListener('visibilitychange', onActivity)
    resetTimer()
  }

  function stop() {
    active.value = false
    activityEvents.forEach((event) => document.removeEventListener(event, onActivity))
    document.removeEventListener('visibilitychange', onActivity)
    clearTimer()
  }

  watch(timeoutMs, () => {
    if (active.value) resetTimer()
  })

  onUnmounted(() => stop())

  return { start, stop }
}