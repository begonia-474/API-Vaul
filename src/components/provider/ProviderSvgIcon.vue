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

// Local provider icons (from cc-switch)
import metaSvg from '@/icons/provider/meta.svg?raw'
import perplexitySvg from '@/icons/provider/perplexity.svg?raw'
import xaiSvg from '@/icons/provider/xai.svg?raw'
import huggingfaceSvg from '@/icons/provider/huggingface.svg?raw'
import siliconflowSvg from '@/icons/provider/siliconflow.svg?raw'
import bailianSvg from '@/icons/provider/bailian.svg?raw'
import volcengineSvg from '@/icons/provider/volcengine.svg?raw'
import githubSvg from '@/icons/provider/github.svg?raw'
import nvidiaSvg from '@/icons/provider/nvidia.svg?raw'
import xiaomimimoSvg from '@/icons/provider/xiaomimimo.svg?raw'
import tencentSvg from '@/icons/provider/tencent.svg?raw'

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
  // New icons
  meta: metaSvg,
  perplexity: perplexitySvg,
  xai: xaiSvg,
  huggingface: huggingfaceSvg,
  siliconflow: siliconflowSvg,
  bailian: bailianSvg,
  volcengine_ark: volcengineSvg,
  github: githubSvg,
  nvidia: nvidiaSvg,
  xiaomimimo: xiaomimimoSvg,
  tencent_hunyuan: tencentSvg,
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
