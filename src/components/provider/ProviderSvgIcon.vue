<script setup lang="ts">
import { computed } from 'vue'

// Color versions where available, monochrome fallback for openai/anthropic/openrouter
import openaiSvg from '@lobehub/icons-static-svg/icons/openai.svg?raw'
import anthropicSvg from '@lobehub/icons-static-svg/icons/anthropic.svg?raw'
import geminiSvg from '@lobehub/icons-static-svg/icons/gemini-color.svg?raw'
import azureaiSvg from '@lobehub/icons-static-svg/icons/azureai-color.svg?raw'
import awsSvg from '@lobehub/icons-static-svg/icons/aws-color.svg?raw'
import deepseekSvg from '@lobehub/icons-static-svg/icons/deepseek-color.svg?raw'
import qwenSvg from '@lobehub/icons-static-svg/icons/qwen-color.svg?raw'
import zhipuSvg from '@lobehub/icons-static-svg/icons/zhipu-color.svg?raw'
import kimiSvg from '@lobehub/icons-static-svg/icons/kimi-color.svg?raw'
import wenxinSvg from '@lobehub/icons-static-svg/icons/wenxin-color.svg?raw'
import sparkSvg from '@lobehub/icons-static-svg/icons/spark-color.svg?raw'
import yiSvg from '@lobehub/icons-static-svg/icons/yi-color.svg?raw'
import minimaxSvg from '@lobehub/icons-static-svg/icons/minimax-color.svg?raw'
import baichuanSvg from '@lobehub/icons-static-svg/icons/baichuan-color.svg?raw'
import cohereSvg from '@lobehub/icons-static-svg/icons/cohere-color.svg?raw'
import mistralSvg from '@lobehub/icons-static-svg/icons/mistral-color.svg?raw'
import openrouterSvg from '@lobehub/icons-static-svg/icons/openrouter.svg?raw'
import difySvg from '@lobehub/icons-static-svg/icons/dify-color.svg?raw'

const props = defineProps<{
  presetId: string
  size?: number
}>()

const svgMap: Record<string, string> = {
  openai: openaiSvg,
  anthropic: anthropicSvg,
  google_gemini: geminiSvg,
  google: geminiSvg,
  azure_openai: azureaiSvg,
  aws_bedrock: awsSvg,
  deepseek: deepseekSvg,
  qwen: qwenSvg,
  zhipu: zhipuSvg,
  moonshot: kimiSvg,
  moonshot_kimi: kimiSvg,
  wenxin: wenxinSvg,
  spark: sparkSvg,
  yi: yiSvg,
  minimax: minimaxSvg,
  baichuan: baichuanSvg,
  cohere: cohereSvg,
  mistral: mistralSvg,
  openrouter: openrouterSvg,
  dify: difySvg,
}

// Security: SVG content is from static @lobehub/icons-static-svg imports only (not user input)
const svgContent = computed(() => {
  const raw = svgMap[props.presetId]
  // Strip inline width/height so CSS can control sizing
  return raw?.replace(/\s(width|height)="[^"]*"/g, '')
})
const s = computed(() => props.size ?? 24)
</script>

<template>
  <div
    v-if="svgContent"
    class="svg-icon"
    v-html="svgContent"
    :style="{ width: s + 'px', height: s + 'px' }"
  />
</template>

<style scoped>
.svg-icon :deep(svg) {
  width: 100%;
  height: 100%;
}
</style>
