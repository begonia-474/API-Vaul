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

const themeOverrides = computed<GlobalThemeOverrides>(() => ({
  common: {
    primaryColor: '#6366F1',
    primaryColorHover: '#818CF8',
    primaryColorPressed: '#4F46E5',
    bodyColor: settingsStore.resolvedTheme === 'dark' ? '#0F1117' : '#F8FAFC',
    cardColor: settingsStore.resolvedTheme === 'dark' ? '#1A1D2E' : '#FFFFFF',
    modalColor: settingsStore.resolvedTheme === 'dark' ? '#1A1D2E' : '#FFFFFF',
    popoverColor: settingsStore.resolvedTheme === 'dark' ? '#1A1D2E' : '#FFFFFF',
    inputColor: settingsStore.resolvedTheme === 'dark' ? '#252840' : '#F1F5F9',
    tableColor: settingsStore.resolvedTheme === 'dark' ? '#1A1D2E' : '#FFFFFF',
    borderRadius: '8px',
    fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
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