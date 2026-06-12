<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { NButton, NIcon, NSpin, useMessage, useDialog } from 'naive-ui'
import { AddOutline } from '@vicons/ionicons5'
import { useProvidersStore } from '@/stores/providers'
import { useApiKeysStore } from '@/stores/apiKeys'
import type { KeyGroup } from '@/stores/providers'
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
const dialog = useDialog()
const providersStore = useProvidersStore()
const apiKeysStore = useApiKeysStore()

const showKeyForm = ref(false)

onMounted(async () => {
  await Promise.all([
    providersStore.fetchProviders(),
    apiKeysStore.fetchKeys(),
  ])
})

function handleSearch(query: string) {
  providersStore.setSearch(query)
}

function handleViewGroup(group: KeyGroup) {
  router.push(`/keys/${group.provider.id}`)
}

async function handleDeleteGroup(group: KeyGroup) {
  dialog.warning({
    title: t('keyDetail.confirmDeleteTitle'),
    content: t('keyCard.deleteConfirm'),
    positiveText: t('keyDetail.delete'),
    negativeText: t('keyDetail.cancel'),
    onPositiveClick: async () => {
      for (const key of group.keys) {
        await apiKeysStore.deleteKey(key.id)
      }
      message.success(t('keys.deleteSuccess'))
      await apiKeysStore.fetchKeys()
    },
  })
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
          :value="providersStore.searchQuery"
          :placeholder="$t('keys.searchPlaceholder')"
          @search="handleSearch"
        />

        <div class="key-grid">
          <n-spin :show="apiKeysStore.loading" size="large">
            <template v-if="providersStore.filteredKeyGroups.length > 0">
              <KeyCard
                v-for="group in providersStore.filteredKeyGroups"
                :key="group.key"
                :group="group"
                @view="handleViewGroup"
                @delete="handleDeleteGroup"
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
  gap: var(--space-5);
}
</style>
