/**
 * Maximum value for u32 (unsigned 32-bit integer)
 */
const U32_MAX = 0xffffffff

/**
 * Generate a u32-safe ID (fits within 0 to 4,294,967,295)
 * Uses the lower 32 bits of Date.now() to ensure compatibility with Rust's u32 type
 */
export function generateU32Id(): number {
  // Use bitwise AND with 0xFFFFFFFF to get lower 32 bits
  // This ensures the ID fits in u32 range (0 to 4,294,967,295)
  return Date.now() & U32_MAX
}

/**
 * Clamp an ID to u32 range to ensure compatibility with Rust's u32 type
 * This is useful when loading existing IDs from storage that might be too large
 */
export function clampToU32(id: number): number {
  if (id < 0) return 0
  if (id > U32_MAX) return id & U32_MAX
  return id
}

