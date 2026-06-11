<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NButton, NIcon, NCard, NDescriptions, NDescriptionsItem, NInput, useMessage, useDialog } from 'naive-ui'
import {
  ArrowBackOutline,
  CopyOutline,
  TrashOutline,
  AddOutline,
  EyeOutline,
  EyeOffOutline,
  CreateOutline,
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
const provider = computed(() => providersStore.getProviderById(providerId.value))
const groupKeys = computed(() =>
  apiKeysStore.keys.filter((k) => k.provider_id === providerId.value),
)
const pageTitle = computed(() => {
  if (!provider.value) return t('keyDetail.noProvider')
  const desc = provider.value.description?.trim()
  return desc
    ? `${provider.value.display_name} - ${desc}`
    : provider.value.display_name
})

const showAddKey = ref(false)
const newKeyName = ref('')
const newKeyRaw = ref('')
const addKeyLoading = ref(false)
const revealedKeys = ref<Map<number, string>>(new Map())

// Provider info editing
const editingProviderInfo = ref(false)
const providerEditForm = ref({ name: '', display_name: '', openai_base_url: '', anthropic_base_url: '', description: '' })

// Key editing
const editingKeyId = ref<number | null>(null)
const editForm = ref({ name: '', raw_key: '' })

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

// Provider info editing
function startEditProviderInfo() {
  if (!provider.value) return
  providerEditForm.value = {
    name: provider.value.name ?? '',
    display_name: provider.value.display_name ?? '',
    openai_base_url: provider.value.openai_base_url ?? '',
    anthropic_base_url: provider.value.anthropic_base_url ?? '',
    description: provider.value.description ?? '',
  }
  editingProviderInfo.value = true
}

function cancelEditProviderInfo() {
  editingProviderInfo.value = false
}

async function saveProviderInfo() {
  const ok = await providersStore.updateProviderMetadata(
    providerId.value,
    providerEditForm.value.name || undefined,
    providerEditForm.value.display_name || undefined,
    providerEditForm.value.openai_base_url || undefined,
    providerEditForm.value.anthropic_base_url || undefined,
    providerEditForm.value.description || undefined,
  )
  if (ok) {
    editingProviderInfo.value = false
    message.success(t('keyDetail.metadataSaved'))
  }
}

// Key editing
function startEditKey(key: any) {
  editingKeyId.value = key.id
  editForm.value = {
    name: key.name,
    raw_key: '',
  }
}

function cancelEditKey() {
  editingKeyId.value = null
}

async function saveEditKey(key: any) {
  const ok = await apiKeysStore.updateKey({
    id: key.id,
    provider_id: key.provider_id,
    name: editForm.value.name,
    raw_key: editForm.value.raw_key || undefined,
  })
  if (ok) {
    editingKeyId.value = null
    message.success(t('keyDetail.keyUpdated'))
  } else {
    message.error(t('keyDetail.keyUpdateFailed'))
  }
}

async function handleAddKey() {
  if (!newKeyRaw.value.trim()) return
  addKeyLoading.value = true
  const keyName = newKeyName.value.trim() || `Key-${Date.now().toString(36)}`
  const ok = await apiKeysStore.createKey({
    provider_id: providerId.value,
    name: keyName,
    raw_key: newKeyRaw.value.trim(),
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
            <n-button v-if="!editingProviderInfo" text size="small" @click="startEditProviderInfo">
              <template #icon><n-icon :component="CreateOutline" /></template>
            </n-button>
          </div>
        </div>

        <!-- View mode -->
        <template v-if="!editingProviderInfo">
          <n-descriptions v-if="provider" bordered :column="1" label-placement="left">
            <n-descriptions-item :label="t('keyDetail.providerName')">
              {{ provider.display_name }}
            </n-descriptions-item>
            <n-descriptions-item :label="t('keyDetail.identifier')">
              <code>{{ provider.name }}</code>
            </n-descriptions-item>
            <n-descriptions-item v-if="provider.openai_base_url" :label="t('keyDetail.openaiBaseUrl')">
              <div class="url-row">
                <code>{{ provider.openai_base_url }}</code>
                <n-button quaternary size="tiny" @click="handleCopyUrl(provider.openai_base_url!)">
                  <template #icon><n-icon :component="CopyOutline" /></template>
                </n-button>
              </div>
            </n-descriptions-item>
            <n-descriptions-item v-if="provider.anthropic_base_url" :label="t('keyDetail.anthropicBaseUrl')">
              <div class="url-row">
                <code>{{ provider.anthropic_base_url }}</code>
                <n-button quaternary size="tiny" @click="handleCopyUrl(provider.anthropic_base_url!)">
                  <template #icon><n-icon :component="CopyOutline" /></template>
                </n-button>
              </div>
            </n-descriptions-item>
            <n-descriptions-item v-if="provider.description" :label="t('keyDetail.description')">
              {{ provider.description }}
            </n-descriptions-item>
          </n-descriptions>
        </template>

        <!-- Edit mode -->
        <template v-else>
          <div class="edit-provider-form">
            <div class="edit-field">
              <label>{{ t('keyDetail.providerName') }}</label>
              <n-input v-model:value="providerEditForm.display_name" placeholder=" " size="small" />
            </div>
            <div class="edit-field">
              <label>{{ t('keyDetail.identifier') }}</label>
              <n-input v-model:value="providerEditForm.name" placeholder=" " size="small" />
            </div>
            <div class="edit-field">
              <label>{{ t('keyDetail.openaiBaseUrl') }}</label>
              <n-input v-model:value="providerEditForm.openai_base_url" placeholder=" " size="small" />
            </div>
            <div class="edit-field">
              <label>{{ t('keyDetail.anthropicBaseUrl') }}</label>
              <n-input v-model:value="providerEditForm.anthropic_base_url" placeholder=" " size="small" />
            </div>
            <div class="edit-field">
              <label>{{ t('keyDetail.description') }}</label>
              <n-input v-model:value="providerEditForm.description" placeholder=" " size="small" />
            </div>
            <div class="edit-actions">
              <n-button size="small" @click="cancelEditProviderInfo">{{ t('keyDetail.cancel') }}</n-button>
              <n-button type="primary" size="small" @click="saveProviderInfo">{{ t('keyDetail.save') }}</n-button>
            </div>
          </div>
        </template>
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
          <template v-for="apiKey in groupKeys" :key="apiKey.id">
            <!-- View mode -->
            <div v-if="editingKeyId !== apiKey.id" class="key-row">
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
                <n-button text size="small" @click="startEditKey(apiKey)">
                  <template #icon><n-icon :component="CreateOutline" /></template>
                </n-button>
                <n-button text size="small" type="error" @click="handleDeleteKey(apiKey.id)">
                  <template #icon><n-icon :component="TrashOutline" /></template>
                </n-button>
              </div>
            </div>

            <!-- Edit mode -->
            <div v-else class="key-edit-row">
              <div class="edit-field">
                <label>{{ t('keyDetail.keyName') }}</label>
                <n-input v-model:value="editForm.name" placeholder=" " size="small" />
              </div>
              <div class="edit-field">
                <label>API Key</label>
                <n-input v-model:value="editForm.raw_key" type="password" show-password-on="click" :placeholder="t('keyDetail.newApiKey')" size="small" />
              </div>
              <div class="edit-actions">
                <n-button size="small" @click="cancelEditKey">{{ t('keyDetail.cancel') }}</n-button>
                <n-button type="primary" size="small" @click="saveEditKey(apiKey)">{{ t('keyDetail.save') }}</n-button>
              </div>
            </div>
          </template>
        </div>
        <div v-else class="keys-empty">
          {{ t('keyDetail.noApiKeys') }}
        </div>

        <!-- Inline add form -->
        <div v-if="showAddKey" class="add-key-form">
          <n-input
            v-model:value="newKeyName"
            :placeholder="t('keyDetail.keyName')"
            size="small"
          />
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
  min-width: 0;
}

.edit-provider-form,
.key-edit-row {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-3);
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
}

.edit-field {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.edit-field label {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--text-secondary);
}

.edit-actions {
  display: flex;
  gap: var(--space-2);
  justify-content: flex-end;
  margin-top: var(--space-2);
}
</style>
