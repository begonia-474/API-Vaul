<script setup lang="ts">
import { ref, computed, watch, onMounted, h } from 'vue'
import { NModal, NForm, NInput, NButton, NSpace, NSelect } from 'naive-ui'
import ProviderIcon from '@/components/provider/ProviderIcon.vue'
import { useApiKeysStore } from '@/stores/apiKeys'
import { useProvidersStore } from '@/stores/providers'
import type { ApiKeyView } from '@/types/apiKey'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const NEW_CARD_VALUE = '__new_card__'

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
  selected: null as number | string | null, // provider_id (existing) or NEW_CARD_VALUE
  provider_name: '',
  display_name: '',
  icon: '',
  openai_base_url: '',
  anthropic_base_url: '',
  description: '',
  name: '',
  raw_key: '',
})

const isNewCard = computed(() => form.value.selected === NEW_CARD_VALUE)
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
          selected: props.editKey.provider_id,
          provider_name: '',
          display_name: '',
          icon: '',
          openai_base_url: '',
          anthropic_base_url: '',
          description: '',
          name: props.editKey.name,
          raw_key: '',
        }
      } else {
        form.value = {
          selected: props.defaultProviderId ?? null,
          provider_name: '',
          display_name: '',
          icon: '',
          openai_base_url: '',
          anthropic_base_url: '',
          description: '',
          name: '',
          raw_key: '',
        }
      }
    }
  },
)

const builtInCategories = ['official', 'cn_official', 'cloud_provider', 'aggregator', 'third_party']

const providerOptions = computed(() => {
  // Built-in providers only
  const builtIn = providersStore.providers
    .filter((p) => p.category && builtInCategories.includes(p.category))
    .map((p) => ({
      label: p.display_name,
      value: p.id,
      icon: p.icon,
      presetId: p.preset_id,
    }))

  const options: any[] = [...builtIn]
  options.push({
    label: t('keyForm.newCard'),
    value: NEW_CARD_VALUE,
    icon: null,
    presetId: null,
  })

  return options
})

function renderLabel(option: { label: string; icon?: string | null; presetId?: string | null; value?: any }) {
  if (option.value === NEW_CARD_VALUE) {
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

// When selecting a built-in provider, prefill display name
watch(
  () => form.value.selected,
  (val) => {
    if (typeof val === 'number') {
      const p = providersStore.providers.find((p) => p.id === val)
      if (p) {
        // Check if this is a built-in provider (no keys yet)
        const hasKeys = apiKeysStore.keys.some((k) => k.provider_id === val)
        if (!hasKeys) {
          // Prefill for new card from built-in
          form.value.provider_name = p.name
          form.value.display_name = p.display_name
          form.value.icon = p.icon ?? ''
          form.value.openai_base_url = ''
          form.value.anthropic_base_url = ''
          form.value.description = ''
        }
      }
    }
  },
)

async function handleSubmit() {
  if (!form.value.selected) return
  if (!form.value.raw_key.trim()) return

  submitting.value = true
  try {
    let providerId: number

    if (isNewCard.value || isBuiltInSelection.value) {
      // Create new card
      if (!form.value.display_name.trim()) return
      const name = form.value.provider_name.trim() || form.value.display_name.trim().toLowerCase().replace(/\s+/g, '_')
      // Get preset_id from built-in provider if applicable
      let presetId: string | undefined
      if (isBuiltInSelection.value && typeof form.value.selected === 'number') {
        const builtin = providersStore.providers.find((p) => p.id === form.value.selected)
        presetId = builtin?.preset_id ?? undefined
      }
      const newProvider = await providersStore.createProvider({
        name,
        display_name: form.value.display_name.trim(),
        icon: form.value.icon.trim() || undefined,
        category: 'custom',
        openai_base_url: form.value.openai_base_url.trim() || undefined,
        anthropic_base_url: form.value.anthropic_base_url.trim() || undefined,
        description: form.value.description.trim() || undefined,
        preset_id: presetId,
      })
      if (!newProvider) return
      providerId = newProvider.id
    } else {
      return
    }

    const keyName = form.value.name.trim() || `Key-${Date.now().toString(36)}`

    if (props.editKey) {
      await apiKeysStore.updateKey({
        id: props.editKey.id,
        provider_id: providerId,
        name: keyName,
        raw_key: form.value.raw_key || undefined,
      })
    } else {
      await apiKeysStore.createKey({
        provider_id: providerId,
        name: keyName,
        raw_key: form.value.raw_key,
      })
    }

    emit('created')
    emit('update:show', false)
  } finally {
    submitting.value = false
  }
}

// Check if selected is a built-in provider (number, but no keys yet)
const isBuiltInSelection = computed(() => {
  if (typeof form.value.selected !== 'number') return false
  return !apiKeysStore.keys.some((k) => k.provider_id === form.value.selected)
})

// Show full form when creating new card
const showFullForm = computed(() => isNewCard.value || isBuiltInSelection.value)
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
          v-model:value="form.selected"
          :options="providerOptions"
          :render-label="renderLabel"
          :placeholder="t('keyForm.providerPlaceholder')"
          filterable
        />
      </n-form-item>

      <!-- New card fields -->
      <template v-if="showFullForm && !editKey">
        <n-form-item :label="t('keyForm.customDisplayName')">
          <n-input v-model:value="form.display_name" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.customName')">
          <n-input v-model:value="form.provider_name" placeholder=" " />
        </n-form-item>
        <n-form-item v-if="isNewCard" :label="t('keyForm.customIcon')">
          <n-input v-model:value="form.icon" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.openaiBaseUrl')">
          <n-input v-model:value="form.openai_base_url" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.anthropicBaseUrl')">
          <n-input v-model:value="form.anthropic_base_url" placeholder=" " />
        </n-form-item>
        <n-form-item :label="t('keyForm.description')">
          <n-input v-model:value="form.description" type="textarea" :rows="2" placeholder=" " />
        </n-form-item>
      </template>

      <n-form-item :label="t('keyDetail.keyName')">
        <n-input v-model:value="form.name" placeholder=" " />
      </n-form-item>

      <n-form-item label="API Key">
        <n-input
          v-model:value="form.raw_key"
          type="password"
          show-password-on="click"
          placeholder=" "
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
