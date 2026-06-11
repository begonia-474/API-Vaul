<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NButton, NIcon, NCard, NDescriptions, NDescriptionsItem, useMessage, useDialog } from 'naive-ui'
import {
  ArrowBackOutline,
  CopyOutline,
  TrashOutline,
  AddOutline,
  EyeOutline,
  EyeOffOutline,
} from '@vicons/ionicons5'
import { useApiKeysStore } from '@/stores/apiKeys'
import { useProvidersStore } from '@/stores/providers'
import { useI18n } from 'vue-i18n'
import AppHeader from '@/components/common/AppHeader.vue'
import ProviderIcon from '@/components/provider/ProviderIcon.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const message = useMessage()
const dialog = useDialog()
const apiKeysStore = useApiKeysStore()
const providersStore = useProvidersStore()

const providerId = computed(() => Number(route.params.id))
const description = computed(() => (route.query.desc as string) ?? '')
const provider = computed(() => providersStore.getProviderById(providerId.value))
const groupKeys = computed(() =>
  apiKeysStore.keys.filter(
    (k) => k.provider_id === providerId.value && (k.description ?? '') === description.value,
  ),
)
const pageTitle = computed(() => {
  if (!provider.value) return t('keyDetail.noProvider')
  return description.value
    ? `${provider.value.display_name} - ${description.value}`
    : provider.value.display_name
})

const showAddKey = ref(false)
const newKeyRaw = ref('')
const addKeyLoading = ref(false)
const revealedKeys = ref<Map<number, string>>(new Map())

onMounted(async () => {
  if (providersStore.providers.length === 0) {
    await providersStore.fetchProviders()
  }
  if (apiKeysStore.keys.length === 0) {
    await apiKeysStore.fetchKeys()
  }
})

function handleBack() {
  router.push('/keys')
}

async function handleCopyKey(id: number) {
  const rawKey = await apiKeysStore.getDecryptedKey(id)
  if (rawKey) {
    await navigator.clipboard.writeText(rawKey)
    message.success(t('keys.copySuccess'))
  } else {
    message.error(t('keys.copyFailed'))
  }
}

async function handleCopyUrl(url: string) {
  try {
    await navigator.clipboard.writeText(url)
    message.success(t('keys.copySuccess'))
  } catch {
    message.error(t('keys.copyFailed'))
  }
}

async function handleDeleteKey(id: number) {
  dialog.warning({
    title: t('keyDetail.confirmDeleteTitle'),
    content: t('keyDetail.confirmDeleteContent'),
    positiveText: t('keyDetail.delete'),
    negativeText: t('keyDetail.cancel'),
    onPositiveClick: async () => {
      const ok = await apiKeysStore.deleteKey(id)
      if (ok) {
        if (groupKeys.value.length === 0) {
          router.push('/keys')
        }
      }
    },
  })
}

async function toggleRevealKey(id: number) {
  if (revealedKeys.value.has(id)) {
    const copy = new Map(revealedKeys.value)
    copy.delete(id)
    revealedKeys.value = copy
    return
  }
  const rawKey = await apiKeysStore.getDecryptedKey(id)
  if (rawKey) {
    const copy = new Map(revealedKeys.value)
    copy.set(id, rawKey)
    revealedKeys.value = copy
  }
}

async function handleAddKey() {
  if (!newKeyRaw.value.trim()) return
  addKeyLoading.value = true
  const keyName = `Key-${Date.now().toString(36)}`
  const ok = await apiKeysStore.createKey({
    provider_id: providerId.value,
    name: keyName,
    raw_key: newKeyRaw.value.trim(),
    description: description.value || undefined,
  })
  addKeyLoading.value = false
  if (ok) {
    newKeyRaw.value = ''
    showAddKey.value = false
    message.success(t('keyDetail.keyAdded'))
  } else {
    message.error(t('keyDetail.keyAddFailed'))
  }
}
</script>

<template>
  <div class="key-detail-page">
    <AppHeader :title="pageTitle">
      <template #actions>
        <n-button quaternary @click="handleBack">
          <template #icon><n-icon :component="ArrowBackOutline" /></template>
          {{ t('keyDetail.back') }}
        </n-button>
      </template>
    </AppHeader>

    <div v-if="provider" class="detail-body">
      <!-- Provider info card -->
      <n-card class="detail-card">
        <div class="detail-header">
          <div class="detail-title-row">
            <ProviderIcon
              :name="provider.icon ?? provider.display_name"
              :preset-id="provider.preset_id ?? undefined"
              :size="24"
            />
            <h2 class="provider-name">{{ pageTitle }}</h2>
          </div>
        </div>

        <!-- Show base URLs from first key in group -->
        <n-descriptions v-if="groupKeys.length > 0" bordered :column="1" label-placement="left">
          <n-descriptions-item v-if="groupKeys[0].openai_base_url" :label="t('keyDetail.openaiBaseUrl')">
            <div class="url-row">
              <code>{{ groupKeys[0].openai_base_url }}</code>
              <n-button quaternary size="tiny" @click="handleCopyUrl(groupKeys[0].openai_base_url!)">
                <template #icon><n-icon :component="CopyOutline" /></template>
              </n-button>
            </div>
          </n-descriptions-item>
          <n-descriptions-item v-if="groupKeys[0].anthropic_base_url" :label="t('keyDetail.anthropicBaseUrl')">
            <div class="url-row">
              <code>{{ groupKeys[0].anthropic_base_url }}</code>
              <n-button quaternary size="tiny" @click="handleCopyUrl(groupKeys[0].anthropic_base_url!)">
                <template #icon><n-icon :component="CopyOutline" /></template>
              </n-button>
            </div>
          </n-descriptions-item>
          <n-descriptions-item v-if="groupKeys[0].description" :label="t('keyDetail.description')">
            {{ groupKeys[0].description }}
          </n-descriptions-item>
        </n-descriptions>
      </n-card>

      <!-- API keys card -->
      <n-card class="detail-card keys-card">
        <div class="keys-header">
          <h3>{{ t('keyDetail.apiKeys') }}</h3>
          <n-button text size="small" @click="showAddKey = !showAddKey">
            <template #icon><n-icon :component="AddOutline" /></template>
          </n-button>
        </div>

        <div v-if="groupKeys.length > 0" class="keys-list">
          <div v-for="apiKey in groupKeys" :key="apiKey.id" class="key-row">
            <div class="key-info">
              <span class="key-name">{{ apiKey.name }}</span>
              <code class="key-masked">{{ revealedKeys.get(apiKey.id) ?? apiKey.masked_preview }}</code>
              <n-button text size="small" @click="toggleRevealKey(apiKey.id)">
                <template #icon>
                  <n-icon :component="revealedKeys.has(apiKey.id) ? EyeOffOutline : EyeOutline" />
                </template>
              </n-button>
            </div>
            <div class="key-actions">
              <n-button text size="small" @click="handleCopyKey(apiKey.id)">
                <template #icon><n-icon :component="CopyOutline" /></template>
              </n-button>
              <n-button text size="small" type="error" @click="handleDeleteKey(apiKey.id)">
                <template #icon><n-icon :component="TrashOutline" /></template>
              </n-button>
            </div>
          </div>
        </div>
        <div v-else class="keys-empty">
          {{ t('keyDetail.noApiKeys') }}
        </div>

        <!-- Inline add form -->
        <div v-if="showAddKey" class="add-key-form">
          <n-input
            v-model:value="newKeyRaw"
            type="password"
            show-password-on="click"
            placeholder="API Key"
            size="small"
          />
          <n-button type="primary" size="small" :loading="addKeyLoading" @click="handleAddKey">
            {{ t('keyDetail.save') }}
          </n-button>
        </div>
      </n-card>
    </div>

    <div v-else class="detail-empty">
      <p>{{ t('keyDetail.noProvider') }}</p>
      <n-button @click="handleBack">{{ t('keyDetail.back') }}</n-button>
    </div>
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
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.detail-card {
  max-width: 800px;
}

.detail-header {
  margin-bottom: var(--space-4);
}

.detail-title-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.provider-name {
  flex: 1;
  margin: 0;
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
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

.url-row {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.keys-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

.keys-header h3 {
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  margin: 0;
}

.keys-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.key-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3);
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
}

.key-info {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.key-name {
  font-weight: var(--font-medium);
  color: var(--text-primary);
}

.key-masked {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  background: var(--bg-surface);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

.key-actions {
  display: flex;
  gap: var(--space-1);
}

.keys-empty {
  color: var(--text-muted);
  font-size: var(--text-sm);
}

.add-key-form {
  display: flex;
  gap: var(--space-3);
  margin-top: var(--space-4);
  align-items: center;
}

.add-key-form .n-input {
  flex: 1;
}
</style>
