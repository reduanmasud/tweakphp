import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useExecuteStore = defineStore('execute', () => {
  const executing = ref(false)
  function setExecuting(value: boolean) {
    executing.value = value
  }

  return { executing, setExecuting }
})
