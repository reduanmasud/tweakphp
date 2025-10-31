<template>
  <Switch
    :style="{
      backgroundColor: settingsStore.colors.backgroundDark,
      color: settingsStore.colors.foreground,
      outlineColor: settingsStore.colors.border,
    }"
    v-model="enabled"
    :class="[
      enabled ? 'bg-primary-600' : 'bg-gray-200',
      'relative inline-flex items-center h-6 w-11 shrink-0 cursor-pointer rounded-full border border-transparent transition-colors duration-200 ease-in-out focus:ring-2 focus:ring-primary-600',
    ]"
  >
    <span
      :class="[
        enabled ? 'translate-x-5' : 'translate-x-0',
        'pointer-events-none relative inline-block size-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out',
      ]"
    >
      <span
        :class="[
          enabled ? 'opacity-0 duration-100 ease-out' : 'opacity-100 duration-200 ease-in',
          'absolute inset-0 flex size-full items-center justify-center transition-opacity',
        ]"
        aria-hidden="true"
      >
        <svg class="size-3 text-primary-600" fill="none" viewBox="0 0 12 12">
          <path
            d="M4 8l2-2m0 0l2-2M6 6L4 4m2 2l2 2"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </span>
      <span
        :class="[
          enabled ? 'opacity-100 duration-200 ease-in' : 'opacity-0 duration-100 ease-out',
          'absolute inset-0 flex size-full items-center justify-center transition-opacity',
        ]"
        aria-hidden="true"
      >
        <svg class="size-3 text-primary-600" fill="currentColor" viewBox="0 0 12 12">
          <path
            d="M3.707 5.293a1 1 0 00-1.414 1.414l1.414-1.414zM5 8l-.707.707a1 1 0 001.414 0L5 8zm4.707-3.293a1 1 0 00-1.414-1.414l1.414 1.414zm-7.414 2l2 2 1.414-1.414-2-2-1.414 1.414zm3.414 2l4-4-1.414-1.414-4 4 1.414 1.414z"
          />
        </svg>
      </span>
    </span>
  </Switch>
</template>

<script setup lang="ts">
  import { ref, watch } from 'vue'
  import { useSettingsStore } from '../stores/settings'
  import { Switch } from '@headlessui/vue'

  const props = defineProps({
    modelValue: Boolean,
  })

  const emit = defineEmits(['update:modelValue'])

  const settingsStore = useSettingsStore()

  const enabled = ref(props.modelValue)

  watch(
    () => props.modelValue,
    newValue => {
      enabled.value = newValue
    }
  )

  watch(enabled, newValue => {
    emit('update:modelValue', newValue)
  })
</script>
<style scoped></style>
