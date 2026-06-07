<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NButton, NIcon, NInput } from 'naive-ui'
import { KeyOutline, LockClosedOutline } from '@vicons/ionicons5'
import { useAuthStore } from '@/stores/auth'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const router = useRouter()
const auth = useAuthStore()

const password = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const errorMsg = ref('')

onMounted(async () => {
  await auth.checkFirstRun()
})

async function handleSubmit() {
  errorMsg.value = ''

  if (auth.isFirstRun) {
    if (password.value.length < 6) {
      errorMsg.value = t('lock.errorMinLength')
      return
    }
    if (password.value !== confirmPassword.value) {
      errorMsg.value = t('lock.errorMismatch')
      return
    }
    loading.value = true
    try {
      const ok = await auth.setupPassword(password.value)
      if (ok) {
        router.push('/keys')
      } else {
        errorMsg.value = t('lock.errorSetupFailed', { error: '' })
      }
    } catch (err: any) {
      errorMsg.value = t('lock.errorSetupFailed', { error: err?.message || err?.toString() || '' })
    }
    loading.value = false
  } else {
    if (!password.value) {
      return
    }
    loading.value = true
    try {
      const ok = await auth.unlock(password.value)
      if (ok) {
        router.push('/keys')
      } else {
        errorMsg.value = t('lock.errorWrongPassword')
      }
    } catch (err: any) {
      errorMsg.value = t('lock.errorUnlockFailed', { error: err?.message || err?.toString() || '' })
    }
    loading.value = false
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') handleSubmit()
}
</script>

<template>
  <div class="lock-screen">
    <div class="lock-card">
      <div class="lock-icon">
        <n-icon size="48" :component="auth.isFirstRun ? KeyOutline : LockClosedOutline" />
      </div>

      <h1 class="lock-title">{{ auth.isFirstRun ? $t('lock.setTitle') : $t('lock.unlockTitle') }}</h1>
      <p class="lock-subtitle">
        {{ auth.isFirstRun
          ? $t('lock.setSubtitle')
          : $t('lock.unlockSubtitle')
        }}
      </p>

      <div class="lock-form">
        <n-input
          v-model:value="password"
          type="password"
          show-password-on="click"
          :placeholder="$t('lock.password')"
          size="large"
          :input-props="{ autocomplete: 'current-password' }"
          @keydown="handleKeydown"
        />

        <n-input
          v-if="auth.isFirstRun"
          v-model:value="confirmPassword"
          type="password"
          show-password-on="click"
          :placeholder="$t('lock.confirmPassword')"
          size="large"
          @keydown="handleKeydown"
        />

        <p v-if="errorMsg" class="error-msg">{{ errorMsg }}</p>

        <n-button
          type="primary"
          size="large"
          block
          :loading="loading"
          @click="handleSubmit"
        >
          {{ auth.isFirstRun ? $t('lock.setButton') : $t('lock.unlockButton') }}
        </n-button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.lock-screen {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100vw;
  height: 100vh;
  background-color: var(--bg-app);
  background-image:
    radial-gradient(ellipse at 50% 0%, rgba(99, 102, 241, 0.08) 0%, transparent 60%);
}

.lock-card {
  width: 100%;
  max-width: 400px;
  padding: var(--space-10);
  text-align: center;
}

.lock-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 80px;
  height: 80px;
  margin: 0 auto var(--space-6);
  border-radius: var(--radius-xl);
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.lock-title {
  font-size: var(--text-2xl);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  margin-bottom: var(--space-2);
}

.lock-subtitle {
  font-size: var(--text-sm);
  color: var(--text-muted);
  margin-bottom: var(--space-8);
  line-height: 1.6;
}

.lock-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.error-msg {
  font-size: var(--text-sm);
  color: var(--color-danger);
  text-align: left;
  margin: calc(var(--space-1) * -1) 0 0;
  word-break: break-all;
}
</style>