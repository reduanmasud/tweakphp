import { ref, computed, readonly } from 'vue'
import type { OpenRouterModel } from '../../types/ai/openrouter.type'

/**
 * Use for fetching and managing OpenRouter models.
 */
export function useOpenRouter() {
  const models = ref<OpenRouterModel[]>([])
  const loading = ref<boolean>(false)
  const error = ref<string | null>(null)

  const fetchModels = async () => {
    loading.value = true
    error.value = null
    try {
      const response = await fetch('https://openrouter.ai/api/v1/models')

      if (!response.ok) {
        throw new Error(`Errore HTTP: ${response.status} - ${response.statusText}`)
      }

      const jsonResponse = await response.json()
      models.value = jsonResponse.data
    } catch (e: any) {
      console.error('Its impossible to fetch models from OpenRouter: ', e)
      error.value = e.message || 'Its impossible to fetch models from OpenRouter'
      models.value = []
    } finally {
      loading.value = false
    }
  }

  const freeModels = computed(() =>
    models.value
      .filter(model => model.pricing.prompt === '0' && model.pricing.completion === '0')
      .sort((a, b) => a.name.localeCompare(b.name))
  )

  const formattedModels = computed(() =>
    models.value
      .map(m => ({
        ...m,
        label: `${m.name} (Prompt: $${m.pricing.prompt}/1K, Completion: $${m.pricing.completion}/1K)`,
      }))
      .sort((a, b) => a.name.localeCompare(b.name))
  )

  return {
    models: formattedModels,
    loading: readonly(loading),
    error: readonly(error),
    fetchModels,
    freeModels,
  }
}
