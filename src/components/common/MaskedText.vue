<script setup lang="ts">
import { ref, computed } from 'vue'

const props = defineProps<{
  text: string
  masked?: boolean
}>()

const revealed = ref(false)

const displayText = computed(() => {
  if (revealed.value || !props.masked) return props.text
  return props.text.replace(/./g, '\u2022').slice(0, 32)
})

function toggle() {
  revealed.value = !revealed.value
}
</script>

<template>
  <span
    class="masked-text"
    :class="{ revealed }"
    @mouseenter="revealed = true"
    @mouseleave="revealed = false"
    @click="toggle"
  >
    <code>{{ displayText }}</code>
  </span>
</template>

<style scoped>
.masked-text {
  cursor: pointer;
  user-select: none;
  font-family: 'SF Mono', 'Fira Code', 'Consolas', monospace;
  font-size: var(--text-sm);
  color: var(--text-secondary);
  transition: color var(--transition-fast);
}

.masked-text:hover,
.masked-text.revealed {
  color: var(--text-primary);
}

.masked-text code {
  background: none;
  padding: 0;
}
</style>
