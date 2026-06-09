<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NButton, NIcon, NCard, NDescriptions, NDescriptionsItem, useMessage, useDialog } from 'naive-ui'
import { ArrowBackOutline, CopyOutline, CreateOutline, TrashOutline, KeyOutline, AddOutline, EyeOutline, EyeOffOutline } from '@vicons/ionicons5'
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

const childKeys = ref<import('@/types/apiKey').ApiKeyView[]>([])
const showAddChild = ref(false)
const newChildName = ref('')
const newChildKey = ref('')
const addChildLoading = ref(false)
const revealedKeys = ref<Map<number, string>>(new Map())

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
  await fetchChildKeys()
})

async function fetchChildKeys() {
  childKeys.value = await apiKeysStore.getChildKeys(keyId.value)
}

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

async function handleAddChild() {
  if (!newChildName.value.trim() || !newChildKey.value.trim() || !apiKey.value) return
  addChildLoading.value = true
  const ok = await apiKeysStore.createKey({
    provider_id: apiKey.value.provider_id,
    name: newChildName.value.trim(),
    raw_key: newChildKey.value.trim(),
    parent_id: keyId.value,
  })
  addChildLoading.value = false
  if (ok) {
    newChildName.value = ''
    newChildKey.value = ''
    showAddChild.value = false
    await fetchChildKeys()
    message.success(t('keyDetail.childKeyAdded'))
  } else {
    message.error(t('keyDetail.childKeyAddFailed'))
  }
}

async function handleCopyChildKey(id: number) {
  const rawKey = await apiKeysStore.getDecryptedKey(id)
  if (rawKey) {
    await navigator.clipboard.writeText(rawKey)
    message.success(t('keys.copySuccess'))
  } else {
    message.error(t('keys.copyFailed'))
  }
}

async function handleDeleteChild(id: number) {
  dialog.warning({
    title: t('keyDetail.confirmDeleteTitle'),
    content: t('keyDetail.confirmDeleteContent'),
    positiveText: t('keyDetail.delete'),
    negativeText: t('keyDetail.cancel'),
    onPositiveClick: async () => {
      const ok = await apiKeysStore.deleteKey(id)
      if (ok) {
        await fetchChildKeys()
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

function formatDate(dateStr: string | null): string {
  if (!dateStr) return '—'
  return new Date(dateStr + 'Z').toLocaleString('zh-CN')
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
            <div class="key-value-row">
              <code>{{ revealedKeys.get(apiKey.id) ?? apiKey.masked_preview }}</code>
              <n-button quaternary size="tiny" @click="toggleRevealKey(apiKey.id)">
                <template #icon><n-icon :component="revealedKeys.has(apiKey.id) ? EyeOffOutline : EyeOutline" /></template>
              </n-button>
            </div>
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

      <!-- Child keys -->
      <n-card class="detail-card child-keys-card">
        <div class="child-keys-header">
          <h3>{{ $t('keyDetail.childKeys') }}</h3>
          <n-button text size="small" @click="showAddChild = !showAddChild">
            <template #icon><n-icon :component="AddOutline" /></template>
          </n-button>
        </div>

        <div v-if="childKeys.length > 0" class="child-keys-list">
          <div v-for="child in childKeys" :key="child.id" class="child-key-row">
            <div class="child-key-info">
              <span class="child-key-name">{{ child.name }}</span>
              <code class="child-key-masked">{{ revealedKeys.get(child.id) ?? child.masked_preview }}</code>
              <n-button text size="small" @click="toggleRevealKey(child.id)">
                <template #icon><n-icon :component="revealedKeys.has(child.id) ? EyeOffOutline : EyeOutline" /></template>
              </n-button>
            </div>
            <div class="child-key-actions">
              <n-button text size="small" @click="handleCopyChildKey(child.id)">
                <template #icon><n-icon :component="CopyOutline" /></template>
              </n-button>
              <n-button text size="small" type="error" @click="handleDeleteChild(child.id)">
                <template #icon><n-icon :component="TrashOutline" /></template>
              </n-button>
            </div>
          </div>
        </div>
        <div v-else class="child-keys-empty">
          {{ $t('keyDetail.noChildKeys') }}
        </div>

        <!-- Inline add form -->
        <div v-if="showAddChild" class="add-child-form">
          <n-input
            v-model:value="newChildName"
            :placeholder="$t('keyDetail.childNamePlaceholder')"
            size="small"
          />
          <n-input
            v-model:value="newChildKey"
            type="password"
            show-password-on="click"
            placeholder="API Key"
            size="small"
          />
          <n-button type="primary" size="small" :loading="addChildLoading" @click="handleAddChild">
            {{ $t('keyDetail.save') }}
          </n-button>
        </div>
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

.key-value-row {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.child-keys-card {
  margin-top: var(--space-4);
}

.child-keys-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

.child-keys-header h3 {
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  margin: 0;
}

.child-keys-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.child-key-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3);
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
}

.child-key-info {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.child-key-name {
  font-weight: var(--font-medium);
  color: var(--text-primary);
}

.child-key-masked {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  background: var(--bg-surface);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

.child-key-actions {
  display: flex;
  gap: var(--space-1);
}

.child-keys-empty {
  color: var(--text-muted);
  font-size: var(--text-sm);
}

.add-child-form {
  display: flex;
  gap: var(--space-3);
  margin-top: var(--space-4);
  align-items: center;
}

.add-child-form .n-input {
  flex: 1;
}
</style>
