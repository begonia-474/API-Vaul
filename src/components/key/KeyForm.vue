<script setup lang="ts">
import { ref, watch, onMounted, h } from 'vue'
import { NModal, NForm, NInput, NButton, NSpace } from 'naive-ui'
import ProviderIcon from '@/components/provider/ProviderIcon.vue'
import { useApiKeysStore } from '@/stores/apiKeys'
import { useProvidersStore } from '@/stores/providers'
import type { ApiKeyView } from '@/types/apiKey'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  show: boolean
  editKey?: ApiKeyView | null
}>()

const emit = defineEmits<{
  'update:show': [value: boolean]
  created: []
}>()

const apiKeysStore = useApiKeysStore()
const providersStore = useProvidersStore()

const form = ref({
  name: '',
  raw_key: '',
  provider_id: null as number | null,
  description: '',
  openai_base_url: '',
  anthropic_base_url: '',
})

const submitting = ref(false)

onMounted(() => {
  if (providersStore.providers.length === 0) {
    providersStore.fetchProviders()
  }
})

watch(
  () => props.show,
  async (val) => {
    if (val) {
      if (providersStore.providers.length === 0) {
        await providersStore.fetchProviders()
      }

      if (props.editKey) {
        form.value = {
          name: props.editKey.name,
          raw_key: '',
          provider_id: props.editKey.provider_id,
          description: props.editKey.description ?? '',
          openai_base_url: props.editKey.openai_base_url ?? '',
          anthropic_base_url: props.editKey.anthropic_base_url ?? '',
        }
      } else {
        form.value = { name: '', raw_key: '', provider_id: null, description: '', openai_base_url: '', anthropic_base_url: '' }
      }
    }
  },
)

const providerOptions = () =>
  providersStore.providers.map((p) => ({
    label: p.display_name,
    value: p.id,
    icon: p.icon,
    presetId: p.preset_id,
  }))

function renderProviderLabel(option: { label: string; icon?: string; presetId?: string }) {
  return h('div', { style: 'display: flex; align-items: center; gap: 8px;' }, [
    h(ProviderIcon, {
      name: option.icon ?? option.label,
      presetId: option.presetId ?? undefined,
      size: 20,
    }),
    h('span', null, option.label),
  ])
}

async function handleSubmit() {
  if (!form.value.name.trim() || !form.value.provider_id) return

  submitting.value = true
  try {
    if (props.editKey) {
      await apiKeysStore.updateKey({
        id: props.editKey.id,
        provider_id: form.value.provider_id,
        name: form.value.name,
        raw_key: form.value.raw_key || undefined,
        description: form.value.description || undefined,
        openai_base_url: form.value.openai_base_url || undefined,
        anthropic_base_url: form.value.anthropic_base_url || undefined,
      })
    } else {
      await apiKeysStore.createKey({
        provider_id: form.value.provider_id,
        name: form.value.name,
        raw_key: form.value.raw_key,
        description: form.value.description || undefined,
        openai_base_url: form.value.openai_base_url || undefined,
        anthropic_base_url: form.value.anthropic_base_url || undefined,
      })
    }

    emit('created')
    emit('update:show', false)
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <n-modal
    :show="show"
    @update:show="emit('update:show', $event)"
    preset="card"
    :title="editKey ? t('keyForm.editTitle') : t('keyForm.addTitle')"
    style="max-width: 500px"
  >
    <n-form label-placement="left" label-width="80">
      <n-form-item :label="t('keyForm.name')">
        <n-input v-model:value="form.name" :placeholder="t('keyForm.namePlaceholder')" />
      </n-form-item>
      <n-form-item :label="t('keyForm.provider')">
        <n-select
          v-model:value="form.provider_id"
          :options="providerOptions()"
          :render-label="renderProviderLabel"
          :placeholder="t('keyForm.providerPlaceholder')"
          filterable
        />
      </n-form-item>
      <n-form-item label="API Key">
        <n-input
          v-model:value="form.raw_key"
          type="password"
          show-password-on="click"
          :placeholder="editKey ? t('keyForm.apiKeyEditPlaceholder') : t('keyForm.apiKeyPlaceholder')"
        />
      </n-form-item>
      <n-form-item :label="t('keyForm.openaiBaseUrl')">
        <n-input v-model:value="form.openai_base_url" :placeholder="t('keyForm.openaiBaseUrlPlaceholder')" />
      </n-form-item>
      <n-form-item :label="t('keyForm.anthropicBaseUrl')">
        <n-input v-model:value="form.anthropic_base_url" :placeholder="t('keyForm.anthropicBaseUrlPlaceholder')" />
      </n-form-item>
      <n-form-item :label="t('keyForm.notes')">
        <n-input
          v-model:value="form.description"
          type="textarea"
          :placeholder="t('keyForm.notesPlaceholder')"
          :rows="3"
        />
      </n-form-item>
    </n-form>

    <template #action>
      <n-space justify="end">
        <n-button @click="emit('update:show', false)">{{ t('keyForm.cancel') }}</n-button>
        <n-button type="primary" :loading="submitting" @click="handleSubmit">
          {{ editKey ? t('keyForm.save') : t('keyForm.add') }}
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template>
