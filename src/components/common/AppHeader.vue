<script setup lang="ts">
import { NIcon } from 'naive-ui'
import { LockClosedOutline } from '@vicons/ionicons5'
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

defineProps<{
  title: string
}>()

const auth = useAuthStore()
const router = useRouter()

function handleLock() {
  auth.lock()
  router.push('/')
}
</script>

<template>
  <header class="app-header">
    <div class="header-left">
      <h1 class="header-title">{{ title }}</h1>
    </div>
    <div class="header-actions">
      <slot name="actions" />
      <n-icon
        class="lock-btn"
        :component="LockClosedOutline"
        :size="20"
        @click="handleLock"
      />
    </div>
  </header>
</template>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: var(--header-height);
  padding: 0 var(--space-6);
  border-bottom: 1px solid var(--border-default);
  background: color-mix(in srgb, var(--bg-surface) 80%, transparent);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  flex-shrink: 0;
}

.header-title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.lock-btn {
  cursor: pointer;
  color: var(--text-muted);
  padding: var(--space-2);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.lock-btn:hover {
  color: var(--color-warning);
  background: var(--bg-hover);
}
</style>
