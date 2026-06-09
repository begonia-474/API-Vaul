/**
 * Mask an API key, showing only the first and last few characters.
 * e.g. sk-abcde...fghij -> sk-ab...ghij
 */
export function formatMaskedKey(key: string, visibleStart = 4, visibleEnd = 4): string {
  if (!key) return ''
  if (key.length <= visibleStart + visibleEnd + 3) return key

  const start = key.slice(0, visibleStart)
  const end = key.slice(-visibleEnd)
  return `${start}${'•'.repeat(8)}${end}`
}

/**
 * Format a date string to a human-readable local format.
 */
export function formatDate(dateStr: string | null | undefined): string {
  if (!dateStr) return '—'
  try {
    const date = new Date(dateStr + 'Z')
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return dateStr
  }
}

/**
 * Truncate text to a maximum length with ellipsis.
 */
export function truncateText(text: string, maxLength = 50): string {
  if (!text) return ''
  if (text.length <= maxLength) return text
  return text.slice(0, maxLength) + '…'
}

/**
 * Format relative time (e.g. "3 分钟前").
 */
export function formatRelativeTime(dateStr: string | null | undefined): string {
  if (!dateStr) return '—'
  try {
    const now = Date.now()
    const then = new Date(dateStr + 'Z').getTime()
    const diffMs = now - then
    const diffMin = Math.floor(diffMs / 60_000)

    if (diffMin < 1) return '刚刚'
    if (diffMin < 60) return `${diffMin} 分钟前`

    const diffHour = Math.floor(diffMin / 60)
    if (diffHour < 24) return `${diffHour} 小时前`

    const diffDay = Math.floor(diffHour / 24)
    if (diffDay < 30) return `${diffDay} 天前`

    return formatDate(dateStr)
  } catch {
    return dateStr
  }
}
