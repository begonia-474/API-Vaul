<script setup lang="ts">
import { darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import { computed, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useSettingsStore } from '@/stores/settings'
import { useIdleTimer } from '@/composables/useIdleTimer'

const route = useRoute()
const auth = useAuthStore()
const settingsStore = useSettingsStore()

const isLockScreen = computed(() => route.name === 'LockScreen')

const naiveTheme = computed(() => (settingsStore.resolvedTheme === 'dark' ? darkTheme : undefined))

const isDark = computed(() => settingsStore.resolvedTheme === 'dark')

const themeOverrides = computed<GlobalThemeOverrides>(() => ({
  common: {
    primaryColor: '#0A84FF',
    primaryColorHover: '#409CFF',
    primaryColorPressed: '#0060DF',
    bodyColor: isDark.value ? '#1C1C1E' : '#FFFFFF',
    cardColor: isDark.value ? '#262628' : '#FFFFFF',
    modalColor: isDark.value ? '#262628' : '#FFFFFF',
    popoverColor: isDark.value ? '#262628' : '#FFFFFF',
    inputColor: isDark.value ? '#2C2C2E' : '#FFFFFF',
    inputColorFocus: isDark.value ? '#2C2C2E' : '#FFFFFF',
    tableColor: isDark.value ? '#262628' : '#FFFFFF',
    borderRadius: '12px',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
  },
  Input: {
    border: isDark.value ? '1px solid #3A3A3C' : '1px solid #D1D1D6',
    borderHover: '1px solid #0A84FF',
    borderFocus: '1px solid #0A84FF',
    boxShadowFocus: '0 0 0 2px rgba(10, 132, 255, 0.2)',
    caretColor: '#0A84FF',
  },
  Card: {
    borderRadius: '14px',
    borderColor: isDark.value ? '#3A3A3C' : '#E5E5EA',
    boxShadow: 'none',
  },
}))

const idleTimeoutMs = computed(() => Math.max(settingsStore.autoLockMinutes, 1) * 60_000)
const idleTimer = useIdleTimer(idleTimeoutMs, () => auth.lock())

watch(
  () => auth.isUnlocked,
  (unlocked) => {
    if (unlocked) {
      idleTimer.start()
      return
    }
    idleTimer.stop()
  },
  { immediate: true },
)

onMounted(async () => {
  settingsStore.bindSystemThemeListener()
  await settingsStore.fetchSettings()
})
</script>

<template>
  <n-config-provider :theme="naiveTheme" :theme-overrides="themeOverrides">
    <n-dialog-provider>
      <n-notification-provider>
        <n-message-provider>
          <div class="app-root">
            <router-view v-if="isLockScreen" />
            <div v-else class="app-layout">
              <router-view v-slot="{ Component }">
                <transition name="fade" mode="out-in">
                  <component :is="Component" />
                </transition>
              </router-view>
            </div>
          </div>
        </n-message-provider>
      </n-notification-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<style scoped>
.app-root {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background-color: var(--bg-app);
}

.app-layout {
  display: flex;
  width: 100%;
  height: 100%;
}
</style>