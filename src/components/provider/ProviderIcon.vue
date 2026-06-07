<script setup lang="ts">
import { computed } from 'vue'
import { NIcon } from 'naive-ui'
import { GlobeOutline } from '@vicons/ionicons5'
import ProviderSvgIcon from './ProviderSvgIcon.vue'

const props = defineProps<{
  name: string | null
  presetId?: string | null
  size?: number
}>()

const presetIdMap: Record<string, string> = {
  openai: 'openai',
  anthropic: 'anthropic',
  google_gemini: 'google_gemini',
  google: 'google',
  azure_openai: 'azure_openai',
  aws_bedrock: 'aws_bedrock',
  deepseek: 'deepseek',
  qwen: 'qwen',
  zhipu: 'zhipu',
  moonshot_kimi: 'moonshot_kimi',
  moonshot: 'moonshot',
  wenxin: 'wenxin',
  spark: 'spark',
  yi: 'yi',
  minimax: 'minimax',
  baichuan: 'baichuan',
  cohere: 'cohere',
  mistral: 'mistral',
  openrouter: 'openrouter',
  dify: 'dify',
}

const resolvedPresetId = computed(() => {
  if (props.presetId && presetIdMap[props.presetId]) return props.presetId
  const n = (props.name ?? '').toLowerCase().trim()
  return presetIdMap[n] ?? null
})

const hasSvg = computed(() => !!resolvedPresetId.value)

function isLikelyEmoji(value: string): boolean { for (const c of value) { const code = c.codePointAt(0) ?? 0; if (code > 0x2300) return true; } return false; }

const normalized = computed(() => (props.name ?? '').trim())

const isEmoji = computed(() => normalized.value.length > 0 && isLikelyEmoji(normalized.value))

const iconSize = computed(() => props.size ?? 24)
</script>

<template>
  <div
    class="provider-icon"
    :style="{
      width: iconSize + 'px',
      height: iconSize + 'px',
      fontSize: isEmoji ? iconSize * 0.55 + 'px' : iconSize * 0.46 + 'px',
    }"
  >
    <ProviderSvgIcon v-if="hasSvg && resolvedPresetId" :preset-id="resolvedPresetId" :size="iconSize * 0.6" />
    <span v-else-if="isEmoji" class="icon-emoji">{{ normalized }}</span>
    <span v-else-if="normalized" class="icon-letter">{{ normalized.charAt(0).toUpperCase() }}</span>
    <n-icon v-else :component="GlobeOutline" :size="iconSize * 0.6" />
  </div>
</template>

<style scoped>
.provider-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  flex-shrink: 0;
  color: var(--text-primary);
}

.icon-emoji {
  line-height: 1;
}

.icon-letter {
  font-weight: var(--font-bold);
  line-height: 1;
}
</style>
