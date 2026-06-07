<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NCard, NForm, NFormItem, NInput, NInputNumber, NSelect, NButton, NSpace, useMessage } from 'naive-ui'
import AppHeader from '@/components/common/AppHeader.vue'
import AppSidebar from '@/components/common/AppSidebar.vue'
import { useAuthStore } from '@/stores/auth'
import { useSettingsStore } from '@/stores/settings'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { LOCALE_OPTIONS, type SupportedLocale } from '@/locales'

const { t } = useI18n()
const auth = useAuthStore()
const settingsStore = useSettingsStore()
const router = useRouter()
const message = useMessage()

const lockTimeout = ref(5)
const theme = ref<'dark' | 'light' | 'system'>('dark')
const currentLanguage = ref<SupportedLocale>('en')
const loading = ref(false)

const currentPassword = ref('')
const newPassword = ref('')
const confirmPassword = ref('')
const passwordLoading = ref(false)

const themeOptions = computed(() => [
  { label: t('settings.themeDark'), value: 'dark' },
  { label: t('settings.themeLight'), value: 'light' },
  { label: t('settings.themeSystem'), value: 'system' },
])

const languageOptions = LOCALE_OPTIONS

onMounted(async () => {
  if (!settingsStore.initialized) {
    await settingsStore.fetchSettings()
  }
  theme.value = settingsStore.theme
  lockTimeout.value = settingsStore.autoLockMinutes
  currentLanguage.value = settingsStore.language
})

async function handleSave() {
  loading.value = true
  try {
    const ok = await settingsStore.updateSettings({
      theme: theme.value,
      auto_lock_minutes: lockTimeout.value,
      language: currentLanguage.value,
    })
    if (ok) {
      message.success(t('settings.settingsSaved'))
    } else {
      message.error(t('settings.settingsSaveFailed'))
    }
  } catch (err) {
    console.error('Failed to save settings:', err)
    message.error(t('settings.settingsSaveFailed'))
  } finally {
    loading.value = false
  }
}

async function handleChangePassword() {
  if (!currentPassword.value) {
    message.warning(t('settings.currentPasswordPlaceholder'))
    return
  }
  if (newPassword.value.length < 6) {
    message.warning(t('settings.newPasswordPlaceholder'))
    return
  }
  if (newPassword.value !== confirmPassword.value) {
    message.warning(t('settings.confirmPasswordPlaceholder'))
    return
  }

  passwordLoading.value = true
  try {
    const ok = await auth.changePassword(currentPassword.value, newPassword.value)
    if (ok) {
      currentPassword.value = ''
      newPassword.value = ''
      confirmPassword.value = ''
      message.success(t('settings.passwordUpdated'))
    } else {
      message.error(t('settings.passwordUpdateFailed'))
    }
  } catch (err) {
    console.error('Failed to change password:', err)
    message.error(t('settings.passwordUpdateError'))
  } finally {
    passwordLoading.value = false
  }
}

function handleLock() {
  auth.lock()
  router.push('/')
}
</script>

<template>
  <div class="settings-page">
    <AppSidebar />

    <div class="main-content">
      <AppHeader :title="$t('settings.title')" />

      <div class="content-body">
        <n-card :title="$t('settings.security')">
          <n-form label-placement="left" label-width="180">
            <n-form-item :label="$t('settings.autoLock')">
              <n-input-number
                v-model:value="lockTimeout"
                :min="1"
                :max="120"
                placeholder="5"
              />
            </n-form-item>
          </n-form>
          <n-space>
            <n-button type="warning" @click="handleLock">{{ $t('settings.lockNow') }}</n-button>
          </n-space>
        </n-card>

        <n-card :title="$t('settings.changePassword')" style="margin-top: var(--space-4)">
          <n-form label-placement="left" label-width="180">
            <n-form-item :label="$t('settings.currentPassword')">
              <n-input
                v-model:value="currentPassword"
                type="password"
                show-password-on="click"
                :placeholder="$t('settings.currentPasswordPlaceholder')"
              />
            </n-form-item>
            <n-form-item :label="$t('settings.newPassword')">
              <n-input
                v-model:value="newPassword"
                type="password"
                show-password-on="click"
                :placeholder="$t('settings.newPasswordPlaceholder')"
              />
            </n-form-item>
            <n-form-item :label="$t('settings.confirmPassword')">
              <n-input
                v-model:value="confirmPassword"
                type="password"
                show-password-on="click"
                :placeholder="$t('settings.confirmPasswordPlaceholder')"
              />
            </n-form-item>
          </n-form>
          <n-space justify="end">
            <n-button type="primary" :loading="passwordLoading" @click="handleChangePassword">{{ $t('settings.updatePassword') }}</n-button>
          </n-space>
        </n-card>

        <n-card :title="$t('settings.appearance')" style="margin-top: var(--space-4)">
          <n-form label-placement="left" label-width="180">
            <n-form-item :label="$t('settings.theme')">
              <n-select v-model:value="theme" :options="themeOptions" />
            </n-form-item>
            <n-form-item :label="$t('settings.language')">
              <n-select v-model:value="currentLanguage" :options="languageOptions" />
            </n-form-item>
          </n-form>
        </n-card>

        <n-space style="margin-top: var(--space-4)">
          <n-button type="primary" :loading="loading" @click="handleSave">{{ $t('settings.saveSettings') }}</n-button>
        </n-space>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
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
}

.content-body {
  flex: 1;
  padding: var(--space-6);
  overflow-y: auto;
}
</style>
