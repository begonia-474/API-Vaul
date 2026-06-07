<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NButton, NIcon, NCard, NDescriptions, NDescriptionsItem, useMessage, useDialog } from 'naive-ui'
import { ArrowBackOutline, CopyOutline, CreateOutline, TrashOutline, KeyOutline } from '@vicons/ionicons5'
import { useApiKeysStore } from '@/stores/apiKeys'
import { useProvidersStore } from '@/stores/providers'
import { useI18n } from 'vue-i18n'
import AppHeader from '@/components/common/AppHeader.vue'
import KeyForm from '@/components/key/KeyForm.vue'
import ProviderIcon from '@/components/provider/ProviderIcon.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const message = useMessage()
const dialog = useDialog()
const apiKeysStore = useApiKeysStore()
const providersStore = useProvidersStore()

const showEditForm = ref(false)
const loadingKey = ref(false)
const keyId = computed(() => Number(route.params.id))

const apiKey = computed(() => apiKeysStore.getKeyById(keyId.value))
const provider = computed(() => {
  if (!apiKey.value) return null
  return providersStore.getProviderById(apiKey.value.provider_id)
})

onMounted(async () => {
  if (apiKeysStore.keys.length === 0) {
    await apiKeysStore.fetchKeys()
  }
  if (providersStore.providers.length === 0) {
    await providersStore.fetchProviders()
  }
})

async function handleCopyKey() {
  loadingKey.value = true
  const rawKey = await apiKeysStore.getDecryptedKey(keyId.value)
  loadingKey.value = false

  if (rawKey) {
    await navigator.clipboard.writeText(rawKey)
    message.success(t('keys.copySuccess'))
  } else {
    message.error(t('keys.copyFailed'))
  }
}

async function handleDelete() {
  dialog.warning({
    title: t('keyDetail.confirmDeleteTitle'),
    content: t('keyDetail.confirmDeleteContent'),
    positiveText: t('keyDetail.delete'),
    negativeText: t('keyDetail.cancel'),
    onPositiveClick: async () => {
      const ok = await apiKeysStore.deleteKey(keyId.value)
      if (ok) {
        router.push('/keys')
      }
    },
  })
}

async function handleCopyUrl(url: string) {
  try {
    await navigator.clipboard.writeText(url)
    message.success(t('keys.copySuccess'))
  } catch {
    message.error(t('keys.copyFailed'))
  }
}

function handleBack() {
  router.push('/keys')
}

function handleEditDone() {
  showEditForm.value = false
}

function formatDate(dateStr: string | null): string {
  if (!dateStr) return '—'
  return new Date(dateStr).toLocaleString('zh-CN')
}
</script>

<template>
  <div class="key-detail-page">
    <AppHeader :title="apiKey?.name ?? t('keyDetail.noKeySelected')">
      <template #actions>
        <n-button quaternary @click="handleBack">
          <template #icon><n-icon :component="ArrowBackOutline" /></template>
          {{ $t('keyDetail.back') }}
        </n-button>
      </template>
    </AppHeader>

    <div v-if="apiKey" class="detail-body">
      <n-card class="detail-card">
        <div class="detail-header">
          <div class="detail-title-row">
            <n-button type="primary" @click="handleCopyKey" :loading="loadingKey">
              <template #icon><n-icon :component="CopyOutline" /></template>
              {{ $t('keyDetail.copyKey') }}
            </n-button>
            <n-button @click="showEditForm = true">
              <template #icon><n-icon :component="CreateOutline" /></template>
              {{ $t('keyDetail.edit') }}
            </n-button>
            <n-button type="error" @click="handleDelete">
              <template #icon><n-icon :component="TrashOutline" /></template>
              {{ $t('keyDetail.delete') }}
            </n-button>
          </div>
        </div>

        <n-descriptions bordered :column="1" label-placement="left">
          <n-descriptions-item :label="$t('keyDetail.apiKey')">
            <code>{{ apiKey.masked_preview }}</code>
          </n-descriptions-item>
          <n-descriptions-item :label="$t('keyDetail.provider')">
            <span v-if="provider" class="provider-display">
              <ProviderIcon :name="provider.icon ?? provider.name" :preset-id="provider.preset_id ?? undefined" :size="20" />
              {{ provider.display_name }}
            </span>
            <span v-else>{{ apiKey.provider_display_name }}</span>
          </n-descriptions-item>
          <n-descriptions-item :label="$t('keyDetail.openaiBaseUrl')">
            <div class="url-row" v-if="apiKey.openai_base_url">
              <code>{{ apiKey.openai_base_url }}</code>
              <n-button quaternary size="tiny" @click="handleCopyUrl(apiKey.openai_base_url!)">
                <template #icon><n-icon :component="CopyOutline" /></template>
              </n-button>
            </div>
            <span v-else>—</span>
          </n-descriptions-item>
          <n-descriptions-item :label="$t('keyDetail.anthropicBaseUrl')">
            <div class="url-row" v-if="apiKey.anthropic_base_url">
              <code>{{ apiKey.anthropic_base_url }}</code>
              <n-button quaternary size="tiny" @click="handleCopyUrl(apiKey.anthropic_base_url!)">
                <template #icon><n-icon :component="CopyOutline" /></template>
              </n-button>
            </div>
            <span v-else>—</span>
          </n-descriptions-item>
          <n-descriptions-item :label="$t('keyDetail.description')">
            {{ apiKey.description || $t('keyDetail.noDescription') }}
          </n-descriptions-item>
          <n-descriptions-item :label="$t('keyDetail.createdAt')">
            {{ formatDate(apiKey.created_at) }}
          </n-descriptions-item>
          <n-descriptions-item :label="$t('keyDetail.updatedAt')">
            {{ formatDate(apiKey.updated_at) }}
          </n-descriptions-item>
        </n-descriptions>
      </n-card>
    </div>

    <div v-else class="detail-empty">
      <n-icon :component="KeyOutline" :size="48" color="var(--text-muted)" />
      <p>{{ $t('keyDetail.noKeySelected') }}</p>
      <n-button @click="handleBack">{{ $t('keyDetail.back') }}</n-button>
    </div>

    <KeyForm
      v-if="apiKey"
      v-model:show="showEditForm"
      :edit-key="apiKey"
      @created="handleEditDone"
    />
  </div>
</template>

<style scoped>
.key-detail-page {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100vh;
  background-color: var(--bg-app);
}

.detail-body {
  flex: 1;
  padding: var(--space-6);
  overflow-y: auto;
}

.detail-card {
  max-width: 800px;
}

.detail-header {
  margin-bottom: var(--space-6);
}

.detail-title-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.detail-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
  color: var(--text-muted);
}

code {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  background: var(--bg-elevated);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

.provider-display {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.url-row {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}
</style>
