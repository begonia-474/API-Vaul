<script setup lang="ts">
import { NButton, NIcon, useMessage } from 'naive-ui'
import { CopyOutline, CheckmarkOutline } from '@vicons/ionicons5'
import { useClipboard } from '@/composables/useClipboard'

const props = defineProps<{
  text: string
  size?: 'tiny' | 'small' | 'medium' | 'large'
  label?: string
}>()

const message = useMessage()
const { copy, copied } = useClipboard()

async function handleCopy() {
  const ok = await copy(props.text)
  if (ok) {
    message.success('已复制到剪贴板')
  } else {
    message.error('复制失败')
  }
}
</script>

<template>
  <n-button
    :size="size ?? 'tiny'"
    quaternary
    :type="copied ? 'success' : 'default'"
    @click.stop="handleCopy"
  >
    <template #icon>
      <n-icon :component="copied ? CheckmarkOutline : CopyOutline" />
    </template>
    <span v-if="label">{{ label }}</span>
  </n-button>
</template>
