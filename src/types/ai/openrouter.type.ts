export interface OpenRouterModel {
  id: string
  name: string
  description: string
  pricing: {
    prompt: string
    completion: string
    request: string
    image: string
  }
  context_length: number
  architecture: {
    modality: string
    tokenizer: string
    instruct_type: string | null
  }
  top_provider: {
    max_completion_tokens: number | null
    is_moderated: boolean
  }
  per_request_limits: {
    prompt_tokens: string
    completion_tokens: string
  } | null
}
