<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NIcon } from 'naive-ui'
import { KeyOutline, SettingsOutline } from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const router = useRouter()
const route = useRoute()

const navItems = computed(() => [
  { key: 'keys', label: t('nav.allKeys'), icon: KeyOutline, route: '/keys' },
  { key: 'settings', label: t('nav.settings'), icon: SettingsOutline, route: '/settings' },
])

function isActiveRoute(path: string): boolean {
  return route.path.startsWith(path)
}

function handleNav(path: string) {
  router.push(path)
}
</script>

<template>
  <aside class="app-sidebar">
    <div class="sidebar-logo">
      <span class="logo-text">{{ $t('app.name') }}</span>
    </div>

    <nav class="sidebar-nav">
      <div
        v-for="item in navItems"
        :key="item.key"
        class="nav-item"
        :class="{ active: isActiveRoute(item.route) }"
        @click="handleNav(item.route)"
      >
        <n-icon :component="item.icon" :size="18" />
        <span>{{ item.label }}</span>
      </div>
    </nav>
  </aside>
</template>

<style scoped>
.app-sidebar {
  width: var(--sidebar-width);
  height: 100vh;
  background-color: var(--bg-surface);
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow-y: auto;
}

.sidebar-logo {
  padding: var(--space-4) var(--space-4);
  border-bottom: 1px solid var(--border-default);
}

.logo-text {
  font-size: var(--text-lg);
  font-weight: var(--font-bold);
  background: linear-gradient(135deg, var(--color-primary-hover), var(--color-primary));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.sidebar-nav {
  padding: var(--space-2) 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  cursor: pointer;
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  font-size: var(--text-sm);
}

.nav-item:hover {
  color: var(--text-primary);
  background-color: var(--bg-hover);
}

.nav-item.active {
  color: var(--color-primary);
  background-color: var(--color-primary-light);
  position: relative;
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 4px;
  bottom: 4px;
  width: 3px;
  border-radius: 0 2px 2px 0;
  background: var(--color-primary);
}
</style>
