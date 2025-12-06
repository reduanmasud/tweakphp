import fetch from 'node-fetch'
import { getSettings } from '../settings.ts'
import { Tab } from '../../types/tab.type.ts'
import {
  DEFAULT_PROMPT_COMPLETE_CODE,
  DEFAULT_PROMPT_COMPLETE_COMMENT,
  DEFAULT_PROMPT_GENERATE_CODE_FROM_COMMENT,
} from '../../types/ai/prompts.ts'

interface CompletionContext {
  language: string
  textBeforeCursor: string
  textAfterCursor: string
  cursorPosition: {
    lineNumber: number
    column: number
  }
}

interface AiCompletionMetadata {
  completionMetadata: CompletionContext
}

export class AiCompletion {
  public async getCompletions(
    completionMetadata: AiCompletionMetadata,
    tab: Tab
  ): Promise<{
    completion: string | null
    error: string | null
  }> {
    const settings = getSettings()
    const context = completionMetadata.completionMetadata
    if (!settings.aiApiKey) {
      return {
        completion: null,
        error: 'API key is not set.',
      }
    }

    const prompt = this.buildPrompt(context, tab)

    console.log('Generated Prompt:', prompt)

    try {
      switch (settings.aiProvider) {
        case 'openrouter':
          if (!settings.aiModelId) {
            return {
              completion: null,
              error: 'AI model ID is not set for OpenRouter.',
            }
          }
          try {
            const response = await this.fetchOpenRouter(settings.aiApiKey, settings.aiModelId, prompt)
            return {
              completion: response,
              error: null,
            }
          } catch (error) {
            console.log('Error fetching OpenRouter completions:', error)
            return {
              completion: null,
              error: (error as Error).message,
            }
          }
        default:
          return {
            completion: null,
            error: 'Unsupported AI provider.',
          }
      }
    } catch (error) {
      console.log('Error fetching AI completions:', error)
      return {
        completion: null,
        error: (error as Error).message,
      }
    }
  }

  /**
   * Determines if the user intends to generate code from a comment.
   * This is true if the line immediately preceding the cursor is a comment
   * and the current line is empty.
   * @param context The current editor context.
   */
  private isCommentToCodeScenario(context: CompletionContext): boolean {
    const lines = context.textBeforeCursor.split('\n')

    const currentLine = lines[lines.length - 1]
    if (currentLine.trim() !== '') {
      return false
    }

    for (let i = lines.length - 2; i >= 0; i--) {
      const lastMeaningfulLine = lines[i].trim()

      if (lastMeaningfulLine !== '') {
        const isComment = lastMeaningfulLine.startsWith('//') || lastMeaningfulLine.startsWith('#')

        if (!isComment) {
          return false
        }

        let commentContent = ''
        if (lastMeaningfulLine.startsWith('//')) {
          commentContent = lastMeaningfulLine.substring(2)
        } else {
          commentContent = lastMeaningfulLine.substring(1)
        }

        return commentContent.trim() !== ''
      }
    }

    return false
  }

  /**
   * Determines if the cursor is currently inside a comment block.
   * @param context The current editor context.
   * @returns `true` if the cursor is inside a comment, otherwise `false`.
   */
  private isInsideComment(context: CompletionContext): boolean {
    const textBeforeCursor = context.textBeforeCursor

    const lastMultiLineStart = textBeforeCursor.lastIndexOf('/*')
    const lastMultiLineEnd = textBeforeCursor.lastIndexOf('*/')

    if (lastMultiLineStart > lastMultiLineEnd) {
      return true
    }

    const lines = textBeforeCursor.split('\n')
    const currentLineBeforeCursor = lines[lines.length - 1] || ''
    const singleLineCommentPos = Math.max(currentLineBeforeCursor.indexOf('//'), currentLineBeforeCursor.indexOf('#'))

    return singleLineCommentPos !== -1
  }

  /**
   * Builds an intelligent prompt based on the code context.
   * It recognizes whether the user is writing a comment, writing code,
   * or wants to generate code from a comment.
   */
  private buildPrompt(context: CompletionContext, tab: Tab): string {
    const settings = getSettings()
    const fullCodeWithCursor = `${context.textBeforeCursor}<cursor>${context.textAfterCursor}`

    const framework = tab.info.name?.toLowerCase() || 'plain php'
    const frameworkVersion = tab.info.version ? ` v${tab.info.version}` : 'an unknown version of framework'
    const phpVersion = tab.info.php_version ? `PHP v${tab.info.php_version}` : 'an unknown PHP version'

    let frameworkGuidelines = `Follow best practices and conventions for ${framework}${frameworkVersion} and ${phpVersion} development.\n`
    if (framework === 'laravel') {
      frameworkGuidelines += `Leverage Laravel's core features, such as its helper functions, Facades, and Collection classes, to write idiomatic and efficient code. Prioritize Eloquent for database interactions and Blade for templating.\n`
    }
    if (framework === 'plain php') {
      frameworkGuidelines += `Focus on writing secure, efficient, and maintainable PHP code. Adhere to PSR standards and best practices for modern PHP development.`
    }

    let promptTemplate = ''

    // Scenario 1: Generate code from a comment
    if (this.isCommentToCodeScenario(context)) {
      const lines = context.textBeforeCursor.split('\n')
      const commentLine = lines[lines.length - 2].trim()

      // Use settings template if available, otherwise default
      promptTemplate = settings.aiPromptTemplateGenerateCodeFromComment || DEFAULT_PROMPT_GENERATE_CODE_FROM_COMMENT

      promptTemplate = promptTemplate.replace('{{commentLine}}', commentLine)
    }
    // Scenario 2: Complete a comment
    else if (this.isInsideComment(context)) {
      promptTemplate = settings.aiPromptTemplateCompleteComment || DEFAULT_PROMPT_COMPLETE_COMMENT
    }
    // Scenario 3: Complete code (default)
    else {
      promptTemplate = settings.aiPromptTemplateCompleteCode || DEFAULT_PROMPT_COMPLETE_CODE
    }

    // Replace common placeholders
    return promptTemplate
      .replace('{{frameworkGuidelines}}', frameworkGuidelines)
      .replace('{{fullCodeWithCursor}}', fullCodeWithCursor)
  }

  private async fetchOpenRouter(apiKey: string, model: string, prompt: string): Promise<string> {
    try {
      const response = await fetch('https://openrouter.ai/api/v1/chat/completions', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${apiKey}`,
          'Content-Type': 'application/json',
          'HTTP-Referer': 'https://tweakphp.com',
          'X-Title': 'TweakPHP',
        },
        body: JSON.stringify({
          model: model,
          messages: [
            {
              role: 'user',
              content: prompt,
            },
          ],
          max_tokens: 1024,
        }),
      })

      if (response.status !== 200) {
        const errorBody = await response.text()
        throw new Error(`OpenRouter API error: ${response.status} - ${response.statusText}. Response: ${errorBody}`)
      }

      const jsonResponse = await response.json()
      return jsonResponse?.choices?.[0]?.message?.content
    } catch (error) {
      throw error
    }
  }
}
