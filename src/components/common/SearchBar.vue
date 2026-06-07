<script setup lang="ts">
import { ref, watch } from 'vue'
import { NInput, NIcon } from 'naive-ui'
import { SearchOutline } from '@vicons/ionicons5'

const props = defineProps<{
  value?: string
  placeholder?: string
}>()

const emit = defineEmits<{
  search: [query: string]
}>()

const localValue = ref(props.value ?? '')

watch(() => props.value, (v) => {
  if (v !== undefined) localValue.value = v
})

let debounceTimer: ReturnType<typeof setTimeout> | null = null

function handleInput(val: string) {
  localValue.value = val
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    emit('search', val)
  }, 250)
}
</script>

<template>
  <div class="search-bar">
    <n-input
      :value="localValue"
      :placeholder="placeholder ?? '搜索...'"
      clearable
      @update:value="handleInput"
    >
      <template #prefix>
        <n-icon :component="SearchOutline" />
      </template>
    </n-input>
  </div>
</template>

<style scoped>
.search-bar {
  width: 100%;
}

.search-bar :deep(.n-input--focus) {
  box-shadow: var(--shadow-glow);
}
</style>
