import { ref, Ref } from 'vue'
import { defineStore } from 'pinia'
import { ConnectionConfig } from '../types/vapor.type'
import { integer } from 'vscode-languageserver'

const STORAGE_KEY = 'vapor-connections'
const TYPE = 'vapor'

const normalize = (connection: any): ConnectionConfig => ({
  id: connection.id ?? 0,
  type: TYPE,
  client_path: connection.client_path ?? null,
  environment: connection.environment ?? null,
  environments: connection.environments ?? [],
})

export const useVaporStore = defineStore(TYPE, () => {
  const loadFromStorage = (): ConnectionConfig[] => {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return []
    try {
      return JSON.parse(raw).map((connection: any) => normalize(connection))
    } catch (error) {
      console.error('Error parsing Vapor connections from localStorage:', error)
      return []
    }
  }

  const connectionConfigs: Ref<ConnectionConfig[]> = ref(loadFromStorage())

  const getConnectionConfig = (id: integer | undefined | null): ConnectionConfig | undefined => {
    return connectionConfigs.value.find(c => c.id === id)
  }

  const setClientPath = (id: integer, path: string | null) => {
    const config = connectionConfigs.value.find(c => c.id === id)
    if (config) {
      config.client_path = path
      localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
      return
    }

    connectionConfigs.value.push({ id, type: TYPE, client_path: path, environment: null, environments: [] })
    localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
  }

  const setEnviroments = (id: integer, environments: string[]) => {
    const config = connectionConfigs.value.find(c => c.id === id)
    if (config) {
      config.environments = environments
      localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
      return
    }

    connectionConfigs.value.push({ id, type: TYPE, client_path: null, environment: null, environments })
    localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
  }

  const setEnvironment = (id: integer, environment: string | null) => {
    const config = connectionConfigs.value.find(c => c.id === id)
    if (config) {
      config.environment = environment
      localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
      return
    }
    connectionConfigs.value.push({ id, type: TYPE, client_path: null, environment, environments: [] })
    localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
  }

  const removeEnvironment = (id: integer) => {
    const config = connectionConfigs.value.find(c => c.id === id)
    if (config) {
      config.environment = null
      localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
      return
    }

    console.warn(`Vapor config with id ${id} not found for environment removal.`)
  }

  const resetVaporConfig = (id: integer) => {
    const index = connectionConfigs.value.findIndex(c => c.id === id)
    if (index !== -1) {
      connectionConfigs.value.splice(index, 1)
      localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
      return
    }

    connectionConfigs.value.push({ id, type: 'vapor', client_path: null, environment: null, environments: [] })
    localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
  }

  const removeVaporConfig = (id: integer) => {
    const index = connectionConfigs.value.findIndex(c => c.id === id)
    if (index !== -1) {
      connectionConfigs.value.splice(index, 1)
      localStorage.setItem(STORAGE_KEY, JSON.stringify(connectionConfigs.value))
      return
    }
    console.warn(`Vapor config with id ${id} not found for removal.`)
  }

  return {
    getConnectionConfig,
    setClientPath,
    setEnvironment,
    removeEnvironment,
    setEnviroments,
    resetVaporConfig,
    removeVaporConfig,
  }
})
