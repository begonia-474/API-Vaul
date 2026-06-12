<script setup lang="ts">
import { NIcon, NCard, NButton, NPopconfirm } from 'naive-ui'
import { TrashOutline } from '@vicons/ionicons5'
import ProviderIcon from '@/components/provider/ProviderIcon.vue'
import type { KeyGroup } from '@/stores/providers'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  group: KeyGroup
}>()

const emit = defineEmits<{
  view: [group: KeyGroup]
  delete: [group: KeyGroup]
}>()

const displayTitle = props.group.description
  ? `${props.group.provider.display_name} - ${props.group.description}`
  : props.group.provider.display_name
</script>

<template>
  <n-card class="key-card" hoverable @click="emit('view', group)">
    <div class="card-header">
      <div class="card-title-row">
        <div class="card-icon">
          <ProviderIcon
            :name="group.provider.icon ?? group.provider.display_name"
            :preset-id="group.provider.preset_id ?? undefined"
            :size="20"
          />
        </div>
        <div class="card-text">
          <div class="card-title">{{ displayTitle }}</div>
        </div>
        <n-popconfirm @positive-click.stop="emit('delete', group)">
          <template #trigger>
            <n-button text size="small" type="error" @click.stop>
              <n-icon :component="TrashOutline" />
            </n-button>
          </template>
          {{ t('keyCard.deleteConfirm') }}
        </n-popconfirm>
      </div>
    </div>
  </n-card>
</template>

<style scoped>
.key-card {
  cursor: pointer;
  transition: all var(--transition-normal);
}

:deep(.n-card) {
  border-radius: var(--radius-xl) !important;
  border: 1px solid var(--border-default);
}

:deep(.n-card:hover) {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 1px var(--color-primary), var(--shadow-sm);
}

:deep(.n-card__content) {
  padding: var(--space-4) !important;
}

.card-header {
  display: flex;
  align-items: center;
}

.card-title-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  width: 100%;
}

.card-icon {
  flex-shrink: 0;
}

.card-text {
  flex: 1;
  min-width: 0;
}

.card-title {
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}
</style>
