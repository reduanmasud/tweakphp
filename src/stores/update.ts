import { ref } from 'vue'
import { defineStore } from 'pinia'
// TODO(Step 3): Replace with Tauri update types
import { UpdateInfo } from '../types/update.type'
import semver from 'semver'

export const useUpdateStore = defineStore('update', () => {
  const update = ref<UpdateInfo>()
  const checking = ref(true)

  let storedUpdate = localStorage.getItem('update')
  if (storedUpdate) {
    update.value = JSON.parse(storedUpdate)
  }

  const setUpdate = (info: UpdateInfo): void => {
    checking.value = false
    update.value = info
    localStorage.setItem('update', JSON.stringify(info))
  }

  const setChecking = (value: boolean): void => {
    checking.value = value
  }

  const isUpdateAvailable = (currentVersion: string, newVersion?: string) => {
    if (newVersion) {
      return semver.gt(newVersion, currentVersion)
    }

    return false
  }

  return { update, setUpdate, checking, setChecking, isUpdateAvailable }
})
