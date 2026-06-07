<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  title?: string
  description?: string
}>()
</script>

<template>
  <div class="empty-state">
    <div class="empty-illustration">
      <svg width="120" height="100" viewBox="0 0 120 100" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect x="20" y="50" width="80" height="40" rx="8" fill="var(--bg-elevated)" stroke="var(--border-default)" stroke-width="1.5"/>
        <rect x="30" y="58" width="60" height="6" rx="3" fill="var(--border-default)"/>
        <rect x="30" y="70" width="40" height="6" rx="3" fill="var(--border-default)"/>
        <circle cx="85" cy="73" r="8" fill="var(--color-primary-light)" stroke="var(--color-primary)" stroke-width="1.5"/>
        <line x1="90.5" y1="78.5" x2="96" y2="84" stroke="var(--color-primary)" stroke-width="2" stroke-linecap="round"/>
        <path d="M55 20 L55 8 M55 20 L45 14 M55 20 L65 14" stroke="var(--text-muted)" stroke-width="1.5" stroke-linecap="round"/>
        <rect x="48" y="20" width="14" height="18" rx="3" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="1.5"/>
        <circle cx="55" cy="32" r="2" fill="var(--text-muted)"/>
      </svg>
    </div>
    <h3 class="empty-title">{{ title ?? t('common.noData') }}</h3>
    <p class="empty-desc" v-if="description">{{ description }}</p>
    <slot />
  </div>
</template>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-12) var(--space-6);
  text-align: center;
}

.empty-illustration {
  margin-bottom: var(--space-4);
  opacity: 0.7;
}

.empty-title {
  font-size: var(--text-lg);
  font-weight: var(--font-medium);
  color: var(--text-secondary);
  margin-bottom: var(--space-2);
}

.empty-desc {
  font-size: var(--text-sm);
  color: var(--text-muted);
  max-width: 320px;
  line-height: 1.6;
}
</style>
