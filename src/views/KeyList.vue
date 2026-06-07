<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { NButton, NIcon, NSpin, useMessage } from 'naive-ui'
import { AddOutline } from '@vicons/ionicons5'
import { useApiKeysStore } from '@/stores/apiKeys'
import { useProvidersStore } from '@/stores/providers'
import { useI18n } from 'vue-i18n'
import AppHeader from '@/components/common/AppHeader.vue'
import AppSidebar from '@/components/common/AppSidebar.vue'
import SearchBar from '@/components/common/SearchBar.vue'
import KeyCard from '@/components/key/KeyCard.vue'
import KeyForm from '@/components/key/KeyForm.vue'
import EmptyState from '@/components/common/EmptyState.vue'

const { t } = useI18n()
const router = useRouter()
const message = useMessage()
const apiKeysStore = useApiKeysStore()
const providersStore = useProvidersStore()

const showKeyForm = ref(false)

onMounted(async () => {
  await Promise.all([
    apiKeysStore.fetchKeys(),
    providersStore.fetchProviders(),
  ])
})

function handleSearch(query: string) {
  apiKeysStore.setSearch(query)
}

function handleViewKey(id: number) {
  router.push(`/keys/${id}`)
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

function handleKeyCreated() {
  showKeyForm.value = false
  apiKeysStore.fetchKeys()
}
</script>

<template>
  <div class="key-list-page">
    <AppSidebar />

    <div class="main-content">
      <AppHeader :title="$t('keys.title')">
        <template #actions>
          <n-button type="primary" @click="showKeyForm = true">
            <template #icon><n-icon :component="AddOutline" /></template>
            {{ $t('keys.addKey') }}
          </n-button>
        </template>
      </AppHeader>

      <div class="content-body">
        <SearchBar
          :value="apiKeysStore.searchQuery"
          :placeholder="$t('keys.searchPlaceholder')"
          @search="handleSearch"
        />

        <div class="key-grid">
          <n-spin :show="apiKeysStore.loading" size="large">
            <template v-if="apiKeysStore.filteredKeys.length > 0">
              <KeyCard
                v-for="key in apiKeysStore.filteredKeys"
                :key="key.id"
                :api-key="key"
                @view="handleViewKey"
                @delete="apiKeysStore.deleteKey"
                @copy="handleCopyKey"
              />
            </template>
            <EmptyState
              v-else
              :title="$t('keys.noKeys')"
              :description="$t('keys.noKeysDesc')"
            />
          </n-spin>
        </div>
      </div>
    </div>

    <KeyForm v-model:show="showKeyForm" @created="handleKeyCreated" />
  </div>
</template>

<style scoped>
.key-list-page {
  display: flex;
  width: 100%;
  height: 100vh;
  background-color: var(--bg-app);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
  position: relative;
}

.content-body {
  flex: 1;
  padding: var(--space-6);
  overflow-y: auto;
}

.key-grid {
  margin-top: var(--space-4);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
</style>
