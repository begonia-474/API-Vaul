<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { NSelect } from 'naive-ui'
import { useProvidersStore } from '@/stores/providers'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  value: number | null
}>()

const emit = defineEmits<{
  'update:value': [value: number | null]
}>()

const providersStore = useProvidersStore()

onMounted(() => {
  if (providersStore.providers.length === 0) {
    providersStore.fetchProviders()
  }
})

const options = computed(() =>
  providersStore.providers.map((p) => ({
    label: p.display_name,
    value: p.id,
  })),
)

function handleChange(val: number | null) {
  emit('update:value', val)
}
</script>

<template>
  <n-select
    :value="value"
    :options="options"
    :placeholder="t('keyForm.providerPlaceholder')"
    filterable
    clearable
    @update:value="handleChange"
  />
</template>