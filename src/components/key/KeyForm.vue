<script setup lang="ts">
import { ref, computed, watch, onMounted, h } from 'vue'
import { NModal, NForm, NInput, NButton, NSpace, NSelect } from 'naive-ui'
import ProviderIcon from '@/components/provider/ProviderIcon.vue'
import { useApiKeysStore } from '@/stores/apiKeys'
import { useProvidersStore } from '@/stores/providers'
import type { ApiKeyView } from '@/types/apiKey'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const CUSTOM_PROVIDER_VALUE = '__custom__'

const props = defineProps<{
  show: boolean
  editKey?: ApiKeyView | null
  defaultProviderId?: number | null
}>()

const emit = defineEmits<{
  'update:show': [value: boolean]
  created: []
}>()

const apiKeysStore = useApiKeysStore()
const providersStore = useProvidersStore()

const form = ref({
  provider_id: null as number | string | null,
  raw_key: '',
  custom_name: '',
  custom_display_name: '',
  custom_icon: '',
  openai_base_url: '',
  anthropic_base_url: '',
  description: '',
})

const isCustomProvider = computed(() => form.value.provider_id === CUSTOM_PROVIDER_VALUE)
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
          provider_id: props.editKey.provider_id,
          raw_key: '',
          custom_name: '',
          custom_display_name: '',
          custom_icon: '',
          openai_base_url: '',
          anthropic_base_url: '',
          description: '',
        }
      } else {
        form.value = {
          provider_id: props.defaultProviderId ?? null,
          raw_key: '',
          custom_name: '',
          custom_display_name: '',
          custom_icon: '',
          openai_base_url: '',
          anthropic_base_url: '',
          description: '',
        }
      }
    }
  },
)

const providerOptions = computed(() => {
  const builtInCategories = ['official', 'cn_official', 'cloud_provider', 'aggregator', 'third_party']
  const builtIn = providersStore.providers
    .filter((p) => p.category && builtInCategories.includes(p.category))
    .map((p) => ({
      label: p.display_name,
      value: p.id,
      icon: p.icon,
      presetId: p.preset_id,
    }))

  return [
    ...builtIn,
    {
      label: t('keyForm.addCustomProvider'),
      value: CUSTOM_PROVIDER_VALUE,
      icon: null,
      presetId: null,
    },
  ]
})

function renderLabel(option: { label: string; icon?: string | null; presetId?: string | null; value?: any }) {
  if (option.value === CUSTOM_PROVIDER_VALUE) {
    return h('div', { style: 'display: flex; align-items: center; gap: 8px; color: var(--primary-color);' }, [
      h('span', { style: 'font-size: 16px;' }, '+'),
      h('span', null, option.label),
    ])
  }
  if (!option.icon) {
    return h('span', null, option.label)
  }
  return h('div', { style: 'display: flex; align-items: center; gap: 8px;' }, [
    h(ProviderIcon, {
      name: option.icon,
      presetId: option.presetId ?? undefined,
      size: 20,
    }),
    h('span', null, option.label),
  ])
}

async function handleSubmit() {
  if (!form.value.provider_id) return
  if (!form.value.raw_key.trim()) return

  submitting.value = true
  try {
    let providerId: number

    if (isCustomProvider.value) {
      if (!form.value.custom_display_name.trim()) return

      const name = form.value.custom_name.trim() || form.value.custom_display_name.trim().toLowerCase().replace(/\s+/g, '_')
      const newProvider = await providersStore.createProvider({
        name,
        display_name: form.value.custom_display_name.trim(),
        icon: form.value.custom_icon.trim() || undefined,
        category: 'custom',
        openai_base_url: form.value.openai_base_url.trim() || undefined,
        anthropic_base_url: form.value.anthropic_base_url.trim() || undefined,
        description: form.value.description.trim() || undefined,
      })
      if (!newProvider) return
      providerId = newProvider.id
    } else {
      providerId = form.value.provider_id as number
    }

    if (props.editKey) {
      await apiKeysStore.updateKey({
        id: props.editKey.id,
        provider_id: providerId,
        name: props.editKey.name,
        raw_key: form.value.raw_key || undefined,
        description: form.value.description.trim() || undefined,
        openai_base_url: form.value.openai_base_url.trim() || undefined,
        anthropic_base_url: form.value.anthropic_base_url.trim() || undefined,
      })
    } else {
      const keyName = `Key-${Date.now().toString(36)}`
      await apiKeysStore.createKey({
        provider_id: providerId,
        name: keyName,
        raw_key: form.value.raw_key,
        description: form.value.description.trim() || undefined,
        openai_base_url: form.value.openai_base_url.trim() || undefined,
        anthropic_base_url: form.value.anthropic_base_url.trim() || undefined,
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
    <n-form label-placement="left" label-width="120">
      <n-form-item :label="t('keyForm.provider')">
        <n-select
          v-model:value="form.provider_id"
          :options="providerOptions"
          :render-label="renderLabel"
          :placeholder="t('keyForm.providerPlaceholder')"
          filterable
        />
      </n-form-item>

      <!-- Custom provider fields -->
      <template v-if="isCustomProvider">
        <n-form-item :label="t('keyForm.customDisplayName')">
          <n-input v-model:value="form.custom_display_name" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.customName')">
          <n-input v-model:value="form.custom_name" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.customIcon')">
          <n-input v-model:value="form.custom_icon" placeholder=" " />
        </n-form-item>
      </template>

      <n-form-item label="API Key">
        <n-input
          v-model:value="form.raw_key"
          type="password"
          show-password-on="click"
          placeholder=" "
        />
      </n-form-item>

      <!-- Base URLs and description -->
      <template v-if="form.provider_id">
        <n-form-item :label="t('keyForm.openaiBaseUrl')">
          <n-input v-model:value="form.openai_base_url" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.anthropicBaseUrl')">
          <n-input v-model:value="form.anthropic_base_url" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.description')">
          <n-input
            v-model:value="form.description"
            type="textarea"
            :rows="2"
            placeholder=" "
          />
        </n-form-item>
      </template>
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
