import { describe, it, expect } from 'vitest'
import { formatMaskedKey, formatDate, truncateText } from '@/utils/formatters'
import { validateKeyName, validateKey, validateProviderId } from '@/utils/validators'

describe('formatters', () => {
  describe('formatMaskedKey', () => {
    it('masks a long key correctly', () => {
      const result = formatMaskedKey('sk-abcdefghijklmnop')
      expect(result).toBe('sk-a••••••••mnop')
    })

    it('returns short key as-is', () => {
      const result = formatMaskedKey('sk-ab')
      expect(result).toBe('sk-ab')
    })

    it('returns empty string for empty input', () => {
      expect(formatMaskedKey('')).toBe('')
    })
  })

  describe('truncateText', () => {
    it('truncates long text', () => {
      const result = truncateText('a'.repeat(100), 20)
      expect(result.length).toBe(21) // 20 chars + ellipsis
      expect(result).toMatch(/\…$/)
    })

    it('returns short text as-is', () => {
      expect(truncateText('hello', 50)).toBe('hello')
    })

    it('returns empty for empty input', () => {
      expect(truncateText('')).toBe('')
    })
  })

  describe('formatDate', () => {
    it('returns dash for null', () => {
      expect(formatDate(null)).toBe('—')
    })

    it('returns dash for undefined', () => {
      expect(formatDate(undefined)).toBe('—')
    })

    it('formats a valid date string', () => {
      const result = formatDate('2026-01-15T10:30:00')
      expect(result).toBeTruthy()
      expect(result).not.toBe('—')
    })
  })
})

describe('validators', () => {
  describe('validateKeyName', () => {
    it('rejects empty name', () => {
      const result = validateKeyName('')
      expect(result.valid).toBe(false)
      expect(result.message).toBeTruthy()
    })

    it('rejects whitespace-only name', () => {
      expect(validateKeyName('   ').valid).toBe(false)
    })

    it('accepts valid name', () => {
      expect(validateKeyName('My Key').valid).toBe(true)
    })

    it('rejects name over 100 chars', () => {
      expect(validateKeyName('a'.repeat(101)).valid).toBe(false)
    })

    it('accepts name at exactly 100 chars', () => {
      expect(validateKeyName('a'.repeat(100)).valid).toBe(true)
    })
  })

  describe('validateKey', () => {
    it('rejects empty key', () => {
      expect(validateKey('').valid).toBe(false)
    })

    it('rejects short key', () => {
      expect(validateKey('sk-abc').valid).toBe(false)
    })

    it('accepts valid key', () => {
      expect(validateKey('sk-abcdefghijklmnop').valid).toBe(true)
    })
  })

  describe('validateProviderId', () => {
    it('rejects null', () => {
      expect(validateProviderId(null).valid).toBe(false)
    })

    it('rejects 0', () => {
      expect(validateProviderId(0).valid).toBe(false)
    })

    it('accepts valid id', () => {
      expect(validateProviderId(1).valid).toBe(true)
    })
  })
})