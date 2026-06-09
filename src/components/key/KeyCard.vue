<script setup lang="ts">
import { NIcon, NCard, NSpace, NButton, NPopconfirm } from 'naive-ui'
import { TrashOutline, CopyOutline } from '@vicons/ionicons5'
import type { ApiKeyView } from '@/types/apiKey'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  apiKey: ApiKeyView
}>()

const emit = defineEmits<{
  view: [id: number]
  delete: [id: number]
  copy: [id: number]
}>()

function formatDate(dateStr: string): string {
  return new Date(dateStr + 'Z').toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  })
}
</script>

<template>
  <n-card class="key-card" hoverable>
    <div class="card-header">
      <div class="card-title-row">
        <div class="card-title" @click="emit('view', apiKey.id)">{{ apiKey.name }}</div>
        <n-space :size="4" @click.stop>
          <n-button text size="small" @click="emit('copy', apiKey.id)">
            <n-icon :component="CopyOutline" />
          </n-button>
          <n-popconfirm @positive-click="emit('delete', apiKey.id)">
            <template #trigger>
              <n-button text size="small" type="error">
                <n-icon :component="TrashOutline" />
              </n-button>
            </template>
            {{ t('keyCard.deleteConfirm') }}
          </n-popconfirm>
        </n-space>
      </div>
      <div class="card-provider">{{ apiKey.provider_display_name }}</div>
    </div>

    <div class="card-body" @click="emit('view', apiKey.id)">
      <div class="masked-key">
        <code>{{ apiKey.masked_preview }}</code>
      </div>
      <div class="card-description" v-if="apiKey.description">
        {{ apiKey.description }}
      </div>
    </div>

    <div class="card-footer" @click="emit('view', apiKey.id)">
      <span class="card-date">{{ t('keyCard.updated', { date: formatDate(apiKey.updated_at) }) }}</span>
    </div>
  </n-card>
</template>

<style scoped>
.key-card {
  cursor: pointer;
  transition: all var(--transition-fast);
}

.key-card:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.card-header {
  margin-bottom: var(--space-3);
}

.card-title-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.card-title {
  flex: 1;
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.card-provider {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  margin-top: var(--space-1);
}

.card-body {
  margin-bottom: var(--space-3);
}

.masked-key code {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  background: var(--bg-elevated);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
}

.card-description {
  font-size: var(--text-xs);
  color: var(--text-muted);
  margin-top: var(--space-2);
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: var(--text-xs);
  color: var(--text-muted);
}
</style>
